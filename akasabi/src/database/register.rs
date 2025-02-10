//! レジスタ

use crate::core::database::*;
use std::{cell::RefCell, fmt::Debug, rc::Rc};
use thiserror::Error;

/// レジスタ
///
/// 任意の型を任意の数だけ格納できるレジスタ
#[derive(Debug)]
pub struct Register<T: Debug + Copy + Default, const N: usize, H: RegisterHistory<T>> {
    data: [T; N],
    history: Option<Rc<RefCell<H>>>,
}
impl<T: Debug + Copy + Default, const N: usize, H: RegisterHistory<T>> Register<T, N, H> {
    /// 新しいレジスタを作成
    pub fn new(data: Option<[T; N]>, history: Option<Rc<RefCell<H>>>) -> Self {
        Self {
            data: data.unwrap_or([T::default(); N]),
            history,
        }
    }
}
impl<T: Debug + Copy + Default, const N: usize, H: RegisterHistory<T>> Database
    for Register<T, N, H>
{
    type Key = usize;
    type Value = T;
    type Error = RegisterError;

    fn load(&self, key: Self::Key) -> Result<Self::Value, Self::Error> {
        if key < N {
            Ok(self.data[key])
        } else {
            Err(RegisterError::OutOfRange)
        }
    }

    fn store(&mut self, key: Self::Key, value: Self::Value) -> Result<(), Self::Error> {
        if key < N {
            let pre = self.data[key];
            self.data[key] = value;
            if let Some(ref mut h) = self.history {
                h.borrow_mut().register(key, pre, value);
            }
            Ok(())
        } else {
            Err(RegisterError::OutOfRange)
        }
    }
}

/// レジスタの履歴トレイト
pub trait RegisterHistory<T: Debug + Copy + Default>: History {
    /// レジスタの変更を記録
    ///
    /// # Arguments
    ///
    /// * `key` - キー
    /// * `pre` - 変更前の値
    /// * `post` - 変更後の値
    fn register(&mut self, key: usize, pre: T, post: T);
}

/// レジスタエラー
#[derive(Debug, Error)]
pub enum RegisterError {
    /// 範囲外
    #[error("out of range")]
    OutOfRange,
}

/// ステータスレジスタ
#[derive(Debug)]
pub struct StatusRegister<const N: usize, H: StatusRegisterHistory> {
    data: [bool; N],
    history: Option<Rc<RefCell<H>>>,
}
impl<const N: usize, H: StatusRegisterHistory> StatusRegister<N, H> {
    /// 新しいステータスレジスタを作成
    pub fn new(data: Option<[bool; N]>, history: Option<Rc<RefCell<H>>>) -> Self {
        Self {
            data: data.unwrap_or([false; N]),
            history,
        }
    }
}
impl<const N: usize, H: StatusRegisterHistory> Database for StatusRegister<N, H> {
    type Key = usize;
    type Value = bool;
    type Error = RegisterError;

    fn load(&self, key: Self::Key) -> Result<Self::Value, Self::Error> {
        if key < N {
            Ok(self.data[key])
        } else {
            Err(RegisterError::OutOfRange)
        }
    }

    fn store(&mut self, key: Self::Key, value: Self::Value) -> Result<(), Self::Error> {
        if key < N {
            let pre = self.data[key];
            self.data[key] = value;
            if let Some(ref mut h) = self.history {
                h.borrow_mut().status_register(key, pre, value);
            }
            Ok(())
        } else {
            Err(RegisterError::OutOfRange)
        }
    }
}

/// ステータスレジスタの履歴トレイト
pub trait StatusRegisterHistory: History {
    /// ステータスレジスタの変更を記録
    ///
    /// # Arguments
    ///
    /// * `key` - キー
    /// * `pre` - 変更前の値
    /// * `post` - 変更後の値
    fn status_register(&mut self, key: usize, pre: bool, post: bool);
}
