#![allow(dead_code, unused_macros, unused_imports)]
#[macro_use]
extern crate text_io;
use std::collections::*;
use std::io;
use std::io::prelude::*;

#[derive(PartialEq, Debug, Eq, Copy, Clone)]
enum SeatStatus {
    Occupied,
    Empty,
    Floor,
    Wall,
}

#[derive(PartialEq, Debug, Eq, Copy, Clone, Hash)]
struct Point {
    x: i64,
    y: i64,
}

type GridLayout = HashMap<Point, SeatStatus>;

struct PointVisibleNeighbours<'a> {
    idx: usize,
    orig_point: Point,
    grid: &'a GridLayout,
    needs_vis: bool,
}

const DIRECTIONS: [Point; 8] = [
    Point::new(1, 0),
    Point::new(0, 1),
    Point::new(-1, 0),
    Point::new(0, -1),
    Point::new(1, 1),
    Point::new(-1, -1),
    Point::new(-1, 1),
    Point::new(1, -1),
];

impl Point {
    fn visible_neighbours<'a>(
        &self,
        grid: &'a GridLayout,
        needs_vis: bool,
    ) -> PointVisibleNeighbours<'a> {
        PointVisibleNeighbours::new(self, grid, needs_vis)
    }

    const fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }

    fn add(&self, other: &Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl<'a> PointVisibleNeighbours<'a> {
    fn new(p: &Point, g: &'a GridLayout, needs_vis: bool) -> PointVisibleNeighbours<'a> {
        PointVisibleNeighbours {
            idx: 0,
            orig_point: p.clone(),
            grid: g,
            needs_vis,
        }
    }
}

impl<'a> Iterator for PointVisibleNeighbours<'a> {
    type Item = Point;
    fn next(&mut self) -> Option<Point> {
        while self.idx < DIRECTIONS.len() {
            let dir = &DIRECTIONS[self.idx];
            self.idx += 1;
            let mut p = self.orig_point;
            loop {
                p = p.add(dir);
                let g = *self.grid.get(&p).unwrap_or(&SeatStatus::Wall);
                if g == SeatStatus::Wall || (g == SeatStatus::Floor && self.needs_vis == false) {
                    break;
                }
                if g != SeatStatus::Floor {
                    return Some(p);
                }
            }
        }
        None
    }
}

fn get_next_grid(grid: &mut GridLayout, needs_vis: bool, occ_count: i32) -> bool {
    let mut new_grid_work = HashMap::new();
    for (p, _) in grid.iter().filter(|(_, x)| **x == SeatStatus::Occupied) {
        for n in p.visible_neighbours(&grid, needs_vis) {
            *new_grid_work.entry(n).or_insert(0) += 1;
        }
    }
    update_grid(grid, &new_grid_work, occ_count) > 0
}

fn update_grid(grid: &mut GridLayout, grid_work: &HashMap<Point, i32>, occ_count: i32) -> i32 {
    let mut changes = 0;
    for (p, v) in grid.iter_mut() {
        let count = *grid_work.get(p).unwrap_or(&0);
        if *v == SeatStatus::Occupied && count >= occ_count {
            *v = SeatStatus::Empty;
            changes += 1;
        } else if *v == SeatStatus::Empty && count == 0 {
            *v = SeatStatus::Occupied;
            changes += 1;
        }
    }
    changes
}

fn print(grid: &GridLayout) {
    let max_x = *grid.keys().max_by_key(|x| x.x).unwrap();
    let max_y = *grid.keys().max_by_key(|x| x.y).unwrap();
    for y in 0..=max_y.y {
        for x in 0..=max_x.x {
            let p = Point::new(x, y);
            match grid.get(&p).unwrap() {
                SeatStatus::Occupied => print!("#"),
                SeatStatus::Empty => print!("L"),
                SeatStatus::Wall => print!("|"),
                SeatStatus::Floor => print!(" "),
            }
        }
        println!();
    }
    println!();
}

fn stabilize(orig_grid: &GridLayout, needs_vis: bool, occ_count: i32) -> GridLayout {
    let mut grid = orig_grid.clone();
    while get_next_grid(&mut grid, needs_vis, occ_count) {}
    grid
}

fn main() {
    let stdin = io::stdin();
    let inp: Vec<String> = stdin.lock().lines().map(|input| input.unwrap()).collect();
    let mut orig_grid = HashMap::new();
    for (y, z) in inp.iter().enumerate() {
        for (x, v) in z.chars().enumerate() {
            let p = Point::new(x as i64, y as i64);
            if v == 'L' {
                orig_grid.insert(p, SeatStatus::Empty);
            } else {
                orig_grid.insert(p, SeatStatus::Floor);
            }
        }
    }
    let grid = stabilize(&orig_grid, false, 4);
    println!(
        "{}",
        grid.values()
            .filter(|x| **x == SeatStatus::Occupied)
            .count()
    );
    let grid = stabilize(&orig_grid, true, 5);
    println!(
        "{}",
        grid.values()
            .filter(|x| **x == SeatStatus::Occupied)
            .count()
    );
}
