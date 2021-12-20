#![allow(dead_code, unused_macros, unused_imports)]

use std::collections::*;
use std::io;
use std::io::prelude::*;

use std::fmt::Debug;
use std::str::FromStr;

fn parse_input<Input, Type>(input: Input) -> Type
where
    Type: FromStr,
    <Type as FromStr>::Err: Debug,
    Input: AsRef<str>,
{
    input.as_ref().trim().parse().unwrap()
}

fn loop_val(input: u64, subject: u64) -> u64 {
    (input * subject) % 20201227
}

fn find_loops(public_key: u64) -> usize {
    let mut start = 1;
    for i in 0.. {
        if start == public_key {
            return i;
        }
        start = loop_val(start, 7);
    }
    0
}

fn calc_key(public_key: u64, other_loops: usize) -> u64 {
    let mut start = 1;
    for _ in 0..other_loops {
        start = loop_val(start, public_key);
    }
    start
}

fn main() {
    let stdin = io::stdin();
    let inp: Vec<u64> = stdin
        .lock()
        .lines()
        .map(|input| parse_input(&input.unwrap()))
        .collect();

    // let l1 = find_loops(inp[0]);
    let l2 = find_loops(inp[1]);
    // println!("{} {}", l1, l2);

    println!("{}", calc_key(inp[0], l2));
}
