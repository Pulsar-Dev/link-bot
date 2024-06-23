use async_trait::async_trait;
use error_stack::{Report, Result};
use serenity::{
    builder::{
        CreateCommand, CreateInteractionResponse,
        CreateInteractionResponseMessage,
    },
    client::Context,
    model::application::CommandInteraction,
};

use crate::event_handler::BotEvents;

use super::{Command, CommandExecutionError, CommandInfo};

#[derive(Debug)]
pub struct VerifyCommand;

impl CommandInfo for VerifyCommand {
    fn name(&self) -> String {
        String::from("verify")
    }

    fn description(&self) -> String {
        String::from("Get info about verifying!")
    }
}

#[async_trait]
impl Command for VerifyCommand {
    async fn execute<'a>(
        &self,
        _handler: &BotEvents,
        ctx: &Context,
        interaction: &'a mut CommandInteraction,
    ) -> Result<(), CommandExecutionError> {
        let message = CreateInteractionResponseMessage::new()
            .content("To gain access to support channels you first have to verify your Discord account.\nTo do this, please follow these steps (Also found in <#937373534651559966>)\n\n- Add Steam as a connection to your Discord account. (Found in settings, You can set it as hidden)\n- Head over to https://verify.lythium.dev/\n- Login to your Discord account\n- Ask for help in the correct support channels or create a ticket with /create");

        let builder = CreateInteractionResponse::Message(message);

        interaction
            .create_response(&ctx.http, builder)
            .await
            .map_err(|e| Report::from(e).change_context(CommandExecutionError))?;

        Ok(())
    }

    async fn register(&self, _: &BotEvents) -> CreateCommand {
        return CreateCommand::new(self.name())
            .description(self.description())
            .dm_permission(false);
    }
}
