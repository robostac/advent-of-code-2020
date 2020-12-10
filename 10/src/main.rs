#![allow(dead_code, unused_macros, unused_imports)]
#[macro_use]
extern crate text_io;
use std::collections::*;
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut nums: Vec<i64> = stdin
        .lock()
        .lines()
        .map(|input| {
            let input = input.unwrap();
            input.parse::<i64>().unwrap()
        })
        .collect();
    nums.push(0);
    nums.sort();
    nums.push(nums.last().unwrap() + 3);

    let p1 = nums.windows(2).map(|p| p[1] - p[0]).collect::<Vec<i64>>();
    let ones = p1.iter().filter(|x| **x == 1).count();
    let threes = p1.iter().filter(|x| **x == 3).count();
    println!("{}", ones * threes);
    let mut memo = HashMap::new();
    memo.insert(*nums.last().unwrap(), 1);
    for &x in nums.iter().rev().skip(1) {
        let v = (x + 1..=x + 3)
            .map(|n| *memo.entry(n).or_default())
            .sum::<i64>();
        memo.insert(x, v);
    }
    println!("{}", memo[&0]);
}
