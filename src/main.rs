mod discovery;

use std::{env, net::{UdpSocket, Ipv4Addr, TcpListener, TcpStream}, io::{Read, Write}};



fn main() {
    let addr = Ipv4Addr::new(224, 0, 0, 255);
    let args = env::args().collect::<Vec<String>>();
    let is_server = &args[1] == "server";

    if is_server {
        let socket = UdpSocket::bind("0.0.0.0:12345").expect("UDP bind failed");
//        loop {
            socket.join_multicast_v4(&addr, &Ipv4Addr::UNSPECIFIED).expect("Join failed");
            let mut buf = [0; 1024];
            let (len, peer_addr) = socket.recv_from(&mut buf).expect("Failed to receive packet");
            let name = String::from_utf8(Vec::from(&buf[..len])).expect("Invalid utf8");
            println!("Hello there {name}, from {peer_addr}");
            let mut stream = TcpStream::connect(&peer_addr).expect("Connecting pls");
            stream.write_all("Hello peer".as_bytes()).expect("Failed to write back"); 
            socket.leave_multicast_v4(&addr, &Ipv4Addr::UNSPECIFIED).expect("Failed to leave");
  //      }
    } else {
        let listener = TcpListener::bind("0.0.0.0:12346").expect("I need to listen a bit");
        let socket = UdpSocket::bind("0.0.0.0:12346").expect("UDP bind failed");
        socket.send_to("Client".as_bytes(), (addr, 12345)).expect("Failed to send message");
        let (mut client, _) = listener.accept().expect("Testing port reuse");
        let mut buf = Vec::new();
        client.read_to_end(&mut buf).expect("Failed to read response");
        let msg = String::from_utf8(buf).expect("Invalid utf8");
        println!("The response was {msg}");
    }
}
