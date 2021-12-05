use forzanalyst::forza_packet::*;
use std::fs::File;

fn main() {
    let input = std::env::args().nth(1).unwrap_or("output.ftm".to_string());
    let packets = read_packets(&mut File::open(input).unwrap());

    let mut dedup_packets = ForzaPacketVec::with_capacity(packets.len());
    let mut last_packet_timestamp: u32 = 0;
    for packet in packets {
        if packet.timestamp_ms == last_packet_timestamp {
            continue;
        }

        last_packet_timestamp = packet.timestamp_ms;
        dedup_packets.push(packet);
    }

    let output = std::env::args()
        .nth(2)
        .unwrap_or("output-dedup.ftm".to_string());
    write_packets(&dedup_packets, &mut File::create(output).unwrap());

    // println!("Packets read: {:}", packets.len());

    // for i in 0..10 {
    //     println!("{:?}", packets[i]);
    // }
    // println!(
    //     "First packet: {}",
    //     packets.last().unwrap().timestamp_ms - packets.first().unwrap().timestamp_ms
    // );

    // for i in 1..packets.len() {
    //     let dt = packets[i].timestamp_ms - packets[i - 1].timestamp_ms;
    //     println!("{}", dt);
    // }

    // for i in 1..packets.len() {
    //     let mut h1 = std::collections::hash_map::DefaultHasher::new();
    //     let mut h2 = std::collections::hash_map::DefaultHasher::new();
    //     packets[i].hash(&mut h1);
    //     packets[i - 1].hash(&mut h2);
    //     let dt = packets[i].timestamp_ms - packets[i - 1].timestamp_ms;
    //     if dt == 0 {
    //         println!("{:#?}", packets[i - 1]);
    //         println!("{:#?}", packets[i]);
    //         break;
    //     }
    //     println!("{} -> {}", dt, h1.finish() == h2.finish());
    // }

    // for packet in packets {
    //     let c = if packet.is_race_on == 0 { '.' } else { 'R' };
    //     print!("{}", c);
    // }
    // println!();

    // let mut count = 0;
    // for packet in packets {
    //     if packet.is_race_on == 0 {
    //         if count == 10 {
    //             break;
    //         }

    //         println!("{:4} {}", count, packet.timestamp_ms);
    //         count += 1;
    //     }
    // }
}
