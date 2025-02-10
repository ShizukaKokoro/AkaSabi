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
