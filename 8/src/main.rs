#![allow(dead_code, unused_macros, unused_imports)]
#[macro_use]
extern crate text_io;
use acccomp::{AccComp, AccInst};
use std::collections::*;
use std::io;
use std::io::prelude::*;

fn main() {
    let orig_ac = AccComp::build_from_stdin();
    let mut ac = orig_ac.clone();
    ac.run_till_repeat();
    println!("{:?}", ac.acc);
    for (p, v) in ac.values.iter().enumerate() {
        let mut nc = orig_ac.clone();
        if v.0 == AccInst::Jmp {
            nc.values[p] = (AccInst::Nop, v.1);
        } else if v.0 == AccInst::Nop {
            nc.values[p] = (AccInst::Jmp, v.1);
        } else {
            continue;
        }
        if nc.run_till_repeat() {
            println!("{}", nc.acc);
            break;
        }
    }
}
