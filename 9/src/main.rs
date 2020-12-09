#![allow(dead_code, unused_macros, unused_imports)]
#[macro_use]
extern crate text_io;
use std::collections::*;
use std::io;
use std::io::prelude::*;

fn is_in_pream(nums: &Vec<i64>, pos: usize, pream: usize) -> bool {
    for i in (pos - pream)..pos {
        for j in (i + 1)..pos {
            if nums[i] + nums[j] == nums[pos] {
                return true;
            }
        }
    }
    return false;
}

fn find_contig(nums: &Vec<i64>, tgt: i64) -> (usize, usize) {
    let mut start = 0;
    let mut end;
    let mut val = 0;
    for (p, x) in nums.iter().enumerate() {
        val += x;
        end = p;
        while val >= tgt {
            if val == tgt {
                return (start, end);
            }
            val -= nums[start];
            start += 1;
        }
    }
    return (0, 0);
}

fn main() {
    let stdin = io::stdin();
    let nums: Vec<i64> = stdin
        .lock()
        .lines()
        .map(|input| {
            let input = input.unwrap();
            input.parse::<i64>().unwrap()
        })
        .collect();

    let pream = 25;
    let mut tgt = 0;
    for x in pream..nums.len() {
        if is_in_pream(&nums, x, pream) == false {
            tgt = nums[x];
        }
    }
    println!("{}", tgt);
    let (s, e) = find_contig(&nums, tgt);
    let min_weak = nums[s..=e].iter().min().unwrap();
    let max_weak = nums[s..=e].iter().max().unwrap();
    println!("{}", min_weak + max_weak);
}
