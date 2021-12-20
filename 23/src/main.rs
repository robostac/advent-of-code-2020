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

#[derive(Debug, Clone, Default)]
struct Link {
    before: usize,
    after: usize,
    value: usize,
}

impl Link {
    fn new(value: usize, before: usize, after: usize) -> Link {
        Link {
            before,
            after,
            value,
        }
    }

    fn blank() -> Link {
        Default::default()
    }
}

fn make_crab_move(cups: &mut Vec<Link>, cur_cup: usize) -> usize {
    let l1 = cups[cur_cup].after;
    let l2 = cups[l1].after;
    let l3 = cups[l2].after;

    let mut next = cur_cup - 1;
    loop {
        if next == 0 {
            next = cups.len() - 1;
        }
        if next == l1 || next == l2 || next == l3 {
            next -= 1;
        } else {
            break;
        }
    }

    //remove l1->l3 from after cur_cup
    cups[cur_cup].after = cups[l3].after;
    let after = cups[cur_cup].after;
    cups[after].before = cur_cup;

    //put l1-L3 after next
    cups[l3].after = cups[next].after;
    let after = cups[l3].after;
    cups[after].before = l3;

    cups[l1].before = next;
    cups[next].after = l1;

    //return the next cup in the circle
    cups[cur_cup].after
}

fn get_ans(cups: &Vec<Link>) -> String {
    let mut s = String::new();
    let mut idx = cups[1].after;
    while idx != 1 {
        s += &idx.to_string();
        idx = cups[idx].after;
    }
    s
}

fn get_ans2(cups: &Vec<Link>) -> i64 {
    let l1 = cups[1].after;
    let l2 = cups[l1].after;
    println!("{:?} {:?}", l1, l2);
    l1 as i64 * l2 as i64
}

fn print_cups(cups: &Vec<Link>, cur_cup: usize) {
    print!("{} ", cur_cup);
    let mut current = cups[cur_cup].after;
    while current != cur_cup {
        print!("{} ", current);
        current = cups[current].after;
    }
    println!();
}

fn main() {
    let stdin = io::stdin();
    let inp: Vec<_> = stdin.lock().lines().map(|input| input.unwrap()).collect();
    let values: Vec<_> = inp[0]
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect();

    let mut graph = vec![Link::blank(); 10];
    for i in 0..values.len() {
        let v = values[i];
        graph[v].value = v;
        graph[v].before = values[(values.len() + i - 1) % values.len()];
        graph[v].after = values[(values.len() + i + 1) % values.len()];
    }

    let mut test = graph.clone();
    // let max = *test.iter().max().unwrap();
    let mut cur_cup = values[0];
    for _ in 0..100 {
        cur_cup = make_crab_move(&mut test, cur_cup);
    }

    println!("{}", get_ans(&test));

    let mut test = graph.clone();

    const NUM_CUPS_PART_2: usize = 1000000;

    for i in 10..=NUM_CUPS_PART_2 {
        test.push(Link::new(i, i - 1, i + 1));
    }
    let last = *values.last().unwrap();
    test[last].after = 10;
    test[10].before = last;

    test[NUM_CUPS_PART_2].after = values[0];
    test[values[0]].before = NUM_CUPS_PART_2;

    let mut cur_cup = values[0];
    for _ in 0..10000000 {
        cur_cup = make_crab_move(&mut test, cur_cup);
    }
    println!("{}", get_ans2(&test));
}
