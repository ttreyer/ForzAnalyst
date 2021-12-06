use std::sync::Arc;
use std::{
    fs::File,
    net::UdpSocket,
    sync::atomic::{AtomicBool, Ordering},
};

use forzanalyst::forza_packet::*;

fn main() {
    let output = std::env::args().nth(1).unwrap_or("output.ftm".to_string());

    let socket = UdpSocket::bind("0.0.0.0:7024").expect("couldn't bind to address");
    println!("Listening on {:?}...", socket.local_addr().unwrap());

    let do_print_next_packet = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(29, Arc::clone(&do_print_next_packet)).expect("siginfo");

    let stop = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&stop)).expect("sigint");

    let mut last_packet_timestamp: u32 = 0;
    let mut packets = ForzaPacketVec::with_capacity(10 * 60 * 60);
    loop {
        let mut packet = ForzaPacket::default();
        socket
            .recv_from(packet.as_buf_mut())
            .expect("no data received");

        if packet.timestamp_ms == last_packet_timestamp {
            continue;
        }

        if do_print_next_packet.load(Ordering::Relaxed) {
            println!("{:#?}", packet);
            do_print_next_packet.store(false, Ordering::Relaxed);
        }

        last_packet_timestamp = packet.timestamp_ms;
        packets.push(packet);

        if stop.load(Ordering::Relaxed) {
            break;
        }
    }

    println!();
    println!("Writting output to '{}'", output);
    write_packets(packets.iter(), &mut File::create(output).unwrap());
}
