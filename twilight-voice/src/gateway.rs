use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    tungstenite::{protocol::WebSocketConfig, Message},
    MaybeTlsStream, WebSocketStream,
};

use twilight_model::gateway::payload::incoming::VoiceServerUpdate;
use twilight_voice_model::{
    payload::{Heartbeat, Identify},
    Event,
};

use crate::{client::PartialVoiceStateUpdate, Result};

pub struct DiscordVoiceClient {
    pub websocket: WebSocketStream<MaybeTlsStream<TcpStream>>,
    pub seq: i64,
    heartbeat_interval: Option<f64>,
    voice_server: VoiceServerUpdate,
    voice_state: PartialVoiceStateUpdate,
}

impl DiscordVoiceClient {
    pub async fn connect(
        voice_server: VoiceServerUpdate,
        voice_state: PartialVoiceStateUpdate,
    ) -> Result<Self> {
        let uri = format!("wss://{}/?v=8", voice_server.endpoint.clone().unwrap(),);

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

        let mut client = Self {
            websocket,
            seq: -1,
            heartbeat_interval: None,
            voice_server,
            voice_state,
        };

        client.poll().await?;
        client.poll().await?;
        client.poll().await?;
        client.poll().await?;

        Ok(client)
    }

    pub async fn poll(&mut self) -> Result<()> {
        let Some(message) = self.websocket.next().await else {
            return Ok(());
        };

        println!("{:?}", message);

        if let Message::Text(data) = message? {
            println!("{}", data);
            let event: Event = serde_json::from_str(&data)?;

            match event {
                Event::Hello(data) => {
                    self.heartbeat_interval = Some(data.heartbeat_interval);
                    self.send_identify().await?;
                }
                Event::Ready(data) => {
                    self.send_heartbeat().await?;
                }
                _ => {}
            }
        }

        Ok(())
    }

    pub async fn send_identify(&mut self) -> Result<()> {
        let identify = Event::Identify(Identify {
            server_id: self.voice_server.guild_id,
            session_id: self.voice_state.session_id.clone(),
            token: self.voice_server.token.clone(),
            user_id: self.voice_state.user_id,
        });
        self.websocket
            .send(Message::Text(serde_json::to_string(&identify)?))
            .await?;
        Ok(())
    }

    pub async fn send_heartbeat(&mut self) -> Result<()> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_millis() as u64;
        let heartbeat = Event::Heartbeat(Heartbeat {
            t: now,
            seq_ack: self.seq,
        });
        self.websocket
            .send(Message::Text(serde_json::to_string(&heartbeat)?))
            .await?;
        Ok(())
    }
}
