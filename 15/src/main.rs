#![allow(dead_code, unused_macros, unused_imports)]
#[macro_use]
extern crate text_io;
use std::collections::*;
use std::io;
use std::io::prelude::*;

fn get_next(last: i64, memory: &mut HashMap<i64, i64>, turn: i64) -> i64 {
    let z = memory.insert(last, turn);
    if z.is_none() {
        0
    } else {
        turn - z.unwrap()
    }
}

fn main() {
    let stdin = io::stdin();
    let inp: Vec<_> = stdin
        .lock()
        .lines()
        .map(|input| {
            let s = input.unwrap();
            s
        })
        .collect();
    let mut memory = HashMap::new();
    let mut turn = 1;
    for x in inp[0].split(",") {
        let x = x.parse::<i64>().unwrap();
        memory.insert(x, turn);
        turn += 1;
    }
    let mut last = 0;
    while turn < 2020 {
        last = get_next(last, &mut memory, turn);
        turn += 1;
    }
    println!("P1: {}", last);

    while turn < 30000000 {
        last = get_next(last, &mut memory, turn);
        turn += 1;
    }
    println!("P2: {}", last);
}
