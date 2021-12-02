use std::net::UdpSocket;

mod forza_packet;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:7555").expect("couldn't bind to address");

    loop {
        let mut packet = forza_packet::ForzaPacket::default();
        socket.recv_from(packet.as_buf()).expect("no data received");
        println!("{:#?}", packet);
    }
}
