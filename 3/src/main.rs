#![allow(dead_code, unused_macros, unused_imports)]
#[macro_use]
extern crate text_io;
use std::collections::*;
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let values: Vec<String> = stdin.lock().lines().map(|input| input.unwrap()).collect();
    let mut grid = HashMap::new();
    for (y, i) in values.iter().enumerate() {
        for (x, v) in i.chars().enumerate() {
            grid.insert((x, y), v == '#');
        }
    }
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut ans = Vec::new();
    for slope in slopes.iter() {
        let width = values[0].len();
        let height = values.len();
        let mut count = 0;
        let mut x = 0;
        let mut y = 0;
        while y < height {
            if *grid.get(&(x, y)).unwrap() {
                count += 1;
            }
            y = y + slope.1;
            x = (x + slope.0) % width;
        }
        ans.push(count);
    }
    let mut second_part: i64 = 1;
    for x in ans.iter() {
        second_part *= x;
    }
    println!("{} {}", ans[1], second_part);
}
