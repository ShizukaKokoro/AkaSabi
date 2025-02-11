#![allow(unused)]

use akasabi::lc3_no_memory::*;
use std::{cell::RefCell, fmt::Debug, rc::Rc, thread::sleep, time::Duration};

fn main() {
    let tree = Assembly::new(vec![
        LC3NoMemoryInstructions::Arithmetic(ArithmeticInstructions::ANDI {
            dr: 1,
            sr1: 1,
            imm5: 0.into(),
        }),
        LC3NoMemoryInstructions::Arithmetic(ArithmeticInstructions::ADDI {
            dr: 1,
            sr1: 1,
            imm5: 5.into(),
        }),
        LC3NoMemoryInstructions::Arithmetic(ArithmeticInstructions::ANDI {
            dr: 2,
            sr1: 2,
            imm5: 0.into(),
        }),
        LC3NoMemoryInstructions::Arithmetic(ArithmeticInstructions::ADDI {
            dr: 2,
            sr1: 2,
            imm5: 3.into(),
        }),
        LC3NoMemoryInstructions::Arithmetic(ArithmeticInstructions::ADD {
            dr: 3,
            sr1: 1,
            sr2: 2,
        }),
        LC3NoMemoryInstructions::Control(ControlInstructions::BR {
            n: false,
            z: false,
            p: true,
            pc_offset9: 2,
        }),
        LC3NoMemoryInstructions::Control(ControlInstructions::BR {
            n: true,
            z: true,
            p: true,
            pc_offset9: usize::MAX,
        }),
        // ADD_R_OK
        LC3NoMemoryInstructions::Arithmetic(ArithmeticInstructions::ANDI {
            dr: 4,
            sr1: 4,
            imm5: 0.into(),
        }),
        LC3NoMemoryInstructions::Arithmetic(ArithmeticInstructions::ADDI {
            dr: 4,
            sr1: 4,
            imm5: (-5i8 as u8).into(),
        }),
        LC3NoMemoryInstructions::Control(ControlInstructions::BR {
            n: true,
            z: false,
            p: false,
            pc_offset9: 2,
        }),
        LC3NoMemoryInstructions::Control(ControlInstructions::BR {
            n: true,
            z: true,
            p: true,
            pc_offset9: usize::MAX,
        }),
        // ADD_I_OK
        LC3NoMemoryInstructions::Arithmetic(ArithmeticInstructions::ANDI {
            dr: 5,
            sr1: 5,
            imm5: 0.into(),
        }),
        LC3NoMemoryInstructions::Arithmetic(ArithmeticInstructions::ADDI {
            dr: 5,
            sr1: 5,
            imm5: 6.into(),
        }),
        LC3NoMemoryInstructions::Arithmetic(ArithmeticInstructions::ANDI {
            dr: 6,
            sr1: 6,
            imm5: 0.into(),
        }),
        LC3NoMemoryInstructions::Arithmetic(ArithmeticInstructions::ADDI {
            dr: 6,
            sr1: 6,
            imm5: 3.into(),
        }),
        LC3NoMemoryInstructions::Arithmetic(ArithmeticInstructions::AND {
            dr: 0,
            sr1: 5,
            sr2: 6,
        }),
        LC3NoMemoryInstructions::Control(ControlInstructions::BR {
            n: false,
            z: false,
            p: true,
            pc_offset9: 2,
        }),
        LC3NoMemoryInstructions::Control(ControlInstructions::BR {
            n: true,
            z: true,
            p: true,
            pc_offset9: usize::MAX,
        }),
        // AND_R_OK
        LC3NoMemoryInstructions::Arithmetic(ArithmeticInstructions::ANDI {
            dr: 1,
            sr1: 1,
            imm5: 0.into(),
        }),
        LC3NoMemoryInstructions::Arithmetic(ArithmeticInstructions::ADDI {
            dr: 1,
            sr1: 1,
            imm5: 15.into(),
        }),
        LC3NoMemoryInstructions::Arithmetic(ArithmeticInstructions::ANDI {
            dr: 2,
            sr1: 1,
            imm5: 5.into(),
        }),
        LC3NoMemoryInstructions::Control(ControlInstructions::BR {
            n: false,
            z: false,
            p: true,
            pc_offset9: 2,
        }),
        LC3NoMemoryInstructions::Control(ControlInstructions::BR {
            n: true,
            z: true,
            p: true,
            pc_offset9: usize::MAX,
        }),
        // AND_I_OK
        LC3NoMemoryInstructions::Arithmetic(ArithmeticInstructions::ANDI {
            dr: 3,
            sr1: 3,
            imm5: 0.into(),
        }),
        LC3NoMemoryInstructions::Arithmetic(ArithmeticInstructions::NOT { dr: 3, sr: 3 }),
        LC3NoMemoryInstructions::Control(ControlInstructions::BR {
            n: true,
            z: false,
            p: false,
            pc_offset9: 2,
        }),
        LC3NoMemoryInstructions::Control(ControlInstructions::BR {
            n: true,
            z: true,
            p: true,
            pc_offset9: usize::MAX,
        }),
        // NOT_OK
        LC3NoMemoryInstructions::Arithmetic(ArithmeticInstructions::ANDI {
            dr: 4,
            sr1: 4,
            imm5: 0.into(),
        }),
        LC3NoMemoryInstructions::Control(ControlInstructions::BR {
            n: false,
            z: true,
            p: false,
            pc_offset9: 2,
        }),
        LC3NoMemoryInstructions::Control(ControlInstructions::BR {
            n: true,
            z: true,
            p: true,
            pc_offset9: usize::MAX,
        }),
        // BR_OK
        LC3NoMemoryInstructions::LEA {
            dr: 1,
            pc_offset9: 3,
        },
        LC3NoMemoryInstructions::Arithmetic(ArithmeticInstructions::ANDI {
            dr: 5,
            sr1: 5,
            imm5: 0.into(),
        }),
        LC3NoMemoryInstructions::Control(ControlInstructions::JMP { base_r: 1 }),
        // JMP_RET
        LC3NoMemoryInstructions::Control(ControlInstructions::BR {
            n: false,
            z: true,
            p: false,
            pc_offset9: 2,
        }),
        LC3NoMemoryInstructions::Control(ControlInstructions::BR {
            n: true,
            z: true,
            p: true,
            pc_offset9: usize::MAX,
        }),
        // JMP_OK
        LC3NoMemoryInstructions::Arithmetic(ArithmeticInstructions::ANDI {
            dr: 6,
            sr1: 6,
            imm5: 0.into(),
        }),
        LC3NoMemoryInstructions::Control(ControlInstructions::JSR { pc_offset11: 14 }),
        LC3NoMemoryInstructions::Control(ControlInstructions::BR {
            n: false,
            z: true,
            p: false,
            pc_offset9: 2,
        }),
        LC3NoMemoryInstructions::Control(ControlInstructions::BR {
            n: true,
            z: true,
            p: true,
            pc_offset9: usize::MAX,
        }),
        // JSR_OK
        LC3NoMemoryInstructions::LEA {
            dr: 1,
            pc_offset9: 12,
        },
        LC3NoMemoryInstructions::Arithmetic(ArithmeticInstructions::ANDI {
            dr: 2,
            sr1: 2,
            imm5: 0.into(),
        }),
        LC3NoMemoryInstructions::Control(ControlInstructions::JSRR { base_r: 1 }),
        LC3NoMemoryInstructions::Control(ControlInstructions::BR {
            n: false,
            z: true,
            p: false,
            pc_offset9: 2,
        }),
        LC3NoMemoryInstructions::Control(ControlInstructions::BR {
            n: true,
            z: true,
            p: true,
            pc_offset9: usize::MAX,
        }),
        // JSRR_OK
        LC3NoMemoryInstructions::LEA {
            dr: 7,
            pc_offset9: 3,
        },
        LC3NoMemoryInstructions::Arithmetic(ArithmeticInstructions::ANDI {
            dr: 3,
            sr1: 3,
            imm5: 0.into(),
        }),
        LC3NoMemoryInstructions::Control(ControlInstructions::RET),
        // RET_OK_LABEL
        LC3NoMemoryInstructions::Control(ControlInstructions::BR {
            n: false,
            z: true,
            p: false,
            pc_offset9: 2,
        }),
        LC3NoMemoryInstructions::Control(ControlInstructions::BR {
            n: true,
            z: true,
            p: true,
            pc_offset9: usize::MAX,
        }),
        // RET_OK
        LC3NoMemoryInstructions::Control(ControlInstructions::BR {
            n: true,
            z: true,
            p: true,
            pc_offset9: 3,
        }),
        // JSR_SUB
        LC3NoMemoryInstructions::Control(ControlInstructions::RET),
        // JSRR_SUB
        LC3NoMemoryInstructions::Control(ControlInstructions::RET),
        // ERROR
        // HALT
    ]);
    let history = Rc::new(RefCell::new(LC3NoMemoryHistory::default()));
    let mut prc = LC3NoMemoryProcessor::new(tree, None, None, None, Some(history.clone()));

    while prc.step().is_ok() {
        println!("{:?}", history.borrow());
        sleep(Duration::from_secs(1));
    }
}
