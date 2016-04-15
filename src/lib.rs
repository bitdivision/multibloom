/*********************************************************************************
 * TODO: 
 * Should query return a result? There is an unwrap there but it will only fail if
 * something goes very very wrong. Probably better to error though since this is a 
 * library.
 *
 * Derive Clone? This seems to throw up an error in rustc.
 *
 * Write some tests
 *
 * Add scaleable bloom filter
 *
 * Write Docs
 *
 * Publish on crates.io
 *
 *
 * A good Benchmark / Test suite which shows false positive rate / memory usage / time
 * over a bunch of scenarios
 *
 ********************************************************************************/ 

//! multibloom is a fast, tested and benchmarked bloom filter library.
//!
//! By default, the library uses SipHasher, but any hash function can be specified.

extern crate bit_vec;

use bit_vec::BitVec;
use std::hash::{Hash, SipHasher, Hasher};
use std::fmt;

pub struct BloomFilter<H: Hasher> {
    size: u64,
    hash_count: u64,
    bloom: BitVec,
    hashers: [H; 2]
}

impl<H> fmt::Debug for BloomFilter<H> where H: Hasher + Clone + Default{

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BloomFilter: size: {:?}, hash_count: {:?}, fill ratio: {:?}",
               self.size, self.hash_count, self.get_bits_set() as f32/self.size as f32)
    }
}

impl BloomFilter<SipHasher> {

    /// Default `BloomFilter` constructor using SipHasher
    ///
    /// `size`: The size of the bit vector being stored. (m)
    ///
    /// `hash_count`: The number of hash functions to use. (k)
    pub fn new(size: u64, hash_count: u64) -> BloomFilter<SipHasher> {
        BloomFilter::<SipHasher>::new_with_hasher(size, hash_count)
    }

    /// Default `BloomFilter` constructor for SipHasher using desired error rate and number of items 
    /// 
    /// `n`: The number of items that are going to be stored in the bloom filter.
    ///
    /// `p`: The allowable error rate of false positives
    pub fn new_with_params(n: u64, p: f32) -> BloomFilter<SipHasher> {
        BloomFilter::<SipHasher>::new_with_params_with_hasher(n, p)
    }
}

impl<H> BloomFilter<H> where H: Hasher + Clone + Default {
    
    /// Constructs a bloom filter, generic over hash function
    pub fn new_with_hasher(size: u64, hash_count: u64) -> BloomFilter<H> {
        BloomFilter {
            size: size,
            hash_count: hash_count,
            bloom: BitVec::from_elem(size as usize, false),
            hashers: [H::default(), H::default()]
        }
    }
    
    /// Constructs a new Generic `BloomFilter` using desired error rate and number of items 
    ///
    /// `n`: The number of items that are going to be stored in the bloom filter.
    ///
    /// `p`: The allowable error rate of false positives
    pub fn new_with_params_with_hasher(n: u64, p: f32) -> BloomFilter<H> {
        let m = ((-(n as f32 * (p.ln()))).ceil() / ((2.0f32).ln().powi(2))) as u64;
        let k = (((2.0f32).ln() * (m as f32/ n as f32)).round()) as u64;
        
        BloomFilter {
            size: m,
            hash_count: k,
            bloom: BitVec::from_elem(m as usize, false),
            hashers: [H::default(), H::default()]
        }
    }

    /// Add a Hashable type to the bloom filter.
    pub fn add<T: Hash>(&mut self, val: &T) {
        // Generate two random u64s for each hash and seed SipHaser with that.
        for n in 0..self.hash_count {
            let seed = n as u64;
            let hashed = self.bloom_hash(seed, &val);
            self.bloom.set(hashed, true);
        }
    }
    
    /// Query the bloom filter for some Hashable value.
    pub fn query<T: Hash>(&self, query_val: &T) -> bool{
        for n in 0..self.hash_count {
            let hashed = self.bloom_hash(n as u64, &query_val);
            if !self.bloom.get(hashed).unwrap() {
                return false;
            }
        }
        true
    }
    
    // Uses Hash[i] = (hash_64_part_0 * hash_64_part_1 + i) as per 
    // http://spyced.blogspot.com/2009/01/all-you-ever-wanted-to-know-about.html
    fn bloom_hash<T: Hash>(&self, n: u64, val: &T) -> usize {
        let mut sip1 = self.hashers[0].clone();
        let mut sip2 = self.hashers[1].clone();
        val.hash(&mut sip1);
        val.hash(&mut sip2);
        let fin1 = sip1.finish();
        let fin2 = sip2.finish();
        (fin1.wrapping_mul(fin2.wrapping_add(n)) % (self.size as u64)) as usize
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
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

