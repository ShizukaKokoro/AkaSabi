#![allow(unused)]

use std::{cell::RefCell, fmt::Debug, rc::Rc};

use akasabi::lc3_no_memory::*;

fn main() {
    let tree = Assembly::new(vec![]); // TODO: 命令をテストする
    let history = Rc::new(RefCell::new(LC3NoMemoryHistory::default()));
    let mut prc = LC3NoMemoryProcessor::new(tree, None, None, None, Some(history.clone()));

    while prc.step().is_ok() {
        println!("{:?}", history.borrow());
    }
}
