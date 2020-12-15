#![allow(dead_code, unused_macros, unused_imports)]
#[macro_use]
extern crate text_io;
use std::collections::*;
use std::io;
use std::io::prelude::*;

fn gcd(x: i64, y: i64) -> i64 {
    if y > 0 {
        return gcd(y, x % y);
    }
    return x;
}

fn lcm(x: i64, y: i64) -> i64 {
    return (x * y) / gcd(x, y);
}

fn main() {
    let stdin = io::stdin();
    let inp: Vec<_> = stdin.lock().lines().map(|input| input.unwrap()).collect();
    let ids: Vec<_> = inp[1]
        .split(",")
        .enumerate()
        .filter(|x| x.1 != "x")
        .map(|(p, x)| (x.parse::<i64>().unwrap(), p as i64))
        .collect();
    let time = inp[0].parse::<i64>().unwrap();
    let delays = ids
        .iter()
        .map(|(x, _)| (x, x - (time % x)))
        .collect::<Vec<_>>();
    let first = delays.iter().min_by_key(|x| x.1).unwrap();
    println!("{:?}", first.1 * first.0);

    let mut cur_cycle = 1;
    let mut cur_start: i64 = 0;
    for p in ids.iter() {
        cur_start = (cur_start..)
            .step_by(cur_cycle as usize)
            .find(|x| (x + p.1) % p.0 == 0)
            .unwrap();
        cur_cycle = lcm(cur_cycle, p.0);
    }
    println!("{}", cur_start);
}
