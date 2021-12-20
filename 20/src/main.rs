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

#[derive(Debug, Clone, Default, PartialEq)]
struct ImageTile {
    data: Vec<Vec<bool>>,
    id: u64,
    match_count: u64,
}

struct ImageTileIter {
    it: ImageTile,
    pos: usize,
}

impl Iterator for ImageTileIter {
    type Item = ImageTile;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == 8 {
            return None;
        }
        self.it = self.it.rotate();
        if self.pos == 4 {
            self.it = self.it.flip_vert();
        }
        self.pos += 1;
        Some(self.it.clone())
    }
}

const SEA_MONSTER: [(usize, usize); 15] = [
    (0, 1),
    (1, 2),
    (4, 2),
    (5, 1),
    (6, 1),
    (7, 2),
    (10, 2),
    (11, 1),
    (12, 1),
    (13, 2),
    (16, 2),
    (17, 1),
    (18, 0),
    (18, 1),
    (19, 1),
];

impl ImageTile {
    fn iter(&self) -> ImageTileIter {
        ImageTileIter {
            it: self.clone(),
            pos: 0,
        }
    }
    fn left(&self) -> u64 {
        (0..self.data.len())
            .map(|c| if self.data[0][c] { 1 << c } else { 0 })
            .sum()
    }
    fn right(&self) -> u64 {
        (0..self.data.len())
            .map(|c| {
                if self.data[self.data.len() - 1][c] {
                    1 << c
                } else {
                    0
                }
            })
            .sum()
    }
    fn top(&self) -> u64 {
        (0..self.data.len())
            .map(|c| if self.data[c][0] { 1 << c } else { 0 })
            .sum()
    }
    fn bottom(&self) -> u64 {
        (0..self.data.len())
            .map(|c| {
                if self.data[c][self.data.len() - 1] {
                    1 << c
                } else {
                    0
                }
            })
            .sum()
    }

    fn rotate(&self) -> ImageTile {
        let mut it = self.clone();
        for x in 0..self.data.len() {
            for y in 0..self.data.len() {
                it.data[x][y] = self.data[y][self.data.len() - x - 1];
            }
        }
        it
    }

    fn flip_vert(&self) -> ImageTile {
        let mut it = self.clone();
        for x in 0..self.data.len() {
            for y in 0..self.data.len() {
                it.data[x][y] = self.data[x][self.data.len() - y - 1];
            }
        }
        it
    }

    fn flip_horiz(&self) -> ImageTile {
        let mut it = self.clone();
        for x in 0..self.data.len() {
            for y in 0..self.data.len() {
                it.data[x][y] = self.data[self.data.len() - x - 1][y];
            }
        }
        it
    }

    fn flip_both(&self) -> ImageTile {
        let mut it = self.clone();
        for x in 0..self.data.len() {
            for y in 0..self.data.len() {
                it.data[x][y] = self.data[self.data.len() - x - 1][self.data.len() - y - 1];
            }
        }
        it
    }

    fn new(input: &str) -> ImageTile {
        let mut it: ImageTile = Default::default();
        for (i, s) in input.split("\n").enumerate() {
            if i == 0 {
                let start = s.len() - 5;
                it.id = s[start..(start + 4)].parse::<u64>().unwrap();
            } else {
                let v = s.chars().map(|x| x == '#').collect();
                it.data.push(v);
            }
        }

        it
    }

    fn new_blank(s: usize) -> ImageTile {
        let mut it: ImageTile = Default::default();
        it.data = (0..s).map(|_| vec![false; s]).collect();
        it
    }

    fn is_seamonster(&self, x: usize, y: usize) -> bool {
        if x + 19 >= self.data.len() {
            return false;
        }
        if y + 2 >= self.data.len() {
            return false;
        }
        for p in SEA_MONSTER.iter() {
            if self.data[x + p.0][y + p.1] == false {
                return false;
            }
        }
        true
    }
}

fn find_tile(
    tiles: &Vec<ImageTile>,
    matched_edges: &HashSet<u64>,
    left: Option<u64>,
    top: Option<u64>,
    right: bool,
    bottom: bool,
) -> Option<ImageTile> {
    for t in tiles.iter() {
        for z in t.iter() {
            if right == false || matched_edges.contains(&z.right()) {
                if bottom == false || matched_edges.contains(&z.bottom()) {
                    if left.is_none() || z.left() == left.unwrap() {
                        if top.is_none() || z.top() == top.unwrap() {
                            return Some(z);
                        }
                    }
                }
            }
        }
    }
    None
}

