#!/bin/bash

set -ex

docker build . -t spacestate_irc_bot:latest
docker tag spacestate_irc_bot trangar.azurecr.io/spacestate_irc_bot
docker push trangar.azurecr.io/spacestate_irc_bot
