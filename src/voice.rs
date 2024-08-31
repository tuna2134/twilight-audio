use tokio::net::UdpSocket;

use crate::Result;

pub struct DiscordVoiceConnection {
    pub udp_socket: UdpSocket,
}

impl DiscordVoiceConnection {
    pub async fn connect(ip: String, port: u16) -> Result<Self> {
        let udp_socket = UdpSocket::bind("0.0.0.0:0").await?;
        udp_socket.connect(format!("{}:{}", ip, port)).await?;

        Ok(Self { udp_socket })
    }

    pub async fn ip_discovery(&self, ssrc: u32) -> Result<(String, u16)> {
        let mut buffer = [0u8; 70];
        buffer[0..2].copy_from_slice(&1u16.to_be_bytes());
        buffer[2..4].copy_from_slice(&70u16.to_be_bytes());
        buffer[4..6].copy_from_slice(&ssrc.to_be_bytes());
        self.udp_socket.send(&buffer).await?;

        let (ip, port) = loop {
            let mut buffer = [0u8; 74];
            self.udp_socket.recv(&mut buffer).await?;
            let ip = String::from_utf8_lossy(&buffer[8..72]).to_string();
            let port = u16::from_be_bytes([buffer[72], buffer[73]]);
            break (ip, port);
        };

        Ok((ip, port))
    }
}
