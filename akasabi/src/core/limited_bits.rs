//! 制限されたビット数の整数型

use std::fmt::Debug;

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
