//! アセンブリ

/// アセンブリ
#[derive(Debug)]
pub struct Assembly<I: Clone> {
    instructions: Vec<I>,
}
impl<I: Clone> Assembly<I> {
    /// 新しいアセンブリを作成する
    pub fn new(instructions: Vec<I>) -> Self {
        Self { instructions }
    }

    /// 命令の取得
    ///
    /// # Arguments
    ///
    /// * `index` - インデックス
    ///
    /// # Returns
    ///
    /// インデックスに対応する命令
    pub fn get(&self, index: usize) -> Option<I> {
        self.instructions.get(index).cloned()
    }
}
