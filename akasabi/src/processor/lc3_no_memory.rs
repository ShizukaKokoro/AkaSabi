//! メモリなし LC-3 プロセッサ
//!
//! メモリを持たない、LC-3 プロセッサの最小限の実装。
//! アセンブリの構文木を直接実行する。

// TODO: このファイルを実装する

use super::*;
use crate::{
    core::*,
    database::{
        program_counter::ProgramCounter,
        register::{Register, StatusRegister},
    },
    instructions::lc3::{ArithmeticInstructions, ControlInstructions},
    syntax_tree::assembly::Assembly,
};

/// 最小限の LC-3 命令
pub enum LC3NoMemoryInstructions {
    /// 演算命令
    Arithmetic(ArithmeticInstructions<usize>),
    /// 制御命令
    Control(ControlInstructions<usize, u16>),
}

/// 最小限の LC-3 プロセッサ
pub struct LC3NoMemoryProcessor {
    registers: Register<u16, 8>,
    psr: StatusRegister<3>,
    pc: ProgramCounter<usize>,
    tree: Assembly<LC3NoMemoryInstructions>,
}
