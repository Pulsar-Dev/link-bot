use std::{env, string};

use super::{Command, CommandExecutionError, CommandInfo};
use crate::event_handler::BotEvents;
use async_trait::async_trait;
use error_stack::{IntoReport, Report, Result, ResultExt};
use serde::{Deserialize, Serialize};
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateInteractionResponse, CreateInteractionResponseMessage, Permissions, UserId,
};
use tokio::sync::broadcast::error;

#[derive(Debug)]
pub struct UserAddonsCommand;

impl CommandInfo for UserAddonsCommand {
    fn name(&self) -> String {
        String::from("addons")
    }

    fn description(&self) -> String {
        String::from("Gets a user's addons")
    }
}

#[derive(Debug, Deserialize)]
struct Addon {
    id: Option<String>,
    name: Option<String>,
    error: Option<String>,
}

#[derive(Deserialize)]
struct ErrorResponse {
    error: String,
}

#[async_trait]
impl Command for UserAddonsCommand {
    async fn execute<'a>(
        &self,
        handler: &BotEvents,
        ctx: &Context,
        interaction: &'a mut CommandInteraction,
    ) -> Result<(), CommandExecutionError> {
        let pulsar_id = match interaction.data.options.get(0) {
            Some(target_command_data) => target_command_data,
            None => {
                return Err(Report::from(CommandExecutionError)
                    .attach_printable("Failed to get command arg data for pulsar_id"))
            }
        };

        let pulsar_id = pulsar_id.value.as_str().unwrap();

        let client = reqwest::Client::new();
        let response = client
            .get(format!("{}/user/{}/addons", handler.cfg.api_url, pulsar_id))
            .header("Authorization", handler.cfg.api_key.as_str())
            .send()
            .await
            .expect("Failed to send request");

        let response_text = response.text().await.expect("Failed to get response text");

        let addons_result: Result<Vec<Addon>, _> =
            serde_json::from_str(&response_text).map_err(|err| error_stack::Report::from(err));

        let addons_list;
        match addons_result {
            Ok(addons) => {
                addons_list = addons;
            }
            Err(_) => {
                let error_response: ErrorResponse = serde_json::from_str(&response_text)
                    .expect("Failed to deserialize error response");

                let message = CreateInteractionResponseMessage::new().content(format!(
                    "An error occurred while trying to get the user's addons: {}",
                    error_response.error.to_string()
                ));

                let builder = CreateInteractionResponse::Message(message);

                interaction
                    .create_response(&ctx.http, builder)
                    .await
                    .map_err(|e| Report::from(e).change_context(CommandExecutionError))?;

                return Ok(());
            }
        }

        let mut addons_string = String::new();
        addons_string.push_str("User's owned addons:\n");

        for addon in &addons_list {
            let id = addon.id.as_deref().unwrap_or("");
            let name = addon.name.as_deref().unwrap_or("");
            addons_string.push_str(&format!("[{}](<https://www.gmodstore.com/market/view/{}>)\n", name, id));
        }

        let message = CreateInteractionResponseMessage::new().content(addons_string);

        let builder = CreateInteractionResponse::Message(message);

        interaction
            .create_response(&ctx.http, builder)
            .await
            .map_err(|e| Report::from(e).change_context(CommandExecutionError))?;

        return Ok(());
    }

    fn register(&self) -> CreateCommand {
        CreateCommand::new(self.name())
            .description(self.description())
            .add_option(
                CreateCommandOption::new(CommandOptionType::String, "id", "The user's PulsarID")
                    .required(true),
            )
            .dm_permission(false)
    }
}
