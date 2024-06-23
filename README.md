# Pulsar Link Bot
Pulsar Link Bot is a Discord Bot written in Rust using the Serenity library.
It is designed to be used with the rest of the Pulsar Link project and will not function without them.

## Features
- Ban command
- User management - Get, Create
- 
## Commands
- `/usercreate` - Creates a new user
- `/user` - Gets a user based off: Pulsar Link ID, Discord ID, SteamID, or Gmodstore ID
- `/verify` - Gives a user information on how to verify
- `/ban` - Bans a user
- `/addons` - Gets a list of a users gmodstore purchases

## Installation
This has only been tested to work on Linux. It may work on other operating systems, but it is not guaranteed - No support will be provided for other operating systems.
1. Download [Rust](https://www.rust-lang.org/)
2. Download the latest release from the [releases page](https://github.com/Pulsar-Dev/link-bot/releases/latest).
3. Create a `cargo.toml` file based on [`cargo.toml.example`](https://github.com/Pulsar-Dev/link-bot/blob/master/cargo.toml.example)
4. Run the bot with `./pulsar-link-bot`

## Building
1. Download [Rust](https://www.rust-lang.org/)
2. Clone the repository
3. Create a `cargo.toml` file based on [`cargo.toml.example`](https://github.com/Pulsar-Dev/link-bot/blob/master/cargo.toml.example)
4. Run `cargo build`

### Building a release
1. Run `cargo build --release`
2. The jar will be located in `./target/release/`