//! メモリ

use crate::core::database::*;
use std::{array, cell::RefCell, fmt::Debug, rc::Rc};
use thiserror::Error;

/// メモリ
#[derive(Debug)]
pub struct Memory<T: Debug + Clone + Default, const N: usize, H: MemoryHistory<T>> {
    data: [T; N],
    history: Option<Rc<RefCell<H>>>,
}
impl<T: Debug + Clone + Default, const N: usize, H: MemoryHistory<T>> Memory<T, N, H> {
    /// 新しいメモリを作成
    pub fn new(data: Option<[T; N]>, history: Option<Rc<RefCell<H>>>) -> Self {
        Self {
            data: data.unwrap_or(array::from_fn(|_| T::default())),
            history,
        }
    }
}
impl<T: Debug + Clone + Default, const N: usize, H: MemoryHistory<T>> Database for Memory<T, N, H> {
    type Key = usize;
    type Value = T;
    type Error = MemoryError;

    fn load(&self, key: Self::Key) -> Result<Self::Value, Self::Error> {
        if key >= N {
            return Err(MemoryError::OutOfRange);
        }
        Ok(self.data[key].clone())
    }

    fn store(&mut self, key: Self::Key, value: Self::Value) -> Result<(), Self::Error> {
        if key >= N {
            return Err(MemoryError::OutOfRange);
        }
        if let Some(ref mut h) = self.history {
            let pre = self.data[key].clone();
            h.borrow_mut().memory(key, pre, value.clone());
        }
        self.data[key] = value;
        Ok(())
    }
}

/// メモリの履歴トレイト
pub trait MemoryHistory<T: Debug + Clone + Default>: History {
    /// メモリの変更を記録
    ///
    /// # Arguments
    ///
    /// * `address` - アドレス
    /// * `pre` - 変更前の値
    /// * `post` - 変更後の値
    fn memory(&mut self, address: usize, pre: T, post: T);
}

/// メモリエラー
#[derive(Debug, Error)]
pub enum MemoryError {
    /// 範囲外
    #[error("out of range")]
    OutOfRange,
}
