//! プログラムカウンター

use crate::core::database::*;
use std::{cell::RefCell, fmt::Debug, rc::Rc};

/// プログラムカウンター
#[derive(Debug)]
pub struct ProgramCounter<T: Debug + Clone + Default, H: ProgramCounterHistory<T>> {
    data: T,
    history: Option<Rc<RefCell<H>>>,
}
impl<T: Debug + Clone + Default, H: ProgramCounterHistory<T>> ProgramCounter<T, H> {
    /// 新しいプログラムカウンターを作成
    pub fn new(data: Option<T>, history: Option<Rc<RefCell<H>>>) -> Self {
        Self {
            data: data.unwrap_or_default(),
            history,
        }
    }
}
impl<T: Debug + Clone + Default, H: ProgramCounterHistory<T>> Database for ProgramCounter<T, H> {
    type Key = ();
    type Value = T;
    type Error = ();

    fn load(&self, _: Self::Key) -> Result<Self::Value, Self::Error> {
        Ok(self.data.clone())
    }

    fn store(&mut self, _: Self::Key, value: Self::Value) -> Result<(), Self::Error> {
        if let Some(ref mut h) = self.history {
            let pre = self.data.clone();
            h.borrow_mut().program_counter(pre, value.clone());
        }
        self.data = value;
        Ok(())
    }
}

/// プログラムカウンターの履歴トレイト
pub trait ProgramCounterHistory<T: Debug + Clone + Default>: History {
    /// プログラムカウンターの変更を記録
    ///
    /// # Arguments
    ///
    /// * `pre` - 変更前の値
    /// * `post` - 変更後の値
    fn program_counter(&mut self, pre: T, post: T);
}
