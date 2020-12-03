#![allow(dead_code, unused_macros, unused_imports)]
#[macro_use]
extern crate text_io;
use std::collections::*;
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let values: Vec<(usize, usize, char, String)> = stdin
        .lock()
        .lines()
        .map(|input| {
            let min: usize;
            let max: usize;
            let c: char;
            let d: String;
            let s = input.unwrap();
            scan!(s.bytes() => "{}-{} {}: {}", min,max,c,d);
            (min, max, c, d)
        })
        .collect();

    let valid = values
        .iter()
        .filter(|(min, max, c, d)| {
            let n = d.chars().filter(|dc| dc == c).count();
            n >= *min && n <= *max
        })
        .count();
    println!("{}", valid);

    let valid = values
        .iter()
        .filter(|(min, max, c, d)| {
            let s = d.as_bytes();
            let c = *c as u8;
            (s[(*min - 1)] == c) != (s[(*max - 1)] == c)
        })
        .count();
    println!("{}", valid);
}
