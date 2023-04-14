use std::error::Error;
use std::fs::OpenOptions;
use std::path::Path;
use memmap2::MmapMut;
use crate::bit_array::BitArray;
use crate::mmh3::murmurhash3_32;

pub struct CounterBloomFilter {
    bit_array: BitArray,
    counters: MmapMut,
    size: usize,
    num_hash_functions: u32,
}

impl CounterBloomFilter {
    pub fn  new<P: AsRef<Path>>(size: usize, num_hash_functions: u32, path: P) -> Result<Self, Box<dyn Error>> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;

        file.set_len((size * 4) as u64)?;

        let counters = unsafe { MmapMut::map_mut(&file)? };
        let bit_array = BitArray::new(size);

        let mut cbf = CounterBloomFilter {
            bit_array,
            counters,
            size,
            num_hash_functions,
        };

        cbf.rebuild_bitarray();

        Ok(cbf)
    }

    fn rebuild_bitarray(&mut self){
        for index in 0..self.size {
            let counter = self.get_counter(index);
            if counter > 0 {
                self.bit_array.set(index)
            }
        }
    }

    fn get_counter(&self, index: usize) -> u32 {
        let start = index * 4;
        let end = start + 4;
        let bytes = &self.counters[start..end];
        u32::from_ne_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }

    fn set_counter(&mut self, index: usize, value: u32) {
        let start = index * 4;
        let end = start + 4;
        let bytes = value.to_ne_bytes();
        self.counters[start..end].copy_from_slice(&bytes);
    }

    pub fn insert(&mut self, data: &[u8]) {
        for i in 0..self.num_hash_functions {
            let index = self.hash(data, i);
            let counter = self.get_counter(index);
            self.set_counter(index, counter.saturating_add(1));
            if counter == 0 {
                self.bit_array.set(index);
            }
        }
    }

    pub fn contains(&self, data: &[u8]) -> bool {
        (0..self.num_hash_functions)
            .map(|i| self.hash(data, i))
            .all(|index| self.bit_array.get(index))
    }

    pub fn remove(&mut self, data: &[u8]) {
        if self.contains(data) {
            for i in 0..self.num_hash_functions {
                let index = self.hash(data, i);
                let counter = self.get_counter(index);
                self.set_counter(index, counter.saturating_sub(1));
                if counter == 1 {
                    self.bit_array.set_zero(index);
                }
            }
        }
    }

    fn flush_async(&self) -> std::io::Result<()> {
        self.counters.flush_async()
    }

    fn hash(&self, data: &[u8], i: u32) -> usize {
        (murmurhash3_32(data, i) % self.size as u32) as usize
    }
}