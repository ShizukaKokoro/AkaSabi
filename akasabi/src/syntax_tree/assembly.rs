//! アセンブリ

use std::ops::Index;

/// アセンブリ
#[derive(Debug)]
pub struct Assembly<I> {
    instructions: Vec<I>,
}
impl<Idx, I> Index<Idx> for Assembly<I>
where
    Idx: std::slice::SliceIndex<[I]>,
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.instructions[index]
    }
}
impl<I> Assembly<I> {
    /// 新しいアセンブリを作成する
    pub fn new(instructions: Vec<I>) -> Self {
        Self { instructions }
    }
}
