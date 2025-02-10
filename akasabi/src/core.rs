//! コアモジュール

use std::fmt::Debug;

/// 差分
///
/// データベースを変更する操作が行われた際の差分
#[derive(Debug)]
pub struct Diff<P: Debug + From<Vec<String>>, T: Debug> {
    pos: P,
    pre: T,
    post: T,
}
impl<P: Debug + From<Vec<String>>, T: Debug> Diff<P, T> {
    /// 差分を作成
    pub fn new(pos: P, pre: T, post: T) -> Self {
        Self { pos, pre, post }
    }

    /// 差分の位置を取得
    ///
    /// # Returns
    ///
    /// 差分の位置
    pub fn pos(&self) -> &P {
        &self.pos
    }

    /// 差分の前の値を取得
    ///
    /// # Returns
    ///
    /// 差分の前の値
    pub fn pre(&self) -> &T {
        &self.pre
    }

    /// 差分の後の値を取得
    ///
    /// # Returns
    ///
    /// 差分の後の値
    pub fn post(&self) -> &T {
        &self.post
    }
}

/// 差分群
#[derive(Debug)]
pub struct Diffs<P: Debug + From<Vec<String>>, T: Debug> {
    diffs: Vec<Diff<P, T>>,
}
impl<P: Debug + From<Vec<String>>, T: Debug> Diffs<P, T> {
    /// 差分を追加
    pub fn push(&mut self, diff: Diff<P, T>) {
        self.diffs.push(diff);
    }
}
impl<P: Debug + From<Vec<String>>, T: Debug> Default for Diffs<P, T> {
    fn default() -> Self {
        Self { diffs: Vec::new() }
    }
}
impl<P: Debug + From<Vec<String>>, T: Debug> Diffs<P, T> {
    /// 差分群のイテレーターを取得
    ///
    /// # Returns
    ///
    /// 差分群のイテレーター
    pub fn iter(&self) -> impl Iterator<Item = &Diff<P, T>> {
        self.diffs.iter()
    }
}
