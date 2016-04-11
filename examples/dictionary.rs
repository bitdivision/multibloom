extern crate multibloom;

use multibloom::BloomFilter;

use std::fs::File;
use std::path::Path;
use std::io::Read;

fn main() {
    let path = Path::new("/usr/share/dict/words");
    let mut dict_file = File::open(&path).unwrap();
    let mut word_list = String::new();
    dict_file.read_to_string(&mut word_list).ok();

    let words: Vec<&str> = word_list.split('\n').collect();

    let expected_false_positive_rate = 0.1;

    let size_of_bloom_filter = 100_000;

    let mut dict_bloom = BloomFilter::new_with_params(size_of_bloom_filter, expected_false_positive_rate);
    
    let words_iter = words.iter();
    
    let no_to_check = 20000;
    let no_to_add = 100000;

    let new_words = words_iter.clone().take(no_to_add);
    for new_word in new_words {
        dict_bloom.add(new_word);
    }

    let mut false_positives = 0;

    let check_words = words_iter.skip(no_to_add).take(no_to_check);
    for check_word in check_words {
        if dict_bloom.query(check_word) {
            false_positives += 1;
        }
    }

    println!("Bloom filter: {:?}", dict_bloom);
    println!("Found {:?}% false positives. Expected {:?}", false_positives as f32 * 100.0 / no_to_check as f32, expected_false_positive_rate);
}
