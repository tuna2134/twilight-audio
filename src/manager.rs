#[derive(thiserror::Error, Debug)]
pub enum JoinError {}

pub trait VoiceUpdate {
    /// Send a voice update message to the inner shard handle.
    async fn update_voice_state(
        &self,
        guild_id: GuildId,
        channel_id: Option<ChannelId>,
        self_deaf: bool,
        self_mute: bool,
    ) -> Result<(), JoinError>
}


pub struct VoiceManager {}

impl VoiceManager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&self) {
        println!("VoiceManager update");
    }

    pub fn join(&self) {
        println!("VoiceManager join");
    }
}
