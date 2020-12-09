#![allow(dead_code, unused_macros, unused_imports)]
#[macro_use]
extern crate text_io;
use std::collections::*;
use std::io;
use std::io::prelude::*;

#[derive(Debug, Default)]
struct Bag {
    contains: HashMap<String, usize>,
}

impl Bag {
    fn new() -> Bag {
        Default::default()
    }
}

fn inside(bags: &HashMap<String, Bag>, name: &str) -> usize {
    bags[name]
        .contains
        .iter()
        .map(|(k, v)| v * inside(bags, k))
        .sum::<usize>()
        + 1
}

fn main() {
    let stdin = io::stdin();
    let bags: HashMap<String, Bag> = stdin
        .lock()
        .lines()
        .map(|input| {
            let input = input.unwrap();
            let s: Vec<&str> = input.split(" ").collect();
            let btype = s[0].to_owned() + " " + s[1];
            let mut bb = Bag::new();
            for idx in (4..s.len()).step_by(4) {
                if s[idx] == "no" {
                    continue;
                }
                let count = s[idx].parse::<usize>().unwrap();
                let tt = s[idx + 1].to_owned() + " " + s[idx + 2];
                bb.contains.insert(tt, count);
            }
            (btype.to_owned(), bb)
        })
        .collect();
    let mybag = "shiny gold";
    let mut test = vec![mybag];
    let mut containers = HashSet::new();
    while test.len() > 0 {
        let bname = test.pop().unwrap();
        let contains = bags.iter().filter(|(_, v)| v.contains.contains_key(bname));
        for x in contains {
            if containers.insert(x.0) {
                test.push(x.0);
            }
        }
    }
    println!("{}", containers.len());
    println!("{}", inside(&bags, mybag) - 1);
}
