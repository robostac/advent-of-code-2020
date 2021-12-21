#![allow(dead_code, unused_macros, unused_imports)]
#[macro_use]
extern crate text_io;
use std::io;
use std::io::prelude::*;
use std::{collections::*, iter::Zip};

#[derive(PartialEq, Debug, Eq, Copy, Clone, Hash)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
}

type GridLayout = HashMap<Point, bool>;

struct PointVisibleNeighbours<'a> {
    idx: usize,
    orig_point: Point,
    directions: &'a Vec<Point>,
}

impl Point {
    fn visible_neighbours<'a>(&self, dir: &'a Vec<Point>) -> PointVisibleNeighbours<'a> {
        PointVisibleNeighbours::new(self, dir)
    }

    const fn new(x: i64, y: i64, z: i64, w: i64) -> Point {
        Point { x, y, z, w }
    }

    fn add(&self, other: &Point) -> Point {
        Point::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        )
    }

    fn is_zero(&self) -> bool {
        self.x == 0 && self.y == 0 && self.z == 0 && self.w == 0
    }
}

impl<'a> PointVisibleNeighbours<'a> {
    fn new(p: &Point, dir: &'a Vec<Point>) -> PointVisibleNeighbours<'a> {
        PointVisibleNeighbours {
            idx: 0,
            orig_point: p.clone(),
            directions: dir,
        }
    }
}

impl<'a> Iterator for PointVisibleNeighbours<'a> {
    type Item = Point;
    fn next(&mut self) -> Option<Point> {
        while self.idx < self.directions.len() {
            let dir = &self.directions[self.idx];
            self.idx += 1;
            return Some(self.orig_point.add(dir));
        }
        None
    }
}

fn get_next_grid(grid: &GridLayout, dir: &Vec<Point>) -> GridLayout {
    let mut new_grid_work = HashMap::new();
    for (p, _) in grid.iter() {
        for n in p.visible_neighbours(dir) {
            *new_grid_work.entry(n).or_insert(0) += 1;
        }
    }
    update_grid(grid, &new_grid_work)
}

fn update_grid(grid: &GridLayout, grid_work: &HashMap<Point, i32>) -> GridLayout {
    let mut new_grid = GridLayout::new();
    for (p, count) in grid_work.iter().filter(|(_, v)| **v == 2 || **v == 3) {
        if *count == 2 && *grid.get(p).unwrap_or(&false) {
            new_grid.insert(*p, true);
        } else if *count == 3 {
            new_grid.insert(*p, true);
        }
    }

    new_grid
}

fn get_directions(dim_count: usize) -> Vec<Point> {
    let mut points = vec![Point::new(0, 0, 0, 0); 81];
    for (i, p) in points.iter_mut().enumerate() {
        let i = i as i64;
        p.x = ((i) % 3) - 1;
        p.y = ((i / 3) % 3) - 1;
        p.z = ((i / 9) % 3) - 1;
        p.w = ((i / 27) % 3) - 1;
    }
    points.retain(|x| x.is_zero() == false && (dim_count == 4 || x.w == 0));
    points
}

fn stabilize(orig_grid: &GridLayout, boot_count: i32, dim_count: usize) -> GridLayout {
    let dir = get_directions(dim_count);
    let mut grid = orig_grid.clone();
    for _ in 0..boot_count {
        grid = get_next_grid(&grid, &dir);
    }
    grid
}

fn main() {
    let stdin = io::stdin();
    let inp: Vec<String> = stdin.lock().lines().map(|input| input.unwrap()).collect();
    let mut orig_grid = HashMap::new();
    for (y, z) in inp.iter().enumerate() {
        for (x, v) in z.chars().enumerate() {
            let p = Point::new(x as i64, y as i64, 0, 0);
            if v == '#' {
                orig_grid.insert(p, true);
            }
        }
    }
    let grid = stabilize(&orig_grid, 6, 3);
    println!("{}", grid.values().filter(|x| **x).count());
    let grid = stabilize(&orig_grid, 6, 4);
    println!("{}", grid.values().filter(|x| **x).count());
}
