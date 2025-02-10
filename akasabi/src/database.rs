//! データベースモジュール

pub mod program_counter;
pub mod register;

use crate::core::Diffs;
use std::{fmt::Debug, hash::Hash};

/// データベーストレイト
pub trait Database<P: Debug + From<Vec<String>>>: Debug + Default {
    /// キーの型
    type Key: Debug + Hash;
    /// 値の型
    type Value: Debug + Clone;
    /// エラーの型
    type Error: Debug;

    /// データを読み込み
    ///
    /// # Arguments
    ///
    /// * `key` - キー
    ///
    /// # Returns
    ///
    /// キーに対応する値
    fn load(&self, key: Self::Key) -> Result<Self::Value, Self::Error>;

    /// データの書き込み
    ///
    /// # Arguments
    ///
    /// * `key` - キー
    /// * `value` - 値
    fn store(
        &mut self,
        key: Self::Key,
        value: Self::Value,
        diffs: &mut Diffs<P, Self::Value>,
    ) -> Result<(), Self::Error>;
}
