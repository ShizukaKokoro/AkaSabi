//! LC-3 命令セット
//!
//! Little Computer 3 (LC-3) は、シンプルな16ビットのプロセッサ。
//! このモジュールは、LC-3 の命令セットを定義している。

use crate::core::*;
use std::{fmt::Debug, hash::Hash};

/// 演算命令
#[derive(Debug)]
pub enum ArithmeticInstructions<R: Debug + Hash> {
    /// 加算
    ADD {
        /// 宛先
        dr: R,
        /// オペランド1
        sr1: R,
        /// オペランド2
        sr2: R,
    },
    /// 加算（即値）
    ADDI {
        /// 宛先
        dr: R,
        /// オペランド1
        sr1: R,
        /// オペランド2
        imm5: LimitedU8<5>,
    },
    /// 論理積
    AND {
        /// 宛先
        dr: R,
        /// オペランド1
        sr1: R,
        /// オペランド2
        sr2: R,
    },
    /// 論理積（即値）
    ANDI {
        /// 宛先
        dr: R,
        /// オペランド1
        sr1: R,
        /// オペランド2
        imm5: LimitedU8<5>,
    },
    /// 否定
    NOT {
        /// 宛先
        dr: R,
        /// オペランド
        sr: R,
    },
}

/// 制御命令
#[derive(Debug)]
pub enum ControlInstructions<L, R: Debug + Hash> {
    /// 条件ジャンプ
    BR {
        /// 負の場合
        n: bool,
        /// ゼロの場合
        z: bool,
        /// 正の場合
        p: bool,
        /// ジャンプ先
        pc_offset9: L,
    },
    /// ジャンプ
    JMP {
        /// ジャンプ先
        base_r: R,
    },
    /// 復帰前提のジャンプ
    JSR {
        /// ジャンプ先
        pc_offset11: L,
    },
    /// 復帰前提のジャンプ（レジスタ）
    JSRR {
        /// ジャンプ先
        base_r: R,
    },
    /// 復帰
    RET,
    /// 割り込みからの復帰
    RTI,
    /// トラップ
    TRAP {
        /// トラップベクタ
        trap_vect8: LimitedU8<8>,
    },
}

/// ロード・ストア命令
#[derive(Debug)]
pub enum LoadStoreInstructions<L, R: Debug + Hash> {
    /// ロード
    LD {
        /// 宛先
        dr: R,
        /// オフセット
        pc_offset9: L,
    },
    /// 間接ロード
    LDI {
        /// 宛先
        dr: R,
        /// オフセット
        pc_offset9: L,
    },
    /// ロード・ベース・オフセット
    LDR {
        /// 宛先
        dr: R,
        /// ベース
        base_r: R,
        /// オフセット
        offset6: LimitedU8<6>,
    },
    /// アドレスロード
    LEA {
        /// 宛先
        dr: R,
        /// オフセット
        pc_offset9: L,
    },
    /// ストア
    ST {
        /// 宛先
        sr: R,
        /// オフセット
        pc_offset9: L,
    },
    /// 間接ストア
    STI {
        /// 宛先
        sr: R,
        /// オフセット
        pc_offset9: L,
    },
    /// ストア・ベース・オフセット
    STR {
        /// 宛先
        sr: R,
        /// ベース
        base_r: R,
        /// オフセット
        offset6: LimitedU8<6>,
    },
}

/// 未使用命令
#[derive(Debug)]
pub struct UnusedInstruction(pub LimitedU16<12>);

/// 命令
#[derive(Debug)]
pub enum LC3Instructions<L, R: Debug + Hash> {
    /// 演算命令
    Arithmetic(ArithmeticInstructions<R>),
    /// 制御命令
    Control(ControlInstructions<L, R>),
    /// ロード・ストア命令
    LoadStore(LoadStoreInstructions<L, R>),
    /// 未使用命令
    Unused(UnusedInstruction),
}
