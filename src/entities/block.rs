use crate::{ Threerray, Ninerray };
use crate::entities::Value;

#[derive(Debug)]
pub struct Block {
    pub(crate) grid: Threerray<Threerray<Value>>
}

impl Block {
    pub fn values(&self) -> Ninerray<Value> {
        self.grid.iter()
            .flatten()
            .cloned()
            .collect()
    }
}