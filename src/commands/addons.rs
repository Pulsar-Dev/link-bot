use async_trait::async_trait;
use error_stack::{IntoReport, Report, Result, ResultExt};
use serde::Deserialize;
use serenity::all::{CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption, CreateInteractionResponse, CreateInteractionResponseMessage};

use crate::event_handler::BotEvents;

use super::{Command, CommandExecutionError, CommandInfo};

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

#[derive(Debug, Deserialize)]
struct User {
    id: Option<String>,
    steam_id: Option<u64>,
    gmodstore_id: Option<String>,
    discord_id: Option<u64>,
    error: Option<String>,
}

#[async_trait]
impl Command for UserAddonsCommand {
    async fn execute<'a>(
        &self,
        handler: &BotEvents,
        ctx: &Context,
        interaction: &'a mut CommandInteraction,
    ) -> Result<(), CommandExecutionError> {
        let option = interaction.data.options.get(0);

        let mut pulsar_id: String = "unknown".to_string();

        match option.unwrap().name.as_str() {
            "id" => {
                pulsar_id = option.unwrap().value.as_str().unwrap().to_string();
            }
            "discord_user" => {
                let target_user = option.unwrap().value.as_user_id().unwrap();

                let url = format!("{}/user/{}/discord", handler.cfg.api_url, target_user);

                let client = reqwest::Client::new();
                let response = client
                    .get(url.to_owned())
                    .header("Authorization", handler.cfg.api_key.as_str())
                    .send()
                    .await
                    .expect("Failed to send request");

                let user: User = response
                    .json()
                    .await
                    .expect("Failed to deserialize response body");

                if user.error != None {
                    let message = CreateInteractionResponseMessage::new().content(format!(
                        "An error occurred while trying to get the user: {}",
                        user.error.unwrap()
                    ));

                    let builder = CreateInteractionResponse::Message(message);

                    interaction
                        .create_response(&ctx.http, builder)
                        .await
                        .map_err(|e| Report::from(e).change_context(CommandExecutionError))?;

                    return Ok(());
                }

                pulsar_id = user.id.unwrap();
            }
            _ => {
                let message = CreateInteractionResponseMessage::new().content("No data provided.".to_string());

                let builder = CreateInteractionResponse::Message(message);

                interaction
                    .create_response(&ctx.http, builder)
                    .await
                    .map_err(|e| Report::from(e).change_context(CommandExecutionError))?;
            }
        };

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

    async fn register(&self, _: &BotEvents) -> CreateCommand {
        CreateCommand::new(self.name())
            .description(self.description())
            .add_option(
                CreateCommandOption::new(CommandOptionType::String, "id", "The user's PulsarID")
                    .required(false),
            )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::User,
                    "discord_user",
                    "The users Discord account.",
                ).required(false),
            )
            .dm_permission(false)
    }
}
