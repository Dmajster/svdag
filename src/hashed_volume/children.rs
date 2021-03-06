use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Debug, Default)]
pub struct Children {
    pub child_bits: u8,
}

impl Children {
    pub fn new(child_bits: u8) -> Children {
        Children { child_bits }
    }

    pub fn get(&self, child_index: usize) -> bool {
        (self.child_bits >> child_index) & 1 > 0
    }

    pub fn get_n(&self, child_index: usize) -> usize {
        let mut count = 0;
        for index in 0..child_index {
            if self.get(index) {
                count += 1;
            }
        }
        count
    }

    pub fn set(&mut self, child_index: usize, has_child: bool) {
        match has_child {
            true => self.child_bits |= 1 << child_index,
            false => self.child_bits &= !(1 << child_index),
        };
    }

    pub fn have_occupied_children(&self) -> bool {
        self.child_bits > 0b0000_0000
    }

    pub fn count_occupied(&self) -> usize {
        self.child_bits.count_ones() as usize
    }
}

impl Hash for Children {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.child_bits.hash(state);
    }
}
