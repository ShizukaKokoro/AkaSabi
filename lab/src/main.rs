#![allow(unused)]

use std::{cell::RefCell, fmt::Debug, rc::Rc};

use akasabi::{processor::lc3_no_memory::*, syntax_tree::assembly::Assembly};

fn main() {
    let tree = Assembly::new(vec![]);
    let history = Rc::new(RefCell::new(LC3NoMemoryHistory::default()));
    let mut prc = LC3NoMemoryProcessor::new(tree, None, None, None, Some(history.clone()));

    while prc.step().is_ok() {
        println!("{:?}", history.borrow());
    }
}
