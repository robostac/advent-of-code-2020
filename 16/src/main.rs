#![allow(dead_code, unused_macros, unused_imports)]
#[macro_use]
extern crate text_io;
use std::collections::*;
use std::io;
use std::io::prelude::*;

fn is_valid(v: i64, limits: &[(i64, i64)]) -> bool {
    limits.iter().any(|x| (x.0..=x.1).contains(&v))
}

fn main() {
    let stdin = io::stdin();
    let inp: Vec<_> = stdin
        .lock()
        .lines()
        .map(|input| {
            let s = input.unwrap();
            s
        })
        .collect();
    let mut limitations = HashMap::new();
    let mut my_ticket = Vec::new();
    let mut other_tickets = Vec::new();
    let mut stage = 0;
    for x in inp.iter() {
        if x == "" {
            stage += 1;
            continue;
        } else if stage == 0 {
            let name: String;
            let (a, b, c, d): (i64, i64, i64, i64);
            scan!(x.bytes() => "{}: {}-{} or {}-{}", name,a,b,c,d);
            limitations.insert(name, [(a, b), (c, d)]);
        } else if x.chars().last().unwrap() == ':' {
            continue;
        } else {
            let v = x
                .split(",")
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            if stage == 1 {
                my_ticket = v;
            } else {
                other_tickets.push(v);
            }
        }
    }
    let mut invalid_sum = 0;
    let mut valid_tickets = Vec::new();
    for x in other_tickets {
        let mut this_valid = true;
        for z in x.iter() {
            if limitations.values().any(|y| is_valid(*z, y)) == false {
                invalid_sum += z;
                this_valid = false;
            }
        }
        if this_valid {
            valid_tickets.push(x)
        }
    }
    println!("{}", invalid_sum);
    let mut possible = vec![limitations.keys().collect::<Vec<_>>(); limitations.len()];
    for tick in valid_tickets {
        for (p, v) in tick.iter().enumerate() {
            possible[p].retain(|val| is_valid(*v, &limitations[*val]));
        }
    }
    let mut singles = HashSet::new();
    while singles.len() != limitations.len() {
        possible.iter_mut().for_each(|p| {
            if p.len() > 1 {
                p.retain(|k| singles.contains(k) == false);
            } else {
                singles.insert(p.first().unwrap().to_owned());
            }
        });
    }
    let mut ans = 1;
    for (i, p) in possible.iter().enumerate() {
        if p.first().unwrap().starts_with("departure") {
            ans *= my_ticket[i];
        }
    }
    println!("{}", ans);
}
