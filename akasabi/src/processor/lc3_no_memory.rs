//! メモリなし LC-3 プロセッサ
//!
//! メモリを持たない、LC-3 プロセッサの最小限の実装。
//! アセンブリの構文木を直接実行する。

use crate::{
    core::database::*,
    database::{
        program_counter::{ProgramCounter, ProgramCounterHistory},
        register::{
            Register, RegisterError, RegisterHistory, StatusRegister, StatusRegisterHistory,
        },
    },
    instructions::lc3::{ArithmeticInstructions, ControlInstructions},
    syntax_tree::assembly::Assembly,
};
use std::{cell::RefCell, fmt::Debug, rc::Rc};
use thiserror::Error;

/// 最小限の LC-3 命令
#[derive(Debug, Clone)]
pub enum LC3NoMemoryInstructions {
    /// 演算命令
    Arithmetic(ArithmeticInstructions<usize>),
    /// 制御命令
    Control(ControlInstructions<usize, u16>),
}

/// 最小限の LC-3 プロセッサ
pub struct LC3NoMemoryProcessor<
    H: RegisterHistory<u16> + StatusRegisterHistory + ProgramCounterHistory<usize>,
> {
    reg: Register<u16, 8, H>,
    psr: StatusRegister<3, H>,
    pc: ProgramCounter<usize, H>,
    tree: Assembly<LC3NoMemoryInstructions>,
}
impl<H: RegisterHistory<u16> + StatusRegisterHistory + ProgramCounterHistory<usize>>
    LC3NoMemoryProcessor<H>
{
    /// 新しいプロセッサを作成
    pub fn new(
        tree: Assembly<LC3NoMemoryInstructions>,
        reg_data: Option<[u16; 8]>,
        psr_data: Option<[bool; 3]>,
        pc_data: Option<usize>,
        history: Option<Rc<RefCell<H>>>,
    ) -> Self {
        let history = history.map(|h| Rc::clone(&h));
        Self {
            reg: Register::new(reg_data, history.clone()),
            psr: StatusRegister::new(psr_data, history.clone()),
            pc: ProgramCounter::new(pc_data, history.clone()),
            tree,
        }
    }

    fn setcc(&mut self, result: u16) -> Result<(), LC3NoMemoryError> {
        self.psr.store(0, result & 0x80 != 0)?;
        self.psr.store(1, result == 0)?;
        self.psr.store(2, !result & 0x80 != 0)?;
        Ok(())
    }

    /// 1ステップずつ実行
    pub fn step(&mut self) -> Result<(), LC3NoMemoryError> {
        let pc = self.pc.load(()).unwrap();
        let inst = self.tree.get(pc);
        match inst {
            Some(inst) => match inst {
                LC3NoMemoryInstructions::Arithmetic(inst) => match inst {
                    ArithmeticInstructions::ADD { dr, sr1, sr2 } => {
                        let sr1 = self.reg.load(sr1)?;
                        let sr2 = self.reg.load(sr2)?;
                        let result = sr1.wrapping_add(sr2);
                        self.setcc(result)?;
                        self.reg.store(dr, result)?;
                    }
                    ArithmeticInstructions::ADDI { dr, sr1, imm5 } => {
                        let sr1 = self.reg.load(sr1)?;
                        let imm5 = {
                            let imm5 = imm5.get() as u16;
                            if imm5 & 0x10 != 0 {
                                imm5 | 0xFFE0
                            } else {
                                imm5
                            }
                        };
                        let result = sr1.wrapping_add(imm5);
                        self.setcc(result)?;
                        self.reg.store(dr, result)?;
                    }
                    ArithmeticInstructions::AND { dr, sr1, sr2 } => {
                        let sr1 = self.reg.load(sr1)?;
                        let sr2 = self.reg.load(sr2)?;
                        let result = sr1 & sr2;
                        self.setcc(result)?;
                        self.reg.store(dr, result)?;
                    }
                    ArithmeticInstructions::ANDI { dr, sr1, imm5 } => {
                        let sr1 = self.reg.load(sr1)?;
                        let imm5 = {
                            let imm5 = imm5.get() as u16;
                            if imm5 & 0x10 != 0 {
                                imm5 | 0xFFE0
                            } else {
                                imm5
                            }
                        };
                        let result = sr1 & imm5;
                        self.setcc(result)?;
                        self.reg.store(dr, result)?;
                    }
                    ArithmeticInstructions::NOT { dr, sr } => {
                        let sr = self.reg.load(sr)?;
                        let result = !sr;
                        self.setcc(result)?;
                        self.reg.store(dr, result)?;
                    }
                },
                LC3NoMemoryInstructions::Control(inst) => match inst {
                    ControlInstructions::BR {
                        n,
                        z,
                        p,
                        pc_offset9,
                    } => {
                        let n_ = self.psr.load(0)?;
                        let z_ = self.psr.load(1)?;
                        let p_ = self.psr.load(2)?;
                        if (n_ && n) || (z_ && z) || (p_ && p) {
                            let pc_offset9 = pc_offset9 as i16;
                            let pc = pc.wrapping_add(pc_offset9 as usize);
                            self.pc.store((), pc).unwrap();
                        }
                    }
                    ControlInstructions::JMP { base_r } => {
                        let base_r = self.reg.load(base_r as usize)?;
                        self.pc.store((), base_r as usize).unwrap();
                    }
                    ControlInstructions::JSR { pc_offset11 } => {
                        let pc = self.pc.load(()).unwrap();
                        self.reg.store(7, pc as u16)?;
                        let pc_offset11 = pc_offset11 as u16;
                        let pc = pc.wrapping_add(pc_offset11 as usize);
                        self.pc.store((), pc).unwrap();
                    }
                    ControlInstructions::JSRR { base_r } => {
                        let pc = self.pc.load(()).unwrap();
                        self.reg.store(7, pc as u16)?;
                        let base_r = self.reg.load(base_r as usize)?;
                        self.pc.store((), base_r as usize).unwrap();
                    }
                    ControlInstructions::RET => {
                        let pc = self.reg.load(7)?;
                        self.pc.store((), pc as usize).unwrap();
                    }
                },
            },
            None => return Err(LC3NoMemoryError::Halted),
        }

        Ok(())
    }
}

