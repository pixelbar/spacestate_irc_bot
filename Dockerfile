FROM ubuntu:18.04

RUN apt update && \
    apt upgrade -y && \
    apt install libssl-dev -y

COPY target/release/spacestate_irc_bot /spacestate_irc_bot

ENTRYPOINT ["/spacestate_irc_bot"]
