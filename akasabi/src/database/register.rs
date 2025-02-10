//! レジスタ

use super::*;
use crate::core::*;
use std::fmt::Debug;
use thiserror::Error;

/// レジスタ
///
/// 任意の型を任意の数だけ格納できるレジスタ
#[derive(Debug)]
pub struct Register<T: Debug + Copy + Default, const N: usize> {
    data: [T; N],
}
impl<T: Debug + Copy + Default, const N: usize> Default for Register<T, N> {
    fn default() -> Self {
        Self {
            data: [T::default(); N],
        }
    }
}
impl<P: Debug + From<Vec<String>>, T: Debug + Copy + Default, const N: usize> Database<P>
    for Register<T, N>
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

    fn store(
        &mut self,
        key: Self::Key,
        value: Self::Value,
        diffs: &mut Diffs<P, Self::Value>,
    ) -> Result<(), Self::Error> {
        if key < N {
            let pre = self.data[key];
            self.data[key] = value;
            diffs.push(Diff::new(
                P::from(vec!["Register".to_string(), key.to_string()]),
                pre,
                value,
            ));
            Ok(())
        } else {
            Err(RegisterError::OutOfRange)
        }
    }
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
pub struct StatusRegister<const N: usize> {
    data: [bool; N],
}
impl<const N: usize> Default for StatusRegister<N> {
    fn default() -> Self {
        Self { data: [false; N] }
    }
}
impl<P: Debug + From<Vec<String>>, const N: usize> Database<P> for StatusRegister<N> {
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

    fn store(
        &mut self,
        key: Self::Key,
        value: Self::Value,
        diffs: &mut Diffs<P, Self::Value>,
    ) -> Result<(), Self::Error> {
        if key < N {
            let pre = self.data[key];
            self.data[key] = value;
            diffs.push(Diff::new(
                P::from(vec!["StatusRegister".to_string(), key.to_string()]),
                pre,
                value,
            ));
            Ok(())
        } else {
            Err(RegisterError::OutOfRange)
        }
    }
}
