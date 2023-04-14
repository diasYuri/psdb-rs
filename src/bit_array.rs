pub struct BitArray {
    bits: Vec<u8>,
    size: usize,
}

impl BitArray {
    pub fn new(size: usize) -> Self {
        BitArray {
            bits: vec![0; (size + 7) / 8],
            size,
        }
    }

    pub fn set(&mut self, index: usize) {
        if index >= self.size { panic!("out of index") }
        let byte_index = index / 8;
        let bit_index = index % 8;
        self.bits[byte_index] |= 1 << bit_index;
    }

    pub fn set_zero(&mut self, index: usize) {
        if index >= self.size { panic!("out of index") }
        let byte_index = index / 8;
        let bit_index = index % 8;
        self.bits[byte_index] &= !(1 << bit_index);
    }

    pub fn get(&self, index: usize) -> bool {
        if index >= self.size { panic!("out of index") }
        let byte_index = index / 8;
        let bit_index = index % 8;
        (self.bits[byte_index] & (1 << bit_index)) != 0
    }
}
