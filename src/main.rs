use std::{mem::transmute, net::UdpSocket};

mod forza_packet;
pub use forza_packet::ForzaPacket;
use forza_packet::ForzaPacketRaw;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:7555").expect("couldn't bind to address");

    loop {
        let mut packet: ForzaPacket = ForzaPacket::default();
        socket
            .recv_from(unsafe { transmute::<&mut ForzaPacket, &mut ForzaPacketRaw>(&mut packet) })
            .expect("no data received");

        println!("{:#?}", packet);
    }
}
