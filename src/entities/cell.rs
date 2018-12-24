use crate::entities::values::VALUES;
use crate::entities::{ Value, Sudoku };

use std::collections::HashSet;

#[derive(Debug, Default, Copy, Clone)]
pub struct Cell {
    pub(crate) row: usize,
    pub(crate) column: usize,
    pub(crate) value: Value
}

impl Cell {
    pub fn available(&self, sudoku: &Sudoku) -> Vec<Value> {
        let row = sudoku.row(self.row);
        let column = sudoku.column(self.column);
        let block = sudoku.block(self.row, self.column);

        if self.value != Value::None {
            return vec![self.value];
        }

        let possible: HashSet<Value> = VALUES.iter().cloned().collect();
        let mut values: HashSet<Value> = HashSet::with_capacity(9);

        values.extend(&row);
        values.extend(&column);
        values.extend(&block.values());

        possible
            .difference(&values)
            .cloned()
            .collect()
    }
}
