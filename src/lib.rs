/*********************************************************************************
 * TODO: 
 * Should query return a result? There is an unwrap there but it will only fail if
 * something goes very very wrong. Probably better to error though since this is a 
 * library.
 *
 * Write some tests
 *
 * Add scaleable bloom filter
 *
 * Write Docs
 *
 * Publish on crates.io
 *
 * Make Generic over hash functions? (must be 128 bit? or maybe add one for 64 bit)
 * Test with twox-hash crate instead of SIP. May be better.
 *
 * A good Benchmark / Test suite which shows false positive rate / memory usage / time
 * over a bunch of scenarios
 *
 ********************************************************************************/ 

/// BloomFilter is a fast, tested and benchmarked bloom filter library.
///
/// It is designed to be as general and extensible as possible.

extern crate bit_vec;

use bit_vec::BitVec;
use std::hash::{Hash, SipHasher, Hasher};
use std::fmt;


pub struct BloomFilter {
    size: usize,
    hash_count: usize,
    bloom: BitVec,
    bits_full: usize,
    hashers: [SipHasher; 2]
}

impl fmt::Debug for BloomFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BloomFilter: size: {:?}, hash_count: {:?}, bits_full: {:?}, fill ratio: {:?}", self.size, self.hash_count, self.bits_full, self.bits_full as f32/self.size as f32)
    }
}

impl BloomFilter {
    /// Constructs a new `BloomFilter`
    ///
    /// Takes `size` and `hash_count`. These correspond to m and k in standard Bloom Filter
    /// Descriptions.
    ///
    /// `size`: The size of the bit vector being stored. (m)
    /// `hash_count`: The number of hash functions to use. (k)
    pub fn new(size: usize, hash_count: usize) -> BloomFilter {
        BloomFilter {
            size: size,
            hash_count: hash_count,
            bloom: BitVec::from_elem(size, false),
            bits_full: 0,
            hashers: [SipHasher::new(), SipHasher::new()]
        }
    }

    /// Constructs a new `BloomFilter` using desired error rate and number of items 
    ///
    /// `n`: The number of items that are going to be stored in the bloom filter.
    /// `p`: The allowable error rate of false positives
    pub fn new_with_params(n: usize, p: f32) -> BloomFilter {
        let m = ((-(n as f32 * (p.ln()))).ceil() / ((2.0f32).ln().powi(2))) as usize;
        let k = (((2.0f32).ln() * (m as f32/ n as f32)).round()) as usize;

        BloomFilter {
            size: m,
            hash_count: k,
            bloom: BitVec::from_elem(m, false),
            bits_full: 0,
            hashers: [SipHasher::new(), SipHasher::new()]
        }
    }

    /// Add a Hashable type to the bloom filter.
    pub fn add<T: Hash>(&mut self, val: &T) {
        // Generate two random u64s for each hash and seed SipHaser with that.
        for n in 0..self.hash_count {
            let seed = n as u64;
            let hashed = self.bloom_hash(seed, &val);
            if !self.bloom.get(hashed).unwrap() {
                self.bits_full += 1;
            }
            self.bloom.set(hashed, true);
        }
    }
    
    /// Query the bloom filter for some Hashable value.
    pub fn query<T: Hash>(&self, query_val: &T) -> bool{
        for n in 0..self.hash_count {
            let seed = n as u64;
            let hashed = self.bloom_hash(seed, &query_val);
            if !self.bloom.get(hashed).unwrap() {
                return false;
            }
        }
        true
    }
    
    // Uses Hash[i] = (hash_64_part_0 * hash_64_part_1 + i) as per 
    // http://spyced.blogspot.com/2009/01/all-you-ever-wanted-to-know-about.html
    // SipHash seems like it should be reasonable for bloom applications
    // TODO: It would probably be good to make this generic over a hash function
    // in the same way as hash map.
    fn bloom_hash<T: Hash>(&self, seed: u64, val: &T) -> usize {
        let mut sip1 = self.hashers[0].clone();
        let mut sip2 = self.hashers[1].clone();
        val.hash(&mut sip1);
        val.hash(&mut sip2);
        let fin1 = sip1.finish();
        let fin2 = sip2.finish();
        (fin1.wrapping_mul(fin2.wrapping_add(seed)) % (self.size as u64)) as usize
    }
    
    /// Returns the number of bits set in the bloom filter.
    ///
    /// This is an expensive operation, there's no reason to use it unless you're 
    /// doing something strange
    pub fn get_bits_set(&self) -> usize {
        self.bloom.iter().filter(|x| *x).count()
    }
        
    /// Clears the bloom filter of all values.
    pub fn clear(&mut self) {
        self.bloom.clear();
        self.bits_full = 0;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

