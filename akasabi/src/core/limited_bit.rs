//! 制限されたビット

/// 制限されたビット
///
/// 命令セットなどに用いられるビットフィールドを表す。
#[derive(Debug)]
pub struct LimitedBit<T, const N: usize>(T);

macro_rules! impl_limited_operator {
    ($($us:ty),*) => {
        $(
            impl<const N: usize> LimitedBit<$us, N> {
                /// コンストラクタ
                ///
                /// 初期値を指定してインスタンスを生成する。
                /// Nビット目より上位のビットは無視される。
                ///
                /// # Arguments
                ///
                /// * `value` - 初期値
                pub fn new(value: $us) -> Self {
                    if std::mem::size_of::<$us>() * 8 <= N {
                        return Self(value);
                    }

                    let mask = (1 << N) - 1;
                    if value & (1 << (N - 1)) != 0 {
                        Self(value | !mask)
                    } else {
                        Self(value & mask)
                    }
                }

                /// 値を取得する
                ///
                /// # Returns
                ///
                /// 値
                pub fn get(&self) -> $us {
                    self.0
                }
            }
        )*
    };
}

impl_limited_operator!(u8, u16, u32, u64, u128);

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case(0b0000_0000, 0b0000_0000)]
    #[case(0b0000_0100, 0b0000_0100)]
    #[case(0b0000_1000, 0b1111_1000)]
    #[case(0b1111_0000, 0b0000_0000)]
    fn test_limited_bit_u8_4bit_new(#[case] value: u8, #[case] expected: u8) {
        let lb = LimitedBit::<u8, 4>::new(value);
        assert_eq!(lb.get(), expected);
    }

    #[test]
    fn test_limited_bit_u8_over_bit_new() {
        let lb = LimitedBit::<u8, 9>::new(0b1111_1111);
        assert_eq!(lb.get(), 0b1111_1111);
    }
}
