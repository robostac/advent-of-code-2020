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

fn get_deck(s: &str) -> VecDeque<i64> {
    s.split('\n')
        .skip(1)
        .map(|x| parse_input(x))
        .collect::<VecDeque<i64>>()
}

fn calc_score(v: &VecDeque<i64>) -> i64 {
    v.iter()
        .enumerate()
        .map(|(p, x)| ((v.len() - p) as i64) * x)
        .sum()
}

fn get_key(p1dec: &VecDeque<i64>, p2dec: &VecDeque<i64>) -> String {
    let v1: Vec<String> = p1dec.iter().map(|x| x.to_string()).collect();
    let v2: Vec<String> = p2dec.iter().map(|x| x.to_string()).collect();

    v1.join(",") + &v2.join(",")
}

fn recursive_combat(p1dec: &mut VecDeque<i64>, p2dec: &mut VecDeque<i64>) -> bool {
    let mut prev = HashSet::new();

    while prev.insert(get_key(&p1dec, &p2dec)) {
        if p1dec.is_empty() {
            return false;
        }
        if p2dec.is_empty() {
            return true;
        }
        let p1v = p1dec.pop_front().unwrap() as usize;
        let p2v = p2dec.pop_front().unwrap() as usize;
        let p1win;
        if p1dec.len() >= p1v && p2dec.len() >= p2v {
            let mut subdeck1 = (0..p1v).map(|x| p1dec[x]).collect::<VecDeque<i64>>();
            let mut subdeck2 = (0..p2v).map(|x| p2dec[x]).collect::<VecDeque<i64>>();
            p1win = recursive_combat(&mut subdeck1, &mut &mut subdeck2);
        } else {
            if p1v > p2v {
                p1win = true;
            } else {
                p1win = false;
            }
        }
        if p1win {
            p1dec.push_back(p1v as i64);
            p1dec.push_back(p2v as i64);
        } else {
            p2dec.push_back(p2v as i64);
            p2dec.push_back(p1v as i64);
        }
    }
    true
}

fn main() {
    let mut stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer);
    let values: Vec<String> = buffer.split("\n\n").map(|x| x.to_owned()).collect();
    let orig_p1dec = get_deck(&values[0]);
    let orig_p2dec = get_deck(&values[1]);
    let mut p1dec = orig_p1dec.clone();
    let mut p2dec = orig_p2dec.clone();
    println!("{:?} {:?}", p1dec, p2dec);
    while p1dec.is_empty() == false && p2dec.is_empty() == false {
        let p1v = p1dec.pop_front().unwrap();
        let p2v = p2dec.pop_front().unwrap();
        if p1v > p2v {
            p1dec.push_back(p1v);
            p1dec.push_back(p2v);
        } else {
            p2dec.push_back(p2v);
            p2dec.push_back(p1v);
        }
    }
    println!("{:?} {:?}", p1dec, p2dec);

    if p1dec.is_empty() {
        println!("{}", calc_score(&p2dec));
    } else {
        println!("{}", calc_score(&p1dec));
    }

    let mut p1dec = orig_p1dec.clone();
    let mut p2dec = orig_p2dec.clone();
    if recursive_combat(&mut p1dec, &mut p2dec) {
        println!("{}", calc_score(&p1dec));
    } else {
        println!("{}", calc_score(&p2dec));
    }
}
