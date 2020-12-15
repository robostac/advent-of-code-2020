#![allow(dead_code, unused_macros, unused_imports)]
#[macro_use]
extern crate text_io;
use std::collections::*;
use std::io;
use std::io::prelude::*;

fn apply_mask(v: u64, mask_set: u64, mask_clear: u64) -> u64 {
    (v | mask_set) & mask_clear
}

fn mask_from_str(s: &str) -> (u64, u64, u64) {
    let mut mask_ones = 0;
    let mut mask_zeroes = 0;
    let mut mask_x = 0;
    for (p, v) in s.chars().rev().enumerate() {
        if v == '1' {
            mask_ones |= 1 << p;
        } else if v == '0' {
            mask_zeroes |= 1 << p;
        } else {
            mask_x |= 1 << p;
        }
    }
    (mask_ones, mask_zeroes, mask_x)
}

fn do_insert(mask: &(u64, u64, u64), value: u64, mem: &mut HashMap<u64, u64>, start_addr: u64) {
    let mask_pos = mask.2;
    let masked_addr = (mask.0) | (start_addr & mask.1);
    recursive_insert(mask_pos, value, mem, masked_addr);
}

fn recursive_insert(mask_pos: u64, value: u64, mem: &mut HashMap<u64, u64>, cur_addr: u64) {
    if 0 == mask_pos {
        mem.insert(cur_addr, value);
        return;
    }
    let bi = 1 << mask_pos.trailing_zeros();
    let nmask = mask_pos & !bi;
    recursive_insert(nmask, value, mem, cur_addr);
    recursive_insert(nmask, value, mem, cur_addr | bi);
}

fn main() {
    let stdin = io::stdin();
    let inp: Vec<_> = stdin
        .lock()
        .lines()
        .map(|input| {
            let s = input.unwrap();
            let a: String;
            let b: String;
            scan!(s.bytes() => "{} = {}", a, b);
            (a, b)
        })
        .collect();
    let mut mask_set = 0;
    let mut mask_clear = 0;
    let mut mem = HashMap::new();
    for (inst, x) in inp.iter() {
        if inst == "mask" {
            let (a, b, _) = mask_from_str(x);
            mask_set = a;
            mask_clear = !b;
        } else {
            let c: u64;
            scan!(inst.bytes() => "mem[{}]", c);
            let r = x.parse::<u64>().unwrap();
            mem.insert(c, apply_mask(r, mask_set, mask_clear));
        }
    }
    println!("{:?}", mem.values().sum::<u64>());

    let mut mem = HashMap::new();
    let mut masks = Default::default();
    for (inst, x) in inp.iter() {
        if inst == "mask" {
            masks = mask_from_str(x);
        } else {
            let c: u64;
            scan!(inst.bytes() => "mem[{}]", c);
            let r = x.parse::<u64>().unwrap();
            do_insert(&masks, r, &mut mem, c);
        }
    }
    println!("{:?}", mem.values().sum::<u64>());
}
