#![feature(test)]

extern crate multibloom;
extern crate test;
extern crate rand;

use test::{Bencher, black_box};
use multibloom::BloomFilter;
use rand::Rng;

#[bench]
fn bench_add_1(b: &mut Bencher) {
    let mut bloom = BloomFilter::new(1000, 10);
    let bytes = rand::thread_rng().gen_iter::<u8>().take(1).collect::<Vec<u8>>();
    let testing = 100;
    b.iter(|| {
        bloom.add(&testing);
        black_box(&bloom);
    });
}


#[bench]
fn bench_add_64(b: &mut Bencher) {
    let mut bloom = BloomFilter::new(1000, 10);
    let bytes = rand::thread_rng().gen_iter::<u8>().take(64).collect::<Vec<u8>>();

    b.iter(|| {
        bloom.add(&bytes);
        black_box(&bloom);
    });
}


#[bench]
fn bench_add_1k(b: &mut Bencher) {
    let mut bloom = BloomFilter::new(1000, 10);
    let bytes = rand::thread_rng().gen_iter::<u8>().take(1024).collect::<Vec<u8>>();
    b.iter(|| {
        bloom.add(&bytes);
        black_box(&bloom);
    });
}
