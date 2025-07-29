# Ngrok Telegram Bot

A simple telegram bot that user can query current ngrok port due to dynamic port
in free version.

## Features

- Limited to specific tg user
- deployment scripts

## Prerequisites

- rust compiler
- `ngrok.yml`: with your corresponding configuration
  ```yml
  version: "2"
  authtoken: xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
  tunnels:
      ssh:
          addr: 22
          proto: tcp
  ```
- `.env`: setup limited user id and your bot id from `botfather`
  ```env
  USER_TOKEN=0000000000
  TELOXIDE_TOKEN=0000000000:XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
  ```

## Deployment

1. First deploy ngrok

  ```sh
  ./deploy-ngrok.sh
  ```

2. Build & deploy ngrok-bot

  ```sh
  cargo build --release
  ./deploy-bot.sh
  ```

## Usage

In your telegram bot chat, type `/getport` then it will respond with the port number.

## License: MIT
