#![feature(test)]

extern crate aho_corasick;
extern crate test;
extern crate wu_manber;

#[macro_use]
extern crate lazy_static;

use aho_corasick::{Automaton, AcAutomaton as AC};
use std::cell::Cell;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::str;
use test::Bencher;
use wu_manber::TwoByteWM as WM;

const INPUT: &'static str = "benches/pg3200.txt";

fn split(s: String) -> Box<Iterator<Item=String>> {
    let words_vec: Vec<_> = s.split_whitespace().map(|s| s.to_string()).collect();
    Box::new(words_vec.into_iter())
}

fn get_words<Pred>(mut f: Pred, count: usize) -> Vec<String>
        where Pred: FnMut(&str) -> bool {
    let file = BufReader::new(File::open(INPUT).unwrap());

    // Skip over the header.
    let lines = file.lines().skip(1000);
    let words = lines.flat_map(|l| split(l.unwrap()));
    words.filter(|w| f(&w)).take(count).collect()
}

lazy_static!{
    static ref WORDS_3: Vec<String> = get_words(|w| w.len() == 3, 100);
    static ref WORDS_4: Vec<String> = get_words(|w| w.len() == 4, 100);
    static ref WORDS_5: Vec<String> = get_words(|w| w.len() == 5, 100);
    static ref WORDS_6: Vec<String> = get_words(|w| w.len() == 6, 100);
    static ref WORDS_7: Vec<String> = get_words(|w| w.len() == 7, 100);
    static ref WORDS_8: Vec<String> = get_words(|w| w.len() >= 8, 100);

    static ref HAYSTACK: String = {
        let mut file = File::open(INPUT).unwrap();
        let mut string = String::new();
        file.read_to_string(&mut string).unwrap();
        string
    };
}

macro_rules! twain(
    ($name:ident, $constructor:ident, $needles:ident, $needle_count:expr) => (
        #[bench]
        fn $name(b: &mut Bencher) {
            let needles = $constructor::new($needles.iter().take($needle_count));
            b.bytes = haystack.len() as u64;
            b.iter(|| needles.find(&HAYSTACK).count());
        }
    );
);

twain!(twain_wm_3_10, WM, WORDS_3, 10);
twain!(twain_wm_4_10, WM, WORDS_4, 10);
twain!(twain_wm_5_10, WM, WORDS_5, 10);
twain!(twain_wm_6_10, WM, WORDS_6, 10);
twain!(twain_wm_7_10, WM, WORDS_7, 10);
twain!(twain_wm_8_10, WM, WORDS_8, 10);

twain!(twain_ac_3_10, AC, WORDS_3, 10);
twain!(twain_ac_4_10, AC, WORDS_4, 10);
twain!(twain_ac_5_10, AC, WORDS_5, 10);
twain!(twain_ac_6_10, AC, WORDS_6, 10);
twain!(twain_ac_7_10, AC, WORDS_7, 10);
twain!(twain_ac_8_10, AC, WORDS_8, 10);
