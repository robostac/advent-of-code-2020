#![allow(dead_code, unused_macros, unused_imports)]

use std::collections::*;
use std::io;
use std::io::prelude::*;

use std::fmt::Debug;
use std::num::IntErrorKind;
use std::str::FromStr;

fn parse_input<Input, Type>(input: Input) -> Type
where
    Type: FromStr,
    <Type as FromStr>::Err: Debug,
    Input: AsRef<str>,
{
    input.as_ref().trim().parse().unwrap()
}

fn split_inputs(s: &str) -> (Vec<String>, Vec<String>) {
    let mut allergens = Vec::new();
    let mut ingredients = Vec::new();
    let (a, b) = s.split_once("(contains ").unwrap();
    for p in a.split_whitespace() {
        ingredients.push(p.to_owned());
    }
    for p in b[..(b.len() - 1)].split(", ") {
        allergens.push(p.to_owned());
    }
    (ingredients, allergens)
}

fn main() {
    let stdin = io::stdin();
    let inp: Vec<_> = stdin.lock().lines().map(|input| input.unwrap()).collect();

    let mut possible: HashMap<String, HashSet<String>> = HashMap::new();
    let mut ingredients = Vec::new();
    for x in inp.iter() {
        let (ing, all) = split_inputs(x);
        ingredients.extend(ing.iter().cloned());
        for allergen in all.iter() {
            let mut ingset: HashSet<_> = ing.iter().cloned().collect();
            if possible.contains_key(allergen) {
                ingset = possible[allergen].intersection(&ingset).cloned().collect();
            }
            possible.insert(allergen.clone(), ingset);
        }
    }
    let mut possible_allergens = HashSet::new();
    for (_, v) in possible.iter() {
        possible_allergens.extend(v.iter());
    }
    let p1ans = ingredients
        .iter()
        .filter(|x| possible_allergens.contains(*x) == false)
        .count();
    println!("{:?}", p1ans);

    let mut allergens = HashMap::new();
    let mut found_ingredients = HashSet::new();
    while possible.len() > 0 {
        for (k, all) in possible.iter().filter(|(_, all)| all.len() == 1) {
            let ing = all.iter().next().unwrap();
            allergens.insert(k.clone(), ing.clone());
            found_ingredients.insert(ing.clone());
        }
        possible.retain(|k, _| allergens.contains_key(k) == false);
        for (_, v) in possible.iter_mut() {
            v.retain(|x| found_ingredients.contains(x) == false);
        }
    }
    let mut p2ans: Vec<_> = allergens.iter().collect();

    p2ans.sort_by_key(|x| x.0);
    let list = p2ans.iter().map(|x| x.1).cloned().collect::<Vec<_>>();
    println!("{:?}", list.join(","));
}
