#![allow(dead_code, unused_macros, unused_imports)]
#[macro_use]
extern crate text_io;
use std::collections::*;
use std::io;
use std::io::prelude::*;

fn pair(values: &HashSet<i64>, expected: i64) -> Option<i64> {
    for x in values.iter() {
        let rem = expected - *x;
        if values.contains(&rem) {
            return Some(rem * x);
        }
    }
    return None;
}

fn triple(values: &HashSet<i64>, expected: i64) -> Option<i64> {
    for x in values.iter() {
        match pair(values, expected - *x) {
            None => {}
            Some(v) => {
                return Some(x * v);
            }
        }
    }
    return None;
}

fn main() {
    let stdin = io::stdin();
    let values: HashSet<i64> = stdin
        .lock()
        .lines()
        .map(|input| {
            let x: i64;
            let s = input.unwrap();
            scan!(s.bytes() => "{}", x);
            x
        })
        .collect();
    println!("{}", pair(&values, 2020).unwrap());
    println!("{}", triple(&values, 2020).unwrap());
}
