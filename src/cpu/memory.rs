use std::ops::{Index, IndexMut};

const MAX_MEM: usize = 1024 * 64;

pub struct Memory {
    data: [u8; MAX_MEM],
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            data: [0; MAX_MEM],
        }
    }
}

impl Index<usize> for Memory {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
