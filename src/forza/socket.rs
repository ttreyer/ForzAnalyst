use std::{
    net::UdpSocket,
    sync::mpsc::{Iter, Receiver, TryIter},
};

use super::*;

pub struct Socket {
    _thread: std::thread::JoinHandle<()>,
    receiver: Receiver<Packet>,
}

impl Default for Socket {
    fn default() -> Self {
        Self::new("0.0.0.0:7024")
    }
}

impl Socket {
    pub fn new(addr: &str) -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();

        let socket = UdpSocket::bind(addr).expect("couldn't bind to address");
        println!("Listening on {:?}...", socket.local_addr().unwrap());

        let thread = std::thread::spawn(move || {
            let mut last_packet_timestamp = 0u32;
            loop {
                let mut packet = Packet::default();
                socket.recv_from(packet.as_buf_mut()).unwrap();

                if packet.is_race_on == 0 {
                    continue;
                }
                if packet.timestamp_ms == last_packet_timestamp {
                    continue;
                }

                last_packet_timestamp = packet.timestamp_ms;
                sender.send(packet).ok();
            }
        });

        Self {
            _thread: thread,
            receiver,
        }
    }

    pub fn iter(&self) -> Iter<'_, Packet> {
        self.receiver.iter()
    }

    pub fn try_iter(&self) -> TryIter<'_, Packet> {
        self.receiver.try_iter()
    }
}
