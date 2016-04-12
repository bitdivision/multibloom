#![feature(test)]

extern crate multibloom;
extern crate test;
extern crate rand;

use test::{Bencher, black_box};
use multibloom::BloomFilter;
use rand::Rng;

#[bench]
fn bench_add_10k(b: &mut Bencher) {
    let mut bloom = BloomFilter::new(1000, 10);
    let bytes = rand::thread_rng().gen_iter::<u8>().take(10240).collect::<Vec<u8>>();

    b.iter(|| {
        bloom.add(100);
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
