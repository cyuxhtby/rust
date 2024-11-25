use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct BloomFilter {
    bits: Vec<bool>,
    size: usize, // Size of bit array
    k: usize, // Number of hash functions
}

impl BloomFilter {
    pub fn new(size: usize, k: usize) -> Self {
        assert!(size > 0, "size must be positive");
        assert!(k > 0, "number of hash functions must be positive");

        BloomFilter {
            bits: vec![false; size],
            size, 
            k
        }
    }

    pub fn insert<T: Hash>(&mut self, item: &T) {
        for i in 0..self.k {
            let hash_index = self.get_hash(item, i);
            self.bits[hash_index] = true;
        }
    }

    pub fn contains<T: Hash>(&self, item: &T) -> bool {
        for i in 0..self.k {
            let hash_index = self.get_hash(item, i);        
            if !self.bits[hash_index] {
                return false;
            }
        }
        true
    }

    pub fn clear(&mut self) {
        self.bits.fill(false);
    }

    fn get_hash<T: Hash>(&self, item: &T, index: usize) -> usize {
        let mut hasher =  DefaultHasher::new();
        item.hash(&mut hasher);
        let hash = hasher.finish();
        ((hash as usize) + index * (hash as usize)) % self.size

    }
}
