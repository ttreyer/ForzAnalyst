use super::*;

pub type Chunks = std::collections::LinkedList<Chunk>;

pub fn chunkify(packets: impl Iterator<Item = Packet>, chunks: &mut Chunks) {
    if chunks.is_empty() {
        chunks.push_back(Chunk::new())
    };

    for p in packets {
        if p.current_race_time == 0.0 {
            match (p.game_mode(), chunks.back().unwrap().game_mode()) {
                (GameMode::FreeRoam, GameMode::FreeRoam) => {
                    // Doing nothin here, in order to
                    // merge the two FreeRoam chunks together
                }
                (_, _) => {
                    // Re-use last chunk if empty
                    if !chunks.back().unwrap().is_empty() {
                        chunks.back_mut().unwrap().finalize();
                        chunks.push_back(Chunk::new())
                    }
                }
            }
        }

        chunks.back_mut().unwrap().push(p);
    }
}

pub struct Lap(pub u16, pub usize, pub Option<usize>);
pub struct Chunk {
    pub packets: PacketVec,
    pub lap_index: Vec<Lap>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            packets: PacketVec::with_capacity(5 * 60 * 60),
            lap_index: vec![],
        }
    }

    pub fn with_packets(packets: PacketVec) -> Self {
        let mut lap_index = Vec::new();
        packets.iter().enumerate().for_each(|(packet_index, _)| {
            Self::update_index(&packets, &mut &mut lap_index, packet_index)
        });

        Chunk { packets, lap_index }
    }

    pub fn finalize(&mut self) {
        if !self.is_empty() {
            self.packets.shrink_to_fit();
        }
    }

    pub fn is_empty(&self) -> bool {
        self.packets.is_empty()
    }

    pub fn game_mode(&self) -> GameMode {
        self.packets
            .first()
            .map(|p| p.game_mode())
            .unwrap_or(GameMode::FreeRoam)
    }

    pub fn lap_count(&self) -> u16 {
        self.lap_index.len() as u16
    }

    pub fn lap_packets(&self, lap_num: u16) -> &[Packet] {
        if let Some((_, begin, end)) = self.lap_range(lap_num) {
            &self.packets[begin..end]
        } else {
            &[]
        }
    }

    fn lap_range(&self, lap_num: u16) -> Option<(usize, usize, usize)> {
        self.lap_index
            .iter()
            .enumerate()
            .find(|(_, l)| l.0 == lap_num)
            .map(|(lap_idx, lap)| (lap_idx, lap.1, lap.2.unwrap_or(self.packets.len())))
    }

    pub fn remove_lap(&mut self, lap_num: u16) {
        if let Some((lap_idx, begin, end)) = self.lap_range(lap_num) {
            drop(self.packets.drain(begin..end));
            self.lap_index.remove(lap_idx);

            let offset = end - begin;
            self.lap_index.iter_mut().skip(lap_idx).for_each(|l| {
                l.1 -= offset;
                l.2 = l.2.map(|end| end - offset);
            });
        }
    }

    pub fn push(&mut self, packet: Packet) {
        self.packets.push(packet);
        Self::update_index(&self.packets, &mut self.lap_index, self.packets.len() - 1);
    }

    fn update_index(packets: &[Packet], lap_index: &mut Vec<Lap>, packet_index: usize) {
        match &packets[..=packet_index] {
            [.., last, current] => {
                if current.lap_number != last.lap_number {
                    if let Some(Lap(_, _, end)) = lap_index.last_mut() {
                        *end = Some(packet_index);
                    }
                    lap_index.push(Lap(current.lap_number, packet_index, None));
                }
            }
            [current] => lap_index.push(Lap(current.lap_number, packet_index, None)),
            _ => {}
        }
    }
}
