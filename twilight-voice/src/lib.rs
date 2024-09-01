pub mod client;
pub mod gateway;
pub mod types;
pub mod voice;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("Serde_json error: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("VoiceClient is not ready to join voice channel.")]
    NotReady,
    #[error("VoiceClient is already connected or connecting to voice server.")]
    AlreadyJoined,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("SystemTime error: {0}")]
    SystemTime(#[from] std::time::SystemTimeError),
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;
    use client::{PartialVoiceStateUpdate, VoiceUpdate};
    use twilight_gateway::{Event, Intents, MessageSender, Shard, ShardId};
    use twilight_model::{
        gateway::payload::outgoing::UpdateVoiceState,
        id::{
            marker::{ChannelMarker, GuildMarker},
            Id,
        },
    };

    struct TwilightVoiceUpdate {
        sender: MessageSender,
    }

    #[async_trait::async_trait]
    impl VoiceUpdate for TwilightVoiceUpdate {
        async fn update_voice_state(
            &self,
            guild_id: Id<GuildMarker>,
            channel_id: Option<Id<ChannelMarker>>,
            self_deaf: bool,
            self_mute: bool,
        ) -> Result<()> {
            let request = UpdateVoiceState::new(guild_id, channel_id, self_deaf, self_mute);
            if let Err(e) = self.sender.command(&request) {
                println!("Error: {:?}", e);
            }
            Ok(())
        }
    }

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        dotenvy::dotenv().ok();

        let intents = Intents::GUILD_MESSAGES | Intents::GUILD_VOICE_STATES;

        let mut shard = Shard::new(ShardId::ONE, env::var("DISCORD_TOKEN")?, intents);

        let voice_update = TwilightVoiceUpdate {
            sender: shard.sender().clone(),
        };
        let guild_id = Id::new(961916734137315358);
        let channel_id = Id::new(961916734523179051);
        let mut vc = client::VoiceClient::new(voice_update, guild_id, channel_id);

        loop {
            let event = match shard.next_event().await {
                Ok(event) => event,
                Err(err) => {
                    eprintln!("Error: {:?}", err);
                    if err.is_fatal() {
                        break;
                    }
                    continue;
                }
            };

            match event {
                Event::Ready(_) => {
                    println!("Ready!");
                    vc.join(false, false).await?;
                }
                Event::VoiceServerUpdate(content) => {
                    vc.on_voice_server_update(content).await?;
                    println!("test");
                }
                Event::VoiceStateUpdate(content) => {
                    vc.on_voice_state_update(PartialVoiceStateUpdate {
                        session_id: content.session_id.clone(),
                        channel_id: content.channel_id,
                        user_id: content.user_id,
                    })
                    .await?;
                    println!("test2");
                }
                _ => {}
            }
        }
        Ok(())
    }
}
