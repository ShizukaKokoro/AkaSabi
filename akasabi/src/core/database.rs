//! データベースに関わるモジュール

use std::fmt::Debug;

use std::hash::Hash;

/// データベーストレイト
pub trait Database: Debug {
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
    fn store(&mut self, key: Self::Key, value: Self::Value) -> Result<(), Self::Error>;
}

/// 履歴トレイト
///
/// 全てのデータベースは履歴を保存できるようにするべき。
pub trait History: Debug + Default {}

/// 差分
#[derive(Debug, Clone)]
pub struct Diff<K: Debug + Hash, V: Debug + Clone> {
    /// キー
    key: K,
    /// 前の値
    pre: V,
    /// 後の値
    post: V,
}
impl<K: Debug + Hash, V: Debug + Clone> Diff<K, V> {
    /// 新しい差分を作成
    pub fn new(key: K, pre: V, post: V) -> Self {
        Self { key, pre, post }
    }

    /// キーを取得
    pub fn key(&self) -> &K {
        &self.key
    }

    /// 前の値を取得
    pub fn pre(&self) -> V {
        self.pre.clone()
    }

    /// 後の値を取得
    pub fn post(&self) -> V {
        self.post.clone()
    }
}
