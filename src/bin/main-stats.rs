use forzanalyst::forza;
use std::env::args;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let input = args().nth(1).expect("usage: main-stats <file.ftm>");
    let packets = forza::read_packets(&mut File::open(input).unwrap())?;

    for p in packets {
        println!("{}", p.current_race_time);
    }

    Ok(())
}
