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
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;
    use twilight_gateway::{Event, Intents, Shard, ShardId};

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let intents = Intents::GUILD_MESSAGES | Intents::GUILD_VOICE_STATES;

        let mut shard = Shard::new(ShardId::ONE, env::var("DISCORD_TOKEN")?, intents);

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
                }
                _ => {}
            }
        }
        Ok(())
    }
}