fn main() {
    let mut stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer);
    let values: Vec<String> = buffer.split("\n\n").map(|x| x.to_owned()).collect();
    let it_input: Vec<_> = values.iter().map(|x| ImageTile::new(x)).collect();
    let mut test: HashMap<u64, u64> = HashMap::new();
    let mut it = Vec::new();
    let ntiles = it_input.len();
    let sqsize = (ntiles as f64).sqrt() as usize;

    for x in it_input.iter() {
        it.push(x.clone());
        it.push(x.flip_both());
    }
    //count how many times we see each edge
    for x in it.iter() {
        *test.entry(x.left()).or_default() += 1;
        *test.entry(x.right()).or_default() += 1;
        *test.entry(x.top()).or_default() += 1;
        *test.entry(x.bottom()).or_default() += 1;
    }
    //for every tile count how many matchable edges it has (edges that appear on more than 1 tile)
    let mut matched_edges = HashSet::new();
    for x in test.iter().filter(|y| *y.1 > 1) {
        for tile in it.iter_mut() {
            if tile.right() == *x.0
                || tile.left() == *x.0
                || tile.top() == *x.0
                || tile.bottom() == *x.0
            {
                tile.match_count += 1;
                matched_edges.insert(*x.0);
            }
        }
    }
    let corners: HashSet<_> = it
        .iter()
        .filter(|x| x.match_count == 2)
        .map(|x| x.id)
        .collect();

    println!("{}", corners.iter().product::<u64>());

    let mut tiles: Vec<Vec<ImageTile>> = (0..sqsize)
        .map(|_| vec![ImageTile::new_blank(0); sqsize])
        .collect();
    tiles[0][0] = it.iter().find(|x| corners.contains(&x.id)).unwrap().clone();
    //put any corner tile in top left
    for p in tiles[0][0].iter() {
        //find an orientation where the corner tile has matchable edges on bottom and right
        if matched_edges.contains(&p.right()) && matched_edges.contains(&p.bottom()) {
            tiles[0][0] = p;
            break;
        }
    }
    for x in 0..sqsize {
        for y in 0..sqsize {
            if x != 0 || y != 0 {
                //for every position we've already filled in above and left (when not on left or top edge) so we need to match those
                //if we aren't on bottom or right edge with need to make sure the bottom / right edges of this tile are matchable
                tiles[x][y] = find_tile(
                    &it,
                    &matched_edges,
                    if x == 0 {
                        None
                    } else {
                        Some(tiles[x - 1][y].right())
                    },
                    if y == 0 {
                        None
                    } else {
                        Some(tiles[x][y - 1].bottom())
                    },
                    x != sqsize - 1,
                    y != sqsize - 1,
                )
                .unwrap();
            }
            // remove this tile as otherwise we end up matching tiles against themselves
            it.retain(|p| p.id != tiles[x][y].id);
        }
    }

    //print layout if needed
    // for y in 0..sqsize {
    //     for x in 0..sqsize {
    //         print!("{} ", tiles[x][y].id);
    //     }
    //     println!();
    // }

    //build big image
    let image_dim = tiles[0][0].data.len() - 2;
    let canvas_size = image_dim * sqsize;
    let mut canvas = ImageTile::new_blank(canvas_size);
    for y in 0..sqsize {
        for x in 0..sqsize {
            let sx = x * image_dim;
            let sy = y * image_dim;
            for xx in 1..=image_dim {
                for yy in 1..=image_dim {
                    canvas.data[sx + xx - 1][sy + yy - 1] = tiles[x][y].data[xx][yy];
                }
            }
        }
    }
    for t in canvas.iter() {
        let mut count = 0;
        for x in 0..canvas_size {
            for y in 0..canvas_size {
                if t.is_seamonster(x, y) {
                    count += 1;
                }
            }
        }
        //if we have seamonsters we can calculate the answer
        if count > 0 {
            println!(
                "{:?}",
                t.data
                    .iter()
                    .map(|x| x.iter().filter(|y| **y).count())
                    .sum::<usize>()
                    - (count * 15)
            );
            break;
        }
    }
}
