#![allow(dead_code, unused_macros, unused_imports)]
#[macro_use]
extern crate text_io;
use std::collections::*;
use std::io;
use std::io::prelude::*;

fn valid_height(s: &str) -> bool {
    if s.len() <= 2 {
        return false;
    }
    let (amount, unit) = s.split_at(s.len() - 2);
    let amount: i64 = amount.parse().unwrap_or(0);
    if unit == "in" {
        (59..=76).contains(&amount)
    } else if unit == "cm" {
        (150..=193).contains(&amount)
    } else {
        false
    }
}

fn valid_hcl(s: &str) -> bool {
    if s.len() != 7 {
        return false;
    }
    for (p, x) in s.chars().enumerate() {
        if p == 0 && x != '#' {
            return false;
        } else if p > 0 && "1234567890abcdef".contains(x) == false {
            return false;
        }
    }
    true
}

fn valid_ecl(s: &str) -> bool {
    let poss = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    poss.contains(&s)
}

fn valid_pid(s: &str) -> bool {
    if s.len() != 9 {
        return false;
    }
    let amount: i64 = s.parse().unwrap_or(-1);
    amount >= 0
}

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
            for z in x.split(" ") {
                let m = z.split(":").collect::<Vec<&str>>();
                p.insert(m[0], m[1]);
            }
        }
    }
    let req = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let valid = pp
        .iter()
        .filter(|p| req.iter().all(|x| p.contains_key(x)))
        .count();
    println!("{:?}", valid);

    let valid = pp
        .iter()
        .filter(|p| {
            req.iter().all(|x| p.contains_key(x))
                && (1920..=2002).contains(&p["byr"].parse::<i64>().unwrap_or(0))
                && (2010..=2020).contains(&p["iyr"].parse::<i64>().unwrap_or(0))
                && (2020..=2030).contains(&p["eyr"].parse::<i64>().unwrap_or(0))
                && valid_height(p["hgt"])
                && valid_hcl(p["hcl"])
                && valid_ecl(p["ecl"])
                && valid_pid(p["pid"])
        })
        .count();
    println!("{:?}", valid);
}
