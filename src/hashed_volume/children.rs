use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Debug)]
pub struct Children {
    pub child_bits: u8,
}

impl Children {
    pub fn new() -> Children {
        Children { child_bits: 0 }
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

    pub fn are_interesting(&self) -> bool {
        self.child_bits > 0b0000_0000 //&& self.child_bits < 0b1111_1111 TODO fix early termination on completly full nodes
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
