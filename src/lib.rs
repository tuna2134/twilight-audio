pub mod gateway;
pub mod client;
pub mod types;

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
}

pub type Result<T> = std::result::Result<T, Error>;
