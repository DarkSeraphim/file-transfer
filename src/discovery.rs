use std::{net::{Ipv4Addr, SocketAddr}, sync::mpsc::Sender};

use anyhow::{Error, anyhow};
use tokio::{net::UdpSocket, sync::oneshot::Receiver, select};

struct DiscoveryServer {
}

struct DiscoveryClient {
    local_port: u16,
    multicast_group_addr: Ipv4Addr,
}

type Peer = (String, SocketAddr);

async fn read_udp(socket: &mut UdpSocket, sink: Sender<Peer>) -> Result<(), Error> {
    loop {
        let mut buf = [0; 128];
        let (len, addr) = socket.recv_from(&mut buf).await?;
        let name = String::from_utf8(Vec::from(&buf[..len]));
        match name {
            Ok(n) => sink.send((n, addr))?,
            Err(err) => {
                dbg!(err);
            }
        }
    }
}


impl DiscoveryClient {
    pub fn new(local_port: u16, multicast_group_addr: Ipv4Addr) -> Self {
        Self {
            local_port,
            multicast_group_addr,
        }
    }

    pub async fn search(&self, cancel: Receiver<()>,sink: Sender<(String, SocketAddr)>) -> Result<(), anyhow::Error> {
        let mut socket = UdpSocket::bind((Ipv4Addr::new(0, 0, 0, 0), self.local_port)).await?;
        socket.join_multicast_v4(self.multicast_group_addr, Ipv4Addr::UNSPECIFIED)?;
        select! {
            output = read_udp(&mut socket, sink) => output,
            _ = cancel => Err(anyhow!(""))
        }?; 
        return Ok(());
    }
}
