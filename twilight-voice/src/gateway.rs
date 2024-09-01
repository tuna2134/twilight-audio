use futures_util::StreamExt;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    tungstenite::{protocol::WebSocketConfig, Message},
    MaybeTlsStream, WebSocketStream,
};

use twilight_voice_model::Event;
use twilight_model::{
    gateway::payload::incoming::{VoiceServerUpdate},
};

use crate::{client::PartialVoiceStateUpdate, Result};

pub struct DiscordVoiceClient {
    pub websocket: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl DiscordVoiceClient {
    pub async fn connect(
        voice_server: VoiceServerUpdate,
        _voice_state: PartialVoiceStateUpdate,
    ) -> Result<Self> {
        let uri = format!("wss://{}/?v=8", voice_server.endpoint.unwrap(),);

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

    pub async fn send_heartbeat(&mut self, message: Message) -> Result<()> {
        Ok(())
    }
}
