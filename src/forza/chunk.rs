use std::{collections::HashMap, mem::replace};

use crate::event::{Event, EventGenerator};

use super::*;

//pub type Chunks = std::collections::LinkedList<Chunk>;

pub type ChunkId = usize;
pub type LapId = Option<u16>;

#[derive(PartialEq, Default, Clone, Copy)]
pub struct ChunkSelector(pub ChunkId, pub LapId);

#[derive(Default)]
pub struct Chunks {
    chunks: std::collections::LinkedList<Chunk>,
}

impl Chunks {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn chunkify(&mut self, packets: impl Iterator<Item = Packet>) {
        if self.chunks.is_empty() {
            self.chunks.push_back(Chunk::new());
        };

        for p in packets {
            if p.game_mode() != self.last_game_mode() {
                if !self.chunks.back().unwrap().is_empty() {
                    self.finalize_last_chunk();
                }
            }

            self.chunks.back_mut().unwrap().push(p);
        }
    }

    pub fn finalize_last_chunk(&mut self) {
        self.chunks.back_mut().unwrap().finalize();
        self.chunks.push_back(Chunk::new());
    }

    pub fn remove_chunk(&mut self, chunk_selector: &ChunkSelector) {
        match *chunk_selector {
            ChunkSelector(chunk_id, None) => {self._remove_chunk(chunk_id)},
            ChunkSelector(chunk_id, Some(lap_num)) => {
                let chunk = self.chunks.iter_mut().nth(chunk_id).unwrap();
                chunk.remove_lap(lap_num);
                if chunk.packets.is_empty() {
                    self._remove_chunk(chunk_id);
                }
            }
        }
    }

    fn _remove_chunk(&mut self, id: ChunkId) {
        let mut split_list = self.chunks.split_off(id);
        split_list.pop_front();
        self.chunks.append(&mut split_list);
    }

    pub fn last_game_mode(&self) -> GameMode {
        if self.chunks.back().is_some() {
            self.chunks.back().unwrap().game_mode()
        } else {
            GameMode::None
        }
    }

    pub fn last_chunk_selector(&self) -> ChunkSelector {
        self.generate_selector(self.chunks.len() - 1)
    }

    pub fn game_mode_of(&self, chunk_selector: ChunkSelector) -> GameMode {
        match chunk_selector {
            ChunkSelector(chunk_id, _) => {
                if let Some(chunk) = self.chunks.iter().nth(chunk_id) {
                    chunk.game_mode()
                } else {
                    GameMode::None
                }
            }
        }
    }

    pub fn list(&self) -> &std::collections::LinkedList<Chunk> {
        &self.chunks
    }

    fn generate_selector(&self, chunk_id: ChunkId) -> ChunkSelector {
        let chunk = self.chunks.iter().nth(chunk_id);
        let mut lap_id: LapId = None;

        if let Some(chunk) = chunk {
            lap_id = match chunk.game_mode() {
                GameMode::Race => Some(chunk.lap_count() - 1),
                _ => None,
            };
        }

        ChunkSelector(chunk_id, lap_id)
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
            .unwrap_or(GameMode::None)
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
