#![allow(dead_code, unused_macros, unused_imports)]
#[macro_use]
extern crate text_io;
use std::collections::*;
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let values: Vec<String> = stdin.lock().lines().map(|input| input.unwrap()).collect();
    let mut pp = Vec::new();
    let mut p = HashMap::new();
    for x in values.iter() {
        if x == "" {
            pp.push(p);
            p = HashMap::new();
        } else {
            *p.entry('!').or_insert(0) += 1;
            for z in x.chars() {
                *p.entry(z).or_insert(0) += 1;
            }
        }
    }
    pp.push(p);
    let pa: usize = pp.iter().map(|x| x.len() - 1).sum();
    println!("{}", pa);
    let pb: usize = pp
        .iter()
        .map(|x| {
            let req = x[&'!'];
            x.iter()
                .filter(|(k, _)| **k != '!')
                .map(|(_, v)| if *v == req { 1 } else { 0 })
                .sum::<usize>()
        })
        .sum();
    println!("{}", pb);
}
