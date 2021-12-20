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

#[derive(Debug, Clone)]
enum Direction {
    West,
    East,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl Direction {
    fn as_point(&self) -> (i32, i32) {
        match self {
            Direction::East => (2, 0),
            Direction::West => (-2, 0),
            Direction::NorthEast => (1, -1),
            Direction::NorthWest => (-1, -1),
            Direction::SouthEast => (1, 1),
            Direction::SouthWest => (-1, 1),
        }
    }
}

/* HEX GRID

 (      (1,-1)
   (0,0)  (2,0)


*/

fn to_directions(s: &str) -> Vec<Direction> {
    let c: Vec<char> = s.chars().collect();
    let mut directions = Vec::new();
    let mut offset = 0;
    while offset < c.len() {
        if c[offset] == 'e' {
            directions.push(Direction::East);
        } else if c[offset] == 'w' {
            directions.push(Direction::West);
        } else if c[offset] == 'n' {
            offset += 1;
            if c[offset] == 'e' {
                directions.push(Direction::NorthEast);
            } else if c[offset] == 'w' {
                directions.push(Direction::NorthWest);
            }
        } else if c[offset] == 's' {
            offset += 1;
            if c[offset] == 'e' {
                directions.push(Direction::SouthEast);
            } else if c[offset] == 'w' {
                directions.push(Direction::SouthWest);
            }
        }
        offset += 1;
    }
    directions
}

struct NeighbourIter {
    p: (i32, i32),
    idx: usize,
}

impl Iterator for NeighbourIter {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == 6 {
            return None;
        }
        let dir = &[
            Direction::East,
            Direction::West,
            Direction::SouthEast,
            Direction::SouthWest,
            Direction::NorthEast,
            Direction::NorthWest,
        ][self.idx];
        self.idx += 1;
        let pp = dir.as_point();
        Some((self.p.0 + pp.0, self.p.1 + pp.1))
    }
}

fn neighbours(p: (i32, i32)) -> NeighbourIter {
    NeighbourIter { p, idx: 0 }
}

fn living_tiles(tiles: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let mut blank_tiles_to_check = HashSet::new();
    let mut new_tiles = HashSet::new();
    for t in tiles.iter() {
        let mut c = 0;
        for neighbour in neighbours(*t) {
            if tiles.contains(&neighbour) {
                c += 1;
            } else {
                blank_tiles_to_check.insert(neighbour);
            }
        }
        if c == 1 || c == 2 {
            new_tiles.insert(*t);
        }
    }

    for t in blank_tiles_to_check.iter() {
        let mut c = 0;
        for neighbour in neighbours(*t) {
            if tiles.contains(&neighbour) {
                c += 1;
            }
        }
        if c == 2 {
            new_tiles.insert(*t);
        }
    }

    new_tiles
}

fn main() {
    let stdin = io::stdin();
    let inp: Vec<_> = stdin
        .lock()
        .lines()
        .map(|input| to_directions(&input.unwrap()))
        .collect();
    let mut tiles = HashSet::new();

    for dir in inp.iter() {
        let mut pos = (0, 0);
        for d in dir {
            let offset = d.as_point();
            pos = (pos.0 + offset.0, pos.1 + offset.1);
        }
        if tiles.insert(pos) == false {
            tiles.remove(&pos);
        }
    }
    println!("{:?}", tiles.len());

    for _ in 0..100 {
        tiles = living_tiles(&tiles);
    }
    println!("{:?}", tiles.len());
}
