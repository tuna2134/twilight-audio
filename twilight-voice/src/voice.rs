use tokio::net::UdpSocket;
use discortp::discord::{IpDiscoveryPacket, IpDiscoveryType, MutableIpDiscoveryPacket};

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

    pub async fn ip_discovery(&self, ssrc: u32) -> Result<()> {
        let mut buffer = [0; IpDiscoveryPacket::const_packet_size()];

        {
            let mut packet = MutableIpDiscoveryPacket::new(&mut buffer).unwrap();
            packet.set_pkt_type(IpDiscoveryType::Request);
            packet.set_ssrc(ssrc);
            packet.set_length(70);
        }

        self.udp_socket.send(&buffer).await?;

        let (ip, port) = loop {
            let (len, _) = self.udp_socket.recv_from(&mut buffer).await?;
            if let Some(packet) = IpDiscoveryPacket::new(&buffer[..len]) {
                if packet.get_pkt_type() == IpDiscoveryType::Response {
                    break (packet.get_address(), packet.get_port());
                }
            }
        };

        println!("{:?}", ip);

        Ok(())
    }
}
