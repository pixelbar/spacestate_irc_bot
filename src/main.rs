use irc::client::prelude::*;
use lazy_static::lazy_static;
use std::sync::Mutex;

fn main() {
    let config = Config {
        nickname: Some(String::from("spacestate")),
        server: Some(String::from("irc.smurfnet.ch")),
        channels: Some(vec![String::from("#pixelbar")]),

        ..Default::default()
    };
    let client = IrcClient::from_config(config).unwrap();
    client.identify().unwrap();
    {
        let client = client.clone();
        std::thread::spawn(move || poll_state(client));
    }
    client
    .for_each_incoming(|irc_msg| {
        if let Command::PRIVMSG(channel, message) = irc_msg.command {
            if message.contains("!state") || message.contains("!spacestate") {
                let last_state = &*CURRENT_STATE.lock().unwrap();
                client
                    .send_privmsg(&channel, format!("Pixelbar is {:?}!", last_state))
                    .unwrap();
            }
        }
    })
    .unwrap();
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
fn poll_state(bot: IrcClient) {
    loop {
        match try_get_state() {
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
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}

// Try to get the spacestate and return an enum containing the value.
// This requests https://spacestate.pixelbar.nl/spacestate.php and compares it to one of two hardcoded values for Open and Closed.
// If the returning value is different, State::Unknown is returned.
fn try_get_state() -> Result<State, failure::Error> {
    let command = std::process::Command::new("curl")
        .arg("-s")
        .arg("https://spacestate.pixelbar.nl/spacestate.php")
        .output()?;

    let response = std::str::from_utf8(&command.stdout)?;

    Ok(match response {
        r#"{"state":"open"}"# => State::Open,
        r#"{"state":"closed"}"# => State::Closed,
        x => {
            println!("Unknown space state: {:?}", x);
            State::Unknown
        }
    })
}
