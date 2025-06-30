use tracing::info;

#[derive(Debug, Clone, Copy, Default)]
pub struct EventHandler;

impl discord_game_sdk::EventHandler for EventHandler {
    #[tracing::instrument]
    fn on_user_achievement_update(
        &mut self,
        discord: &discord_game_sdk::Discord<'_, Self>,
        user_achievement: &discord_game_sdk::UserAchievement,
    ) {
        let _ = discord;
        info!("user achievement update: {user_achievement:#?}");
    }

    #[tracing::instrument]
    fn on_activity_join(&mut self, discord: &discord_game_sdk::Discord<'_, Self>, secret: &str) {
        let _ = discord;
        info!("activity join: {secret}");
    }

    #[tracing::instrument]
    fn on_activity_spectate(
        &mut self,
        discord: &discord_game_sdk::Discord<'_, Self>,
        secret: &str,
    ) {
        let _ = discord;
        info!("activity spectate: {secret}");
    }

    #[tracing::instrument]
    fn on_activity_join_request(
        &mut self,
        discord: &discord_game_sdk::Discord<'_, Self>,
        user: &discord_game_sdk::User,
    ) {
        let _ = discord;
        info!("activity join request: {user:#?}");
    }

    #[tracing::instrument]
    fn on_activity_invite(
        &mut self,
        discord: &discord_game_sdk::Discord<'_, Self>,
        kind: discord_game_sdk::Action,
        user: &discord_game_sdk::User,
        activity: &discord_game_sdk::Activity,
    ) {
        let _ = discord;
        info!("activity invite: {kind:#?}; {user:#?}; {activity:#?}");
    }

    #[tracing::instrument]
    fn on_lobby_update(
        &mut self,
        discord: &discord_game_sdk::Discord<'_, Self>,
        lobby_id: discord_game_sdk::LobbyID,
    ) {
        let _ = discord;
        info!("lobby update: {lobby_id:#?}");
    }

    #[tracing::instrument]
    fn on_lobby_delete(
        &mut self,
        discord: &discord_game_sdk::Discord<'_, Self>,
        lobby_id: discord_game_sdk::LobbyID,
        reason: u32,
    ) {
        let _ = discord;
        info!("lobby delete: {lobby_id:#?}; reason: {reason}");
    }

    #[tracing::instrument]
    fn on_member_connect(
        &mut self,
        discord: &discord_game_sdk::Discord<'_, Self>,
        lobby_id: discord_game_sdk::LobbyID,
        member_id: discord_game_sdk::UserID,
    ) {
        let _ = discord;
        info!("member connect: {lobby_id:#?}; {member_id:#?}");
    }

    #[tracing::instrument]
    fn on_member_update(
        &mut self,
        discord: &discord_game_sdk::Discord<'_, Self>,
        lobby_id: discord_game_sdk::LobbyID,
        member_id: discord_game_sdk::UserID,
    ) {
        let _ = discord;
        info!("member update: {lobby_id:#?}; {member_id:#?}");
    }

    #[tracing::instrument]
    fn on_member_disconnect(
        &mut self,
        discord: &discord_game_sdk::Discord<'_, Self>,
        lobby_id: discord_game_sdk::LobbyID,
        member_id: discord_game_sdk::UserID,
    ) {
        let _ = discord;
        info!("member disconnect: {lobby_id:#?}; {member_id:#?}");
    }

    fn on_lobby_message(
        &mut self,
        discord: &discord_game_sdk::Discord<'_, Self>,
        lobby_id: discord_game_sdk::LobbyID,
        member_id: discord_game_sdk::UserID,
        data: &[u8],
    ) {
        let vec = Vec::from(data);
        let string = String::from_utf8(vec).unwrap();
        let _ = discord;
        info!("lobby message: {lobby_id:#?}; {member_id:#?}; message {string}");
    }

    fn on_speaking(
        &mut self,
        discord: &discord_game_sdk::Discord<'_, Self>,
        lobby_id: discord_game_sdk::LobbyID,
        member_id: discord_game_sdk::UserID,
        speaking: bool,
    ) {
        let _ = discord;
        info!("speaking: {lobby_id:#?}; {member_id:#?}; speaking: {speaking}");
    }

    fn on_lobby_network_message(
        &mut self,
        discord: &discord_game_sdk::Discord<'_, Self>,
        lobby_id: discord_game_sdk::LobbyID,
        member_id: discord_game_sdk::UserID,
        channel_id: discord_game_sdk::NetworkChannelID,
        data: &[u8],
    ) {
        let vec = Vec::from(data);
        let string = String::from_utf8(vec).unwrap();
        let _ = discord;
        info!(
            "lobby network message: {lobby_id:#?}; {member_id:#?}; {channel_id:#?}; message: {string}"
        );
    }

    fn on_network_message(
        &mut self,
        discord: &discord_game_sdk::Discord<'_, Self>,
        peer_id: discord_game_sdk::NetworkPeerID,
        channel_id: discord_game_sdk::NetworkChannelID,
        data: &[u8],
    ) {
        let vec = Vec::from(data);
        let string = String::from_utf8(vec).unwrap();
        let _ = discord;
        info!("network message: {peer_id:#?}; {channel_id:#?}; message: {string}");
    }

    fn on_network_route_update(
        &mut self,
        discord: &discord_game_sdk::Discord<'_, Self>,
        route: &str,
    ) {
        let _ = discord;
        info!("network route update: {route}");
    }

    fn on_overlay_toggle(&mut self, discord: &discord_game_sdk::Discord<'_, Self>, closed: bool) {
        let _ = discord;
        info!("overlay toggle: {closed}");
    }

    fn on_relationships_refresh(&mut self, discord: &discord_game_sdk::Discord<'_, Self>) {
        let _ = discord;
        info!("relationship refresh");
    }

    fn on_relationship_update(
        &mut self,
        discord: &discord_game_sdk::Discord<'_, Self>,
        relationship: &discord_game_sdk::Relationship,
    ) {
        let _ = discord;
        info!("relationship update: {relationship:#?}");
    }

    fn on_entitlement_create(
        &mut self,
        discord: &discord_game_sdk::Discord<'_, Self>,
        entitlement: &discord_game_sdk::Entitlement,
    ) {
        let _ = discord;
        info!("entitlement create: {entitlement:#?}");
    }

    fn on_entitlement_delete(
        &mut self,
        discord: &discord_game_sdk::Discord<'_, Self>,
        entitlement: &discord_game_sdk::Entitlement,
    ) {
        let _ = discord;
        info!("entitlement delete: {entitlement:#?}");
    }

    fn on_current_user_update(&mut self, discord: &discord_game_sdk::Discord<'_, Self>) {
        let _ = discord;
        info!("current user update");
    }

    fn on_voice_settings_update(&mut self, discord: &discord_game_sdk::Discord<'_, Self>) {
        let _ = discord;
        info!("voice settings update");
    }
}
