use discord_game_sdk::Discord;
use tracing::info;

pub mod event_handler;

#[derive(Debug)]
pub struct DiscordConnection<'a> {
    discord: Discord<'a, event_handler::EventHandler>,
}

impl<'a> DiscordConnection<'a> {
    #[tracing::instrument]
    pub fn new() -> Result<Self, crate::error::Error> {
        Ok(Self {
            discord: Self::init_discord()?,
        })
    }

    #[tracing::instrument]
    fn init_discord() -> Result<Discord<'a, event_handler::EventHandler>, crate::error::Error> {
        let client_id: i64 = dotenv::var("CLIENT_ID")?.parse()?;

        info!("init discord");
        let mut discord = discord_game_sdk::Discord::new(client_id)?;
        *discord.event_handler_mut() = Some(event_handler::EventHandler);

        Ok(discord)
    }

    pub fn run_callbacks(&mut self) -> Result<(), crate::error::Error> {
        self.discord.run_callbacks()?;

        Ok(())
    }

    pub fn update_activity(
        &self,
        activity: &discord_game_sdk::Activity,
        callback: impl 'a
        + FnOnce(&Discord<'a, event_handler::EventHandler>, discord_game_sdk::Result<()>),
    ) {
        self.discord.update_activity(activity, callback)
    }
}
