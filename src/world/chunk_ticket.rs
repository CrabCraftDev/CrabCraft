use evenio::entity::EntityId;
use std::cmp::max;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct PlayerChunkTicket {
    pub entity_id: EntityId,
    pub radius: usize,
    pub chunk_x: i32,
    pub chunk_z: i32,
    current_radius: usize,
    current_index: usize,
}

impl PlayerChunkTicket {
    pub fn new(entity_id: EntityId, radius: usize, chunk_x: i32, chunk_z: i32) -> PlayerChunkTicket {
        Self {
            entity_id,
            radius,
            chunk_x,
            chunk_z,
            current_radius: 0,
            current_index: 0,
        }
    }

    pub fn reset(&mut self) {
        self.current_index = 0;
        self.current_radius = 0;
    }

    pub fn priority(&self) -> u8 {
        self.current_radius as u8
    }
    fn max_index(&self) -> usize {
        max(self.current_radius * 8, 1)
    }
}

impl Iterator for PlayerChunkTicket {
    type Item = (i32, i32);

    /// goofy spiral iterator
    ///
    /// e  f  0  1  2
    ///
    /// d  7  0  1  3
    ///
    /// c  6  0  2  4
    ///
    /// b  5  4  3  5
    ///
    /// a  9  8  7  6
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index >= self.max_index() {
            self.current_index = 0;
            self.current_radius += 1;
        }
        if self.current_radius > self.radius {
            return None;
        }
        let idx = self.current_index as i32;
        let rad = self.current_radius as i32;
        self.current_index += 1;
        if idx <= rad {
            return Some((self.chunk_x - rad, self.chunk_z + idx));
        }
        if idx <= rad * 3 {
            return Some((self.chunk_x - rad * 2 + idx, self.chunk_z + rad));
        }
        if idx <= rad * 5 {
            return Some((self.chunk_x + rad, self.chunk_z + rad * 4 - idx));
        }
        if idx <= rad * 7 {
            return Some((self.chunk_x + rad * 6 - idx, self.chunk_z - rad));
        }
        Some((self.chunk_x - rad, self.chunk_z - rad * 8 + idx))
    }
}
