use std::{any::Any, env, string};

use super::{Command, CommandExecutionError, CommandInfo};
use crate::event_handler::BotEvents;
use async_trait::async_trait;
use error_stack::{IntoReport, Report, Result, ResultExt};
use serde::{Deserialize, Serialize};
use serenity::all::CommandDataOptionValue::SubCommand;
use serenity::all::{
    CommandDataOption, CommandDataOptionValue, CommandInteraction, CommandOption,
    CommandOptionType, Context, CreateCommand, CreateCommandOption, CreateInteractionResponse,
    CreateInteractionResponseMessage, Permissions, UserId,
};
use tokio::sync::broadcast::error;

#[derive(Debug)]
pub struct UserGetCommand;

impl CommandInfo for UserGetCommand {
    fn name(&self) -> String {
        String::from("user")
    }

    fn description(&self) -> String {
        String::from("Gets a user")
    }
}

#[derive(Debug, Deserialize)]
struct User {
    id: Option<String>,
    steamId: Option<u64>,
    gmodstoreId: Option<String>,
    discordId: Option<u64>,
    error: Option<String>,
}

#[async_trait]
impl Command for UserGetCommand {
    async fn execute<'a>(
        &self,
        handler: &BotEvents,
        ctx: &Context,
        interaction: &'a mut CommandInteraction,
    ) -> Result<(), CommandExecutionError> {
        let sub_cmd = match interaction.data.options.get(0) {
            Some(target_command_data) => target_command_data,
            None => {
                return Err(Report::from(CommandExecutionError)
                    .attach_printable("Failed to get sub command arg data"))
            }
        };

        let command_type = sub_cmd.name.as_str();

        let sub_command_data = match &sub_cmd.value {
            SubCommand(options) => match options.get(0) {
                Some(command_data_option) => command_data_option.clone(),
                None => {
                    return Err(Report::from(CommandExecutionError)
                        .attach_printable("Failed to get sub command arg data"))
                }
            },
            _ => {
                return Err(Report::from(CommandExecutionError)
                    .attach_printable("Failed to get sub command arg data"))
            }
        };

        let id;
        match command_type {
            "discord" => {
                let target_user = match sub_command_data.value {
                    CommandDataOptionValue::User(user_id) => user_id,
                    _ => {
                        return Err(Report::from(CommandExecutionError)
                            .attach_printable("Failed to get target user arg"))
                    }
                };

                let target_user = target_user.to_user(&ctx.http).await.map_err(|e| {
                    Report::from(e)
                        .change_context(CommandExecutionError)
                        .attach_printable("Failed to fetch user")
                })?;

                id = target_user.id.to_string();
            }
            _ => {
                id = sub_command_data.value.as_str().unwrap().to_string();
            }
        }

        let url;
        match command_type {
            "pulsar-id" => {
                url = format!("{}/user/{}", handler.cfg.api_url, id);
            }
            "discord" => {
                url = format!("{}/user/{}/discord", handler.cfg.api_url, id);
            }
            "steam-id" => {
                url = format!("{}/user/{}/steam", handler.cfg.api_url, id);
            }
            "gmodstore-id" => {
                url = format!("{}/user/{}/gmodstore", handler.cfg.api_url, id);
            }
            _ => {
                return Err(Report::from(CommandExecutionError)
                    .attach_printable("Invalid sub command type"))
            }
        }

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

        let message = CreateInteractionResponseMessage::new()
            .content(format!(
            "- Pulsar ID: [{}](<https://pornhub.com>)\n- Steam ID: [{}](<https://pornhub.com>)\n- Gmodstore ID: [{}](<https://pornhub.com>)\n- Discord ID: [{}](<https://pornhub.com>)
            ", user.id.unwrap(), user.steamId.unwrap(), user.gmodstoreId.unwrap(), user.discordId.unwrap()));

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
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "pulsar-id",
                    "Get the user from their PulsarID",
                )
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "id",
                        "The users Pulsar ID.",
                    )
                    .required(true),
                ),
            )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "discord",
                    "Get the user from their Discord Account",
                )
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::User,
                        "id",
                        "The users Discord account.",
                    )
                    .required(true),
                ),
            )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "steam-id",
                    "Get the user from their SteamID",
                )
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "id",
                        "The users SteamID64.",
                    )
                    .required(true),
                ),
            )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "gmodstore-id",
                    "Get the user from their Gmodstore ID",
                )
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "id",
                        "The users Gmodstore ID.",
                    )
                    .required(true),
                ),
            )
            .dm_permission(false)
    }
}
