#[macro_use]
extern crate text_io;
use std::collections::HashSet;
use std::io;

use std::io::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum AccInst {
    Nop,
    Acc,
    Jmp,
    Inv,
}
#[derive(Clone, Debug)]
pub struct AccComp {
    pub values: Vec<(AccInst, i64)>,
    pub pc: i64,
    pub acc: i64,
}

impl AccComp {
    pub fn step(&mut self) {
        let cmd = &self.values[self.pc as usize];
        match &cmd.0 {
            AccInst::Acc => {
                self.acc += cmd.1;
            }
            AccInst::Jmp => {
                self.pc += cmd.1 - 1;
            }
            AccInst::Nop => {}

            _ => panic!("Invalid Opcode {:?} {}", cmd, self.pc),
        };
        self.pc += 1;
    }

    pub fn build_from_stdin() -> AccComp {
        let stdin = io::stdin();
        let values: Vec<(AccInst, i64)> = stdin
            .lock()
            .lines()
            .map(|x| {
                let x = x.unwrap();
                let cmd: String;
                let b: i64;
                scan!(x.bytes() => "{} {}", cmd, b);
                let cmd_inst = match cmd.as_str() {
                    "nop" => AccInst::Nop,
                    "acc" => AccInst::Acc,
                    "jmp" => AccInst::Jmp,
                    _ => panic!("Unknown Opcode {:?}", cmd),
                };
                (cmd_inst, b)
            })
            .collect();
        AccComp {
            values: values,
            pc: 0,
            acc: 0,
        }
    }

    pub fn run_till_repeat(&mut self) -> bool {
        let mut seen = HashSet::new();
        while seen.insert(self.pc) {
            self.step();
            if self.pc == self.values.len() as i64 {
                return true;
            }
        }
        return false;
    }
}
