use std::fmt;

use async_trait::async_trait;
use error_stack::Context;
use serenity::{
    builder::CreateCommand, client::Context as SerenityContext,
    model::application::CommandInteraction,
};

use crate::event_handler::BotEvents;

mod ban;
mod user_create;
mod user_get;
mod addons;
mod verify;

#[async_trait]
pub trait Command
where
    Self: CommandInfo,
{
    async fn execute<'a>(
        &self,
        handler: &BotEvents,
        ctx: &SerenityContext,
        interaction: &'a mut CommandInteraction,
    ) -> error_stack::Result<(), CommandExecutionError>;

    async fn register(
        &self,
        handler: &BotEvents,
    ) -> CreateCommand;
}

pub trait CommandInfo {
    fn name(&self) -> String;
    fn description(&self) -> String;
}

#[derive(Debug)]
pub struct CommandExecutionError;

impl fmt::Display for CommandExecutionError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Error whilst executing command")
    }
}

impl Context for CommandExecutionError {}

pub fn load_commands() -> Vec<Box<dyn Command + Send + Sync>> {
    vec![
        Box::new(ban::BanCommand),
        Box::new(user_create::UserCreateCommand),
        Box::new(user_get::UserGetCommand),
        Box::new(addons::UserAddonsCommand),
        Box::new(verify::VerifyCommand),
    ]
}
