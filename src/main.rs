use std::{
    mem::{size_of, transmute},
    net::UdpSocket,
};

mod forza_packet;
pub use forza_packet::ForzaPacket;

fn main() {
    let socket = UdpSocket::bind("localhost:7555").expect("couldn't bind to address");

    loop {
        let mut buf = [0; size_of::<ForzaPacket>()];
        let (_amt, _src) = socket.recv_from(&mut buf).expect("no dta received");

        unsafe {
            let packet = transmute::<[u8; size_of::<ForzaPacket>()], ForzaPacket>(buf);
            println!("{:?}", packet);
        }
    }
}
