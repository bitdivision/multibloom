#![feature(test)]

extern crate multibloom;
extern crate test;
extern crate rand;
extern crate twox_hash;

use test::{Bencher, black_box};
use multibloom::BloomFilter;
use rand::Rng;
use twox_hash::RandomXxHashBuilder;

#[bench]
fn bench_add_1_sip(b: &mut Bencher) {
    let mut bloom = BloomFilter::new(1000, 10);
    let bytes = rand::thread_rng().gen_iter::<u8>().take(1).collect::<Vec<u8>>();
    b.iter(|| {
        bloom.add(&bytes);
        black_box(&bloom);
    });
}


#[bench]
fn bench_add_64_sip(b: &mut Bencher) {
    let mut bloom = BloomFilter::new(1000, 10);
    let bytes = rand::thread_rng().gen_iter::<u8>().take(64).collect::<Vec<u8>>();

    b.iter(|| {
        bloom.add(&bytes);
        black_box(&bloom);
    });
}

#[bench]
fn bench_add_1k_sip(b: &mut Bencher) {
    let mut bloom = BloomFilter::new(1000, 10);
    let bytes = rand::thread_rng().gen_iter::<u8>().take(1024).collect::<Vec<u8>>();
    b.iter(|| {
        bloom.add(&bytes);
        black_box(&bloom);
    });
}

#[bench]
fn bench_add_1_xx_hash(b: &mut Bencher) {
    let mut bloom = BloomFilter::new_with_hasher(RandomXxHashBuilder::default(), 1000, 10);
    let bytes = rand::thread_rng().gen_iter::<u8>().take(1).collect::<Vec<u8>>();
    b.iter(|| {
        bloom.add(&bytes);
        black_box(&bloom);
    });
}

#[bench]
fn bench_add_64_xx_hash(b: &mut Bencher) {
    let mut bloom = BloomFilter::new_with_hasher(RandomXxHashBuilder::default(), 1000, 10);
    let bytes = rand::thread_rng().gen_iter::<u8>().take(64).collect::<Vec<u8>>();

    b.iter(|| {
        bloom.add(&bytes);
        black_box(&bloom);
    });
}

#[bench]
fn bench_add_1k_xx_hash(b: &mut Bencher) {
    let mut bloom = BloomFilter::new_with_hasher(RandomXxHashBuilder::default(), 1000, 10);
    let bytes = rand::thread_rng().gen_iter::<u8>().take(1024).collect::<Vec<u8>>();
    b.iter(|| {
        bloom.add(&bytes);
        black_box(&bloom);
    });
}

