//! プログラムカウンター

use crate::{core::*, database::Database};
use std::fmt::Debug;

/// プログラムカウンター
#[derive(Debug, Default)]
pub struct ProgramCounter<T: Debug + Copy + Default>(T);
impl<P: Debug + From<Vec<String>>, T: Debug + Copy + Default> Database<P> for ProgramCounter<T> {
    type Key = ();
    type Value = T;
    type Error = ();

    fn load(&self, _: Self::Key) -> Result<Self::Value, Self::Error> {
        Ok(self.0)
    }

    fn store(
        &mut self,
        _: Self::Key,
        value: Self::Value,
        diffs: &mut Diffs<P, Self::Value>,
    ) -> Result<(), Self::Error> {
        let pre = self.0;
        self.0 = value;
        diffs.push(Diff::new(
            P::from(vec!["ProgramCounter".to_string()]),
            pre,
            value,
        ));
        Ok(())
    }
}
