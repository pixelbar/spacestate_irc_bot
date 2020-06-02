#[macro_use]
extern crate serde_derive;

use futures::prelude::*;
use irc::client::prelude::*;
use lazy_static::lazy_static;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

#[tokio::main]
async fn main() {
    loop {
        let config = Config {
            nickname: Some(String::from("spacestate")),
            server: Some(String::from("irc.smurfnet.ch")),
            channels: vec![String::from("#pixelbar")],
            use_tls: Some(false),

            ..Default::default()
        };
        let mut client = Client::from_config(config).await.unwrap();
        if let Err(e) = client.identify() {
            eprintln!("Could not identify: {:?}", e);
        } else {
            let sender = client.sender();
            let is_running = Arc::new(AtomicBool::new(true));

            let join_handle = tokio::spawn(poll_state(sender, Arc::clone(&is_running)));

            let mut stream = client.stream().unwrap();
            loop {
                match stream.next().await {
                    Some(Ok(msg)) => {
                        if let Command::PRIVMSG(channel, message) = msg.command {
                            if message.contains("!state") || message.contains("!spacestate") {
                                let last_state = &*CURRENT_STATE.lock().unwrap();
                                let _ = client.send_privmsg(
                                    &channel,
                                    format!("Pixelbar is {:?}!", last_state),
                                );
                            }
                        }
                    }
                    Some(Err(e)) => {
                        eprintln!("IRC bot died: {:?}", e);
                        break;
                    }
                    None => {
                        eprintln!("IRC stream returned None...?");
                    }
                }
            }
            is_running.store(false, Ordering::Relaxed);
            if let Err(e) = join_handle.await {
                eprintln!("Could not join background thread: {:?}", e);
            }
        }

        eprintln!("Rebooting in 10 seconds");
        tokio::time::delay_for(std::time::Duration::from_secs(10)).await;
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum State {
    Unknown,
    Closed,
    Open,
}

lazy_static! {
    static ref CURRENT_STATE: Mutex<State> = Mutex::new(State::Unknown);
}

// Polls the spacestate and updates the CURRENT_STATE accordingly
// This also broadcasts the message to (hardcoded) channel #pixelbar
async fn poll_state(bot: Sender, is_running: Arc<AtomicBool>) {
    while is_running.load(Ordering::Relaxed) {
        match try_get_state().await {
            Ok(State::Unknown) => {
                println!("Unknown spacestate");
            }
            Ok(new_state) => {
                let mut last_state = CURRENT_STATE.lock().unwrap();
                let mut did_send_message = true;
                if *last_state != State::Unknown && *last_state != new_state {
                    if let Err(e) =
                        bot.send_privmsg("#pixelbar", format!("Pixelbar is now {:?}", new_state))
                    {
                        did_send_message = false;
                        println!("Could not broadcast state update: {:?}", e);
                    }
                }

                if did_send_message {
                    *last_state = new_state;
                }
            }
            Err(e) => {
                println!("Could not get space state: {:?}", e);
            }
        }
        tokio::time::delay_for(std::time::Duration::from_secs(60)).await;
    }
}

// Try to get the spacestate and return an enum containing the value.
// This requests https://spacestate.pixelbar.nl/spacestate.php and compares it to one of two hardcoded values for Open and Closed.
// If the returning value is different, State::Unknown is returned.
async fn try_get_state() -> Result<State, Box<dyn std::error::Error>> {
    let state: Spacestate = reqwest::get("https://spacestate.pixelbar.nl/spacestate.php")
        .await?
        .json()
        .await?;

    Ok(match state.state.as_str() {
        "open" => State::Open,
        "closed" => State::Closed,
        x => {
            println!("Unknown space state: {:?}", x);
            State::Unknown
        }
    })
}

#[derive(Deserialize)]
struct Spacestate {
    state: String,
}
