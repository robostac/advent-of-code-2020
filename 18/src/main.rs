#![allow(dead_code, unused_macros, unused_imports)]
#[macro_use]
extern crate text_io;
use std::collections::*;
use std::io;
use std::io::prelude::*;

fn has_precedence(p2: bool, top: char, cur: char) -> bool {
    if top == '(' || cur == '(' {
        return false;
    }
    if p2 && top == '*' && cur == '+' {
        return false;
    }
    true
}

fn shunt(s: &str, p2: bool) -> i64 {
    let mut op = Vec::new();
    let mut output = Vec::new();
    let mut cur = String::new();
    for p in s.chars() {
        if ('0'..='9').contains(&p) {
            cur.push(p);
        } else {
            if cur != "" {
                output.push(cur.to_owned());
                cur = String::new();
            }
            if p != ' ' {
                while has_precedence(p2, *op.last().unwrap_or(&'('), p) {
                    output.push(op.pop().unwrap().to_string());
                }
                if p == ')' {
                    op.pop();
                } else {
                    op.push(p);
                }
            }
        }
    }
    while op.is_empty() == false {
        output.push(op.pop().unwrap().to_string());
    }
    calc(&mut output)
}

fn calc(out: &mut Vec<String>) -> i64 {
    let v = out.pop().unwrap();
    if v == "+" {
        calc(out) + calc(out)
    } else if v == "*" {
        calc(out) * calc(out)
    } else {
        v.parse::<i64>().unwrap()
    }
}

fn main() {
    let stdin = io::stdin();
    let inp: Vec<_> = stdin
        .lock()
        .lines()
        .map(|input| {
            let s = input.unwrap();
            s + " "
        })
        .collect();

    let p1: i64 = inp.iter().map(|x| shunt(x, false)).sum();
    println!("{}", p1);
    let p2: i64 = inp.iter().map(|x| shunt(x, true)).sum();
    println!("{}", p2);
}
