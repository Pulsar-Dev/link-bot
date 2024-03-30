use bot::start;
use clap::{Arg, ArgAction, Command};
use error_stack::{Context, Report, Result, ResultExt};
use std::{env, fmt};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

mod bot;
mod commands;
mod config;
mod event_handler;

#[derive(Debug)]
struct ApplicationInitialisationError;

impl fmt::Display for ApplicationInitialisationError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Failed to start application")
    }
}

impl Context for ApplicationInitialisationError {}

#[tokio::main]
async fn main() -> Result<(), ApplicationInitialisationError> {
    FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(true)
        .with_file(false)
        .with_line_number(false)
        .with_ansi(true)
        .init();

    let commands = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand(Command::new("start").about("Starts the bot"))
        .subcommand(Command::new("push").about("Pushes the latest commands to Discord."));

    match commands.get_matches().subcommand() {
        Some(("start", _)) => start()
            .await
            .change_context(ApplicationInitialisationError)?,
        Some(("push", _)) => start()
            .await
            .change_context(ApplicationInitialisationError)?,
        _ => unreachable!(),
    }

    Ok(())
}
