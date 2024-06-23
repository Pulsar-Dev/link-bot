use async_trait::async_trait;
use error_stack::{IntoReport, Report, Result, ResultExt};
use serde::Serialize;
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateInteractionResponse, CreateInteractionResponseMessage, Permissions,
};

use crate::event_handler::BotEvents;

use super::{Command, CommandExecutionError, CommandInfo};

#[derive(Debug)]
pub struct UserCreateCommand;

impl CommandInfo for UserCreateCommand {
    fn name(&self) -> String {
        String::from("usercreate")
    }

    fn description(&self) -> String {
        String::from("Creates a new user.")
    }
}

#[derive(Serialize)]
struct CreateUserBody {
    steam_id: u64,
    gmodstore_id: String,
    discord_id: u64,
}

#[async_trait]
impl Command for UserCreateCommand {
    async fn execute<'a>(
        &self,
        handler: &BotEvents,
        ctx: &Context,
        interaction: &'a mut CommandInteraction,
    ) -> Result<(), CommandExecutionError> {
        let user = match interaction.data.options.get(0) {
            Some(target_command_data) => target_command_data,
            None => {
                return Err(Report::from(CommandExecutionError)
                    .attach_printable("Failed to get command arg data user"))
            }
        };

        let steam_id = match interaction.data.options.get(1) {
            Some(target_command_data) => target_command_data,
            None => {
                return Err(Report::from(CommandExecutionError)
                    .attach_printable("Failed to get command arg data user"))
            }
        };

        let gmodstore_id = match interaction.data.options.get(2) {
            Some(target_command_data) => target_command_data,
            None => {
                return Err(Report::from(CommandExecutionError)
                    .attach_printable("Failed to get command arg data user"))
            }
        };

        let client = reqwest::Client::new();

        let discord_id = user.value.as_user_id().unwrap().get();
        let steam_id = {
            let sid_str = steam_id.value.as_str().ok_or(Report::from(CommandExecutionError).attach_printable("Failed to extract Steam ID from value"))?;

            println!("{sid_str}");

            sid_str.parse::<u64>().map_err(|e| Report::from(e).change_context(CommandExecutionError))?
        };

        let gmodstore_id = {
            gmodstore_id.value.as_str().ok_or(Report::from(CommandExecutionError).attach_printable("Failed to extract Steam ID from value"))?
        };

        let params = CreateUserBody {
            steam_id,
            gmodstore_id: gmodstore_id.to_string(),
            discord_id,
        };

        let url = format!("{}/user", handler.cfg.api_url);
        let params = match serde_urlencoded::to_string(&params) {
            Ok(string) => string,
            Err(error) => return Err(Report::from(error).change_context(CommandExecutionError))
        };

        let url = format!("{}?{}", url, params);

        let res = client
            .post(url)
            .header("Authorization", handler.cfg.api_key.as_str())
            .send()
            .await;

        if let Ok(_res) = res {
            let message = CreateInteractionResponseMessage::new()
                .content("Successfully created user.")
                .ephemeral(true);

            let builder = CreateInteractionResponse::Message(message);

            interaction
                .create_response(&ctx.http, builder)
                .await
                .map_err(|e| Report::from(e).change_context(CommandExecutionError))?;

            return Ok(());
        } else if let Err(e) = res {
            let message = CreateInteractionResponseMessage::new()
                .content("An error occurred. Please try again later.")
                .ephemeral(true);

            let builder = CreateInteractionResponse::Message(message);

            interaction
                .create_response(&ctx.http, builder)
                .await
                .map_err(|e| Report::from(e).change_context(CommandExecutionError))?;

            println!("Error occurred while creating user: {}", e);

            return Ok(());
        }

        Ok(())
    }

    async fn register(&self, _: &BotEvents) -> CreateCommand {
        CreateCommand::new(self.name())
            .description(self.description())
            .add_option(
                CreateCommandOption::new(CommandOptionType::User, "user", "The Discord User")
                    .required(true),
            )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "steam-id",
                    "The user's SteamID64",
                )
                    .required(true),
            )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "gmodstore-id",
                    "The user's GmodstoreID",
                )
                    .required(true),
            )
            .default_member_permissions(Permissions::ADMINISTRATOR)
            .dm_permission(false)
    }
}