/// 最小限の LC-3 プロセッサの差分
#[derive(Debug, Default)]
struct LC3NoMemoryDiff {
    reg: Vec<Diff<usize, u16>>,
    psr: Vec<Diff<usize, bool>>,
    pc: Vec<Diff<(), usize>>,
}

/// 最小限の LC-3 プロセッサの履歴
#[derive(Default)]
pub struct LC3NoMemoryHistory {
    reg: [u16; 8],
    psr: (bool, bool, bool),
    pc: usize,
    diffs: Vec<LC3NoMemoryDiff>,
    now: usize,
}
impl Debug for LC3NoMemoryHistory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "History (t = {})", self.now)?;
        for (i, r) in self.reg.iter().enumerate() {
            writeln!(f, "\tR{}: {:016b}", i, r)?;
        }
        writeln!(f, "\tPSR: {:?}", self.psr)?;
        writeln!(f, "\tpc: {}", self.pc)?;
        Ok(())
    }
}
impl History for LC3NoMemoryHistory {}
impl RegisterHistory<u16> for LC3NoMemoryHistory {
    fn register(&mut self, key: usize, pre: u16, post: u16) {
        while self.diffs.len() <= self.now {
            self.diffs.push(LC3NoMemoryDiff::default());
        }
        self.diffs[self.now].reg.push(Diff::new(key, pre, post));
    }
}
impl StatusRegisterHistory for LC3NoMemoryHistory {
    fn status_register(&mut self, key: usize, pre: bool, post: bool) {
        while self.diffs.len() <= self.now {
            self.diffs.push(LC3NoMemoryDiff::default());
        }
        self.diffs[self.now].psr.push(Diff::new(key, pre, post));
    }
}
impl ProgramCounterHistory<usize> for LC3NoMemoryHistory {
    fn program_counter(&mut self, pre: usize, post: usize) {
        while self.diffs.len() <= self.now {
            self.diffs.push(LC3NoMemoryDiff::default());
        }
        self.diffs[self.now].pc.push(Diff::new((), pre, post));
    }
}

/// 最小限の LC-3 プロセッサのエラー
#[derive(Debug, Error)]
pub enum LC3NoMemoryError {
    /// レジスタエラー
    #[error("RegisterError: {0}")]
    RegisterError(#[from] RegisterError),

    /// プロセス停止
    #[error("Process Halted")]
    Halted,
}
