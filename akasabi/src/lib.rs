//! 赤錆
//!
//! 赤錆は、 Rust で書かれた、コンピューターのハードウェアやそれを制御するソフトウェアのシミュレーターを作成するライブラリ。
//! トレイトを用いて、ハードウェアやソフトウェアの構成要素を表現し、それらを組み合わせてシミュレーターを構築することができる。

#![warn(
    missing_docs,
    rustdoc::missing_crate_level_docs,
    unused_results,
    clippy::complexity,
    clippy::perf,
    clippy::style
)]
#![deny(clippy::correctness, clippy::suspicious)]
