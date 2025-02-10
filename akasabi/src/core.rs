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

/// 制限された u8
///
/// N bit のみが有効な u8
#[derive(Clone, Copy)]
pub struct LimitedU8<const N: usize>(u8);
impl<const N: usize> LimitedU8<N> {
    /// 制限された数値を作成
    ///
    /// # Arguments
    ///
    /// * `n` - 数値
    ///
    /// # Returns
    ///
    /// 制限された数値
    pub fn new(n: u8) -> Self {
        Self(n & ((1 << N) - 1))
    }

    /// 数値を取得
    ///
    /// # Returns
    ///
    /// 数値
    pub fn get(&self) -> u8 {
        self.0
    }
}
impl<const N: usize> Debug for LimitedU8<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#0width$b}", self.0, width = N)
    }
}
impl<const N: usize> From<u8> for LimitedU8<N> {
    fn from(n: u8) -> Self {
        Self::new(n)
    }
}
impl<const N: usize> From<LimitedU8<N>> for u8 {
    fn from(n: LimitedU8<N>) -> Self {
        n.0
    }
}

/// 制限された u16
///
/// N bit のみが有効な u16
#[derive(Clone, Copy)]
pub struct LimitedU16<const N: usize>(u16);
impl<const N: usize> LimitedU16<N> {
    /// 制限された数値を作成
    ///
    /// # Arguments
    ///
    /// * `n` - 数値
    ///
    /// # Returns
    ///
    /// 制限された数値
    pub fn new(n: u16) -> Self {
        Self(n & ((1 << N) - 1))
    }

    /// 数値を取得
    ///
    /// # Returns
    ///
    /// 数値
    pub fn get(&self) -> u16 {
        self.0
    }
}
impl<const N: usize> Debug for LimitedU16<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#0width$b}", self.0, width = N)
    }
}
impl<const N: usize> From<u16> for LimitedU16<N> {
    fn from(n: u16) -> Self {
        Self::new(n)
    }
}
impl<const N: usize> From<LimitedU16<N>> for u16 {
    fn from(n: LimitedU16<N>) -> Self {
        n.0
    }
}
