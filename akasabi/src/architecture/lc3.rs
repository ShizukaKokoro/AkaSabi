//! Little Computer 3 (LC-3)
//!
//! LC-3 の命令セットを実装する。

use crate::core::limited_bit::LimitedBit;

/// 演算命令
pub enum Arithmetic<R> {
    /// 加算命令
    ADD {
        /// ディスティネーションレジスタ
        dr: R,
        /// ソースレジスタ1
        sr1: R,
        /// ソースレジスタ2
        sr2: R,
    },
    /// 加算命令（即値）
    ADDI {
        /// ディスティネーションレジスタ
        dr: R,
        /// ソースレジスタ1
        sr1: R,
        /// 即値
        imm5: LimitedBit<u16, 5>,
    },
    /// 論理積命令
    AND {
        /// ディスティネーションレジスタ
        dr: R,
        /// ソースレジスタ1
        sr1: R,
        /// ソースレジスタ2
        sr2: R,
    },
    /// 論理積命令（即値）
    ANDI {
        /// ディスティネーションレジスタ
        dr: R,
        /// ソースレジスタ1
        sr1: R,
        /// 即値
        imm5: LimitedBit<u16, 5>,
    },
    /// 否定命令
    NOT {
        /// ディスティネーションレジスタ
        dr: R,
        /// ソースレジスタ
        sr: R,
    },
}

/// 制御命令
pub enum Control<R> {
    /// 分岐命令
    BR {
        /// 負
        n: bool,
        /// ゼロ
        z: bool,
        /// 正
        p: bool,
        /// オフセット
        pc_offset9: LimitedBit<u16, 9>,
    },
    /// ジャンプ命令
    JMP {
        /// ソースレジスタ
        base_r: R,
    },
    /// ジャンプ命令（間接）
    JSR {
        /// オフセット
        pc_offset11: LimitedBit<u16, 11>,
    },
    /// ジャンプ命令（レジスタ間接）
    JSRR {
        /// ソースレジスタ
        base_r: R,
    },
    /// 復帰命令
    RET,
    /// 割り込みからの復帰命令
    RTI,
}

/// ロード/ストア命令
pub enum LoadStore<R> {
    /// ロード命令
    LD {
        /// ディスティネーションレジスタ
        dr: R,
        /// オフセット
        pc_offset9: LimitedBit<u16, 9>,
    },
    /// ロード命令（レジスタ間接）
    LDR {
        /// ディスティネーションレジスタ
        dr: R,
        /// ソースレジスタ
        base_r: R,
        /// オフセット
        offset6: LimitedBit<u16, 6>,
    },
    /// ロード命令（間接）
    LDI {
        /// ディスティネーションレジスタ
        dr: R,
        /// オフセット
        pc_offset9: LimitedBit<u16, 9>,
    },
    /// ロード命令(即値)
    ///
    /// メモリへのアクセスがない。
    LEA {
        /// ディスティネーションレジスタ
        dr: R,
        /// オフセット
        pc_offset9: LimitedBit<u16, 9>,
    },
    /// ストア命令
    ST {
        /// ソースレジスタ
        sr: R,
        /// オフセット
        pc_offset9: LimitedBit<u16, 9>,
    },
    /// ストア命令（レジスタ間接）
    STR {
        /// ソースレジスタ
        sr: R,
        /// ソースレジスタ
        base_r: R,
        /// オフセット
        offset6: LimitedBit<u16, 6>,
    },
    /// ストア命令（間接）
    STI {
        /// ソースレジスタ
        sr: R,
        /// オフセット
        pc_offset9: LimitedBit<u16, 9>,
    },
}

/// トラップ命令
pub struct Trap {
    /// トラップベクタ
    pub trap_vect8: LimitedBit<u16, 8>,
}

/// 予約済み
pub struct Reserved {}

/// 命令セット
pub enum Instruction<R> {
    /// 演算命令
    Arithmetic(Arithmetic<R>),
    /// 制御命令
    Control(Control<R>),
    /// ロード/ストア命令
    LoadStore(LoadStore<R>),
    /// トラップ命令
    Trap(Trap),
    /// 予約済み
    Reserved(Reserved),
}
