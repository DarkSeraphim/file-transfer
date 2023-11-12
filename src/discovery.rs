use std::net::{UdpSocket, Ipv4Addr, SocketAddrV4};

struct DiscoveryServer {
  local_port: u16,
  multicast_group_addr: Ipv4Addr
}

struct DiscoveryClient {}


impl DiscoveryServer  {
    pub fn publish(&self) -> Result<(), anyhow::Error> {
        let socket = UdpSocket::bind((Ipv4Addr::new(0, 0, 0, 0), self.local_port))?;
        socket.join_multicast_v4(&self.multicast_group_addr, &Ipv4Addr::UNSPECIFIED)?;
        


        return Ok(());
    }
}
