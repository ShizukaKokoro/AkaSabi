#![allow(unused)]

use std::fmt::Debug;

use akasabi::{
    core::*,
    database::{program_counter::ProgramCounter, register::Register, Database},
};

#[derive(Debug)]
enum PosKey {
    PC,
    Reg(usize),
}
impl From<Vec<String>> for PosKey {
    fn from(v: Vec<String>) -> Self {
        match v.as_slice() {
            [s] if s == "ProgramCounter" => Self::PC,
            [s1, s2] if s1 == "Register" => Self::Reg(s2.parse().unwrap()),
            _ => unreachable!(),
        }
    }
}

#[derive(Default)]
struct Record {
    pc: u16,
    reg: [u16; 8],
    diffs: Vec<Diffs<PosKey, u16>>,
    time: usize,
}
impl Record {
    fn prev(&mut self) {
        for diff in self.diffs[self.time - 1].iter() {
            match diff.pos() {
                PosKey::PC => {
                    self.pc = *diff.pre();
                }
                PosKey::Reg(i) => {
                    self.reg[*i] = *diff.pre();
                }
            }
        }
        self.time -= 1;
    }

    fn next(&mut self, diffs: Option<Diffs<PosKey, u16>>) {
        match diffs {
            Some(diffs) => {
                self.diffs.push(diffs);
            }
            None => {
                self.diffs.push(Diffs::default());
            }
        }
        self.time += 1;
        for diff in self.diffs[self.time - 1].iter() {
            match diff.pos() {
                PosKey::PC => {
                    self.pc = *diff.post();
                }
                PosKey::Reg(i) => {
                    self.reg[*i] = *diff.post();
                }
            }
        }
    }
}
impl Debug for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Record t = {}", self.time)?;
        writeln!(f, "\tpc: {}", self.pc)?;
        for (i, r) in self.reg.iter().enumerate() {
            writeln!(f, "\treg[{}]: {:016b}", i, r)?;
        }
        Ok(())
    }
}

fn main() {
    let mut record = Record::default();

    let mut pc = ProgramCounter::<u16>::default();
    let mut reg = Register::<u16, 8>::default();

    println!("{:?}", record);

    let mut diffs = Diffs::<PosKey, u16>::default();
    pc.store((), 2, &mut diffs).unwrap();
    reg.store(0, 0x1234, &mut diffs).unwrap();
    record.next(Some(diffs));
    println!("{:?}", record);

    record.prev();
    println!("{:?}", record);

    record.next(None);
    println!("{:?}", record);
}
