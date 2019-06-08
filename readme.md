# Spacestate IRC bot

[![Build Status](https://travis-ci.org/pixelbar/spacestate_irc_bot.svg?branch=master)](https://travis-ci.org/pixelbar/spacestate_irc_bot)

This bot notifies our IRC channel of changes in the spacestate.

It does this by polling [https://spacestate.pixelbar.nl/spacestate.php](https://spacestate.pixelbar.nl/spacestate.php). Non-bots can also use [https://spacestate.pixelbar.nl/](https://spacestate.pixelbar.nl/).

## Running on docker

A docker image is generated every release. Please check the [Releases](https://github.com/pixelbar/spacestate_irc_bot/releases) page. You can download the `spacestate_irc_bot.tar` image there. Afterwards, run the following commands:

```
docker load --input spacestate_irc_bot.tar
docker run -d spacestate_irc_bot
```

## Running from source

First you need to download [rustup](https://rustup.rs).

After that simply run `cargo run` in the source of this directory.

## Modifying the bot
The bot uses rust's [irc](https://docs.rs/irc) library. See that library for more config settings.
