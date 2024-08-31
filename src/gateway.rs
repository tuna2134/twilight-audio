use futures_util::StreamExt;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    tungstenite::{protocol::WebSocketConfig, Message},
    MaybeTlsStream, WebSocketStream,
};

use serenity_voice_model::Event;
use twilight_model::{gateway::payload::incoming::VoiceServerUpdate, voice::VoiceState};

use crate::Result;

pub struct DiscordVoiceClientBuilder {
    pub voice_server_update: VoiceServerUpdate,
    pub voice_state: VoiceState,
}

impl DiscordVoiceClientBuilder {
    pub fn voice_server_update(mut self, voice_server_update: VoiceServerUpdate) -> Self {
        self.voice_server_update = voice_server_update;
        self
    }

    pub fn voice_state(mut self, voice_state: VoiceState) -> Self {
        self.voice_state = voice_state;
        self
    }
}

pub struct DiscordVoiceClient {
    pub websocket: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl DiscordVoiceClient {
    pub async fn connect(builder: DiscordVoiceClientBuilder) -> Result<Self> {
        let uri = format!(
            "wss://{}/?v=4",
            builder.voice_server_update.endpoint.unwrap(),
        );

        let (websocket, _) = tokio_tungstenite::connect_async_tls_with_config(
            uri,
            Some(WebSocketConfig {
                max_message_size: None,
                max_frame_size: None,
                ..Default::default()
            }),
            true,
            None,
        )
        .await?;

        let mut client = Self { websocket };

        client.poll().await?;

        Ok(client)
    }

    pub async fn poll(&mut self) -> Result<()> {
        let Some(message) = self.websocket.next().await else {
            return Ok(());
        };

        if let Message::Text(data) = message? {
            let event: Event = serde_json::from_str(&data)?;

            match event {
                Event::Hello(data) => {
                    println!("Heartbeat interval: {}", data.heartbeat_interval);
                }
                _ => {}
            }
        }

        Ok(())
    }
}
