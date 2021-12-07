use forzanalyst::forza_packet::*;
use std::fs::File;

fn main() {
    let input = std::env::args().nth(1).unwrap();
    let packets = read_packets(&mut File::open(input).unwrap());

    for p in packets {
        println!("{}", p.current_race_time);
    }
}
