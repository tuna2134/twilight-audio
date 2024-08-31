use std::marker::PhantomData;

use tokio::sync::oneshot::{channel, error::RecvError, Receiver, Sender};
use twilight_model::{
    gateway::{
        event::Event,
        payload::incoming::{VoiceServerUpdate, VoiceStateUpdate},
    },
    id::{
        marker::{ChannelMarker, GuildMarker},
        Id,
    },
};

use crate::{gateway::DiscordVoiceClient, Error, Result};

#[async_trait::async_trait]
pub trait VoiceUpdate {
    /// Send a voice update message to the inner shard handle.
    async fn update_voice_state(
        &self,
        guild_id: Id<GuildMarker>,
        channel_id: Option<Id<ChannelMarker>>,
        self_deaf: bool,
        self_mute: bool,
    ) -> Result<()>;
}

pub struct PartialVoiceStateUpdate {
    pub session_id: String,
    pub channel_id: Option<Id<ChannelMarker>>,
}

enum Connection {
    Handshaking {
        server: Option<VoiceServerUpdate>,
        state: Option<PartialVoiceStateUpdate>,
    },
    Establishing,
    Connected(DiscordVoiceClient),
    Disconnected,
}

impl Connection {
    pub fn is_ready(&self) -> bool {
        matches!(
            self,
            Self::Handshaking {
                server: Some(_),
                state: Some(_)
            }
        )
    }

    pub fn is_connected(&self) -> bool {
        matches!(self, Self::Connected(_))
    }

    pub fn is_disconnected(&self) -> bool {
        matches!(self, Self::Disconnected)
    }
}

pub struct VoiceClient<D: VoiceUpdate> {
    driver: D,
    guild_id: Id<GuildMarker>,
    channel_id: Id<ChannelMarker>,
    connection: Connection,
}

impl<D: VoiceUpdate> VoiceClient<D> {
    pub fn new(driver: D, guild_id: Id<GuildMarker>, channel_id: Id<ChannelMarker>) -> Self {
        Self {
            driver,
            guild_id,
            channel_id,
            connection: Connection::Disconnected,
        }
    }

    async fn establish_connection(&mut self) -> Result<()> {
        let connection = std::mem::replace(&mut self.connection, Connection::Establishing);

        if let Connection::Handshaking {
            server: Some(voice_server),
            state: Some(voice_state),
        } = connection
        {
            self.connection = Connection::Connected(
                DiscordVoiceClient::connect(voice_server, voice_state).await?,
            );

            Ok(())
        } else {
            unreachable!("VoiceClient is not ready to establish connection. This is bug.")
        }
    }

    pub async fn on_voice_server_update(&mut self, data: VoiceServerUpdate) -> Result<()> {
        if let Connection::Handshaking { server, .. } = &mut self.connection {
            if server.is_some() {
                todo!("Handle duplicate connecting.")
            } else {
                *server = Some(data);

                if self.connection.is_ready() {
                    self.establish_connection().await?;
                }
            }
        }

        Ok(())
    }

    pub async fn on_voice_state_update(&mut self, data: PartialVoiceStateUpdate) -> Result<()> {
        if let Connection::Handshaking { state, .. } = &mut self.connection {
            if state.is_some() {
                todo!("Handle duplicate connecting.")
            } else {
                *state = Some(data);

                if self.connection.is_ready() {
                    self.establish_connection().await?;
                }
            }
        }

        Ok(())
    }

    pub async fn join(&mut self, self_deaf: bool, self_mute: bool) -> Result<()> {
        if !self.connection.is_disconnected() {
            return Err(Error::AlreadyJoined);
        }

        self.connection = Connection::Handshaking {
            server: None,
            state: None,
        };
        self.driver
            .update_voice_state(self.guild_id, Some(self.channel_id), self_deaf, self_mute)
            .await?;

        Ok(())
    }
}
