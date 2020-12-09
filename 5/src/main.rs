#![allow(dead_code, unused_macros, unused_imports)]
#[macro_use]
extern crate text_io;
use std::collections::*;
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let values: HashMap<u64, (u64, u64)> = stdin
        .lock()
        .lines()
        .map(|input| {
            let v = input.unwrap();
            let (r, c) = v.split_at(7);
            let r = r.replace("B", "1").replace("F", "0");
            let c = c.replace("R", "1").replace("L", "0");
            let r = u64::from_str_radix(&r, 2).unwrap();
            let c = u64::from_str_radix(&c, 2).unwrap();
            let si = r * 8 + c;
            (si, (r, c))
        })
        .collect();
    let minseat = values.keys().min().unwrap();
    let maxseat = values.keys().max().unwrap();
    println!("{:?}", maxseat);
    for x in *minseat..=*maxseat {
        if values.contains_key(&x) == false {
            println!("{}", x);
            break;
        }
    }
}
