use forzanalyst::forza::forza_packet::*;
use std::{collections::LinkedList, fs::File};

fn main() {
    let input = std::env::args().nth(1).unwrap();
    let packets = read_packets(&mut File::open(input).unwrap());

    let mut chunks = LinkedList::from([ForzaChunk::new()]);
    for p in packets {
        if p.current_race_time == 0.0 {
            println!("{:#?}", p);
            match (p.game_mode(), chunks.back().unwrap().game_mode()) {
                (ForzaGameMode::FreeRoam, ForzaGameMode::FreeRoam) => {}
                _ => chunks.push_back(ForzaChunk::new()),
            }
        }

        chunks.back_mut().unwrap().push(p);
    }

    for c in chunks {
        println!("{:?}", c.game_mode());
    }
}
