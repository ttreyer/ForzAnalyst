use forzanalyst::forza;
use std::{collections::LinkedList, fs::File};

fn main() -> std::io::Result<()> {
    let input = std::env::args().nth(1).unwrap();
    let packets = forza::read_packets(&mut File::open(input)?)?;

    let mut chunks = LinkedList::from([forza::Chunk::new()]);
    for p in packets {
        if p.current_race_time == 0.0 {
            println!("{:#?}", p);
            match (p.game_mode(), chunks.back().unwrap().game_mode()) {
                (forza::GameMode::FreeRoam, forza::GameMode::FreeRoam) => {}
                _ => chunks.push_back(forza::Chunk::new()),
            }
        }

        chunks.back_mut().unwrap().push(p);
    }

    for c in chunks {
        println!("{:?}", c.game_mode());
    }

    Ok(())
}
