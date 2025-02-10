//! 赤錆

#![warn(
    missing_docs,
    rustdoc::missing_crate_level_docs,
    unused_results,
    clippy::complexity,
    clippy::perf,
    clippy::style
)]
#![deny(clippy::correctness, clippy::suspicious)]

pub mod core;
pub mod database;
pub mod instructions;
pub mod processor;
pub mod syntax_tree;

/// メモリなし LC-3 プロセッサー
///
/// メモリを持たない、LC-3 プロセッサの最小限の実装。
/// テスト用のプロセッサー
pub mod lc3_no_memory {
    pub use crate::{
        instructions::lc3::{ArithmeticInstructions, ControlInstructions},
        processor::lc3_no_memory::*,
        syntax_tree::assembly::Assembly,
    };
}
