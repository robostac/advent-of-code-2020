#![allow(dead_code, unused_macros, unused_imports)]
#[macro_use]
extern crate text_io;
use std::collections::*;
use std::io;
use std::io::prelude::*;

#[derive(PartialEq, Debug, Eq, Copy, Clone, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    const fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }

    fn add(&self, other: &Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }

    fn add_multiple(&self, other: &Point, count: i64) -> Point {
        Point::new(self.x + count * other.x, self.y + count * other.y)
    }

    fn from_str(s: &str) -> Point {
        match s {
            "N" => return Point::new(0, -1),
            "S" => return Point::new(0, 1),
            "W" => return Point::new(-1, 0),
            "E" => return Point::new(1, 0),
            _ => panic!("Unknown facing str {}", s),
        }
    }

    fn rotate_anticlockwise(&self) -> Point {
        Point::new(self.y, -self.x)
    }
    fn rotate_clockwise(&self) -> Point {
        Point::new(-self.y, self.x)
    }
}

fn main() {
    let stdin = io::stdin();
    let inp: Vec<(String, i64)> = stdin
        .lock()
        .lines()
        .map(|input| {
            let v = input.unwrap();
            let (d, x) = v.split_at(1);
            let mut x = x.parse::<i64>().unwrap();
            if d == "L" || d == "R" {
                if x % 90 != 0 {
                    panic!("Unsupported Rotation, {} {:?}", d, x);
                }
                x = (x / 90) % 4;
            }
            (d.to_owned(), x)
        })
        .collect();

    let mut cur_facing = Point::from_str("E");
    let mut cur_pos = Point::new(0, 0);
    for x in inp.iter() {
        match x.0.as_str() {
            "F" => cur_pos = cur_pos.add_multiple(&cur_facing, x.1),
            "L" => (0..x.1).for_each(|_| cur_facing = cur_facing.rotate_anticlockwise()),
            "R" => (0..x.1).for_each(|_| cur_facing = cur_facing.rotate_clockwise()),
            p => cur_pos = cur_pos.add_multiple(&Point::from_str(p), x.1),
        }
    }
    println!("{:?}", cur_pos.x.abs() + cur_pos.y.abs());

    let mut cur_pos = Point::new(0, 0);
    let mut cur_wp = Point::new(10, -1);
    for x in inp.iter() {
        match x.0.as_str() {
            "F" => cur_pos = cur_pos.add_multiple(&cur_wp, x.1),
            "L" => (0..x.1).for_each(|_| cur_wp = cur_wp.rotate_anticlockwise()),
            "R" => (0..x.1).for_each(|_| cur_wp = cur_wp.rotate_clockwise()),
            p => cur_wp = cur_wp.add_multiple(&Point::from_str(p), x.1),
        }
    }
    println!("{:?}", cur_pos.x.abs() + cur_pos.y.abs());
}
