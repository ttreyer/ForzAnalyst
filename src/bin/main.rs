use std::sync::Arc;
use std::{
    net::UdpSocket,
    sync::atomic::{AtomicBool, Ordering},
};

use forzanalyst::forza_packet::ForzaPacket;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:7024").expect("couldn't bind to address");
    println!("Listening on {:?}...", socket.local_addr().unwrap());

    let do_print_next_packet = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(29, Arc::clone(&do_print_next_packet)).expect("signal");

    loop {
        let mut packet = ForzaPacket::default();
        socket.recv_from(packet.as_buf()).expect("no data received");

        if do_print_next_packet.load(Ordering::Relaxed) {
            println!("{:#?}", packet);
            do_print_next_packet.store(false, Ordering::Relaxed);
        }
    }
}
