#![allow(dead_code, unused_macros, unused_imports)]
#[macro_use]
extern crate text_io;
use std::collections::*;
use std::io;
use std::io::prelude::*;

#[derive(PartialEq, Debug, Eq, Clone, Hash)]
struct SimpleRule {
    match_str: String,
}

#[derive(PartialEq, Debug, Eq, Clone, Hash)]
struct ChainedRule {
    rules: Vec<usize>,
}

#[derive(PartialEq, Debug, Eq, Clone, Hash)]
struct OrRule {
    rules: Vec<usize>,
}

impl SimpleRule {
    fn new(x: String) -> SimpleRule {
        SimpleRule { match_str: x }
    }
}

impl OrRule {
    fn new(x: &Vec<usize>) -> OrRule {
        OrRule { rules: x.clone() }
    }
}
impl ChainedRule {
    fn new_from_string(x: &str) -> ChainedRule {
        let idxs = x
            .trim()
            .split(" ")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        ChainedRule { rules: idxs }
    }
}

trait MatchingRule {
    fn get_matches(&self, s: &str, rules: &HashMap<usize, Box<dyn MatchingRule>>)
        -> HashSet<usize>;
}

impl MatchingRule for SimpleRule {
    fn get_matches(&self, s: &str, _: &HashMap<usize, Box<dyn MatchingRule>>) -> HashSet<usize> {
        let mut p = HashSet::new();
        if s.starts_with(&self.match_str) {
            p.insert(self.match_str.len());
        }
        p
    }
}

impl MatchingRule for ChainedRule {
    fn get_matches(
        &self,
        s: &str,
        rules: &HashMap<usize, Box<dyn MatchingRule>>,
    ) -> HashSet<usize> {
        let mut p = HashSet::new();
        p.insert(0);
        for x in self.rules.iter() {
            let r = rules.get(x).unwrap();
            let mut next = HashSet::new();
            for v in p {
                for z in r.get_matches(&s[v..], rules) {
                    next.insert(v.clone() + &z);
                }
            }
            p = next
        }
        p
    }
}

impl MatchingRule for OrRule {
    fn get_matches(
        &self,
        s: &str,
        rules: &HashMap<usize, Box<dyn MatchingRule>>,
    ) -> HashSet<usize> {
        let mut p = HashSet::new();
        for x in self.rules.iter() {
            let r = rules.get(x).unwrap();
            p.extend(r.get_matches(s, rules));
        }
        p
    }
}

fn create_rule(x: &str, rules: &mut HashMap<usize, Box<dyn MatchingRule>>, extra: &mut usize) {
    let temp = x.split(":").collect::<Vec<_>>();
    let idx = temp[0].parse::<usize>().unwrap();
    let other = temp[1];
    if other.contains("\"") {
        rules.insert(idx, Box::new(SimpleRule::new(other[2..3].to_string())));
    } else {
        let mut t = Vec::new();
        for p in other.split("|") {
            rules.insert(*extra, Box::new(ChainedRule::new_from_string(&p)));
            t.push(*extra);
            *extra += 1;
        }
        rules.insert(idx, Box::new(OrRule::new(&t)));
    }
}

fn main() {
    let stdin = io::stdin();
    let inp: Vec<_> = stdin
        .lock()
        .lines()
        .map(|input| {
            let s = input.unwrap();
            s
        })
        .collect();
    let mut rules: HashMap<usize, Box<dyn MatchingRule>> = HashMap::new();
    let mut phase2 = false;
    let mut ans = Vec::new();
    let mut extra = inp.len();
    for x in inp {
        if x == "" {
            phase2 = true;
            continue;
        }
        if phase2 {
            ans.push(x);
        } else {
            create_rule(&x, &mut rules, &mut extra);
        }
    }
    let mut count = 0;
    for z in ans.iter() {
        let t = rules[&0].as_ref().get_matches(&z, &rules);
        if t.contains(&z.len()) {
            count += 1;
        }
    }
    println!("{}", count);

    create_rule("8: 42 | 42 8", &mut rules, &mut extra);
    create_rule("11: 42 31 | 42 11 31", &mut rules, &mut extra);
    let mut count = 0;
    for z in ans.iter() {
        let t = rules[&0].as_ref().get_matches(&z, &rules);
        if t.contains(&z.len()) {
            count += 1;
        }
    }
    println!("{}", count);
}
