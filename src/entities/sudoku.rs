use crate::Ninerray;
use crate::entities::{ Value, Cell, Block };
use crate::error::SudokuError;

use failure::Fallible;
use arrayvec::ArrayVec;

#[derive(Debug, Clone, PartialEq)]
pub struct Sudoku {
    grid: Ninerray<Ninerray<Value>>
}

impl ::std::str::FromStr for Sudoku {
    type Err = failure::Error;

    fn from_str(input: &str) -> Fallible<Self> {
        fn map_row(row: &str) -> Fallible<Ninerray<Value>> {
            row.chars()
                .map(|c| Ok(Value::from_str(&c.to_string())?))
                .collect()
        }

        let grid: Fallible<_> = input.lines()
            .take(9)
            .map(map_row)
            .collect();

        Ok(Sudoku { grid: grid? })
    }
}

impl Sudoku {
    pub fn row(&self, row: usize) -> Ninerray<Value> {
        self.grid[row].clone()
    }

    pub fn column(&self, column: usize) -> Ninerray<Value> {
        self.grid.iter()
            .map(|row| row[column])
            .collect()
    }

    pub fn block(&self, row: usize, column: usize) -> Block {
        let row = (row / 3) * 3;
        let column = (column / 3) * 3;

        let grid = &self.grid[row..row + 3];
        let grid = grid.iter()
            .map(|row| row[column..column + 3].iter().cloned().collect())
            .collect();

        Block {
            grid
        }
    }

    pub fn cells(&self) -> ArrayVec<[Cell; 81]> {
        fn map_row((row, rows): (usize, &Ninerray<Value>)) -> Ninerray<Cell> {
            rows.iter()
                .enumerate()
                .map(|(column, &value)| Cell { row, column, value })
                .collect()
        }

        self.grid.iter()
            .enumerate()
            .map(map_row)
            .flatten()
            .collect()
    }

    pub fn remaining(&self) -> usize {
        self.grid.iter()
            .flatten()
            .filter(|&&value| value == Value::None)
            .count()
    }

    pub fn solve(&self) -> Fallible<Sudoku> {
        let sudoku = self.clone();

        let grid = sudoku.cells()
            .iter()
            .map(|&cell| {
                let available = cell.available(&sudoku);
                if available.len() == 1 { available[0] } else { cell.value }
            })
            .collect::<ArrayVec<[_; 81]>>()
            .chunks(9)
            .map(|row| row.iter().cloned().collect())
            .collect();

        let new = Sudoku { grid };

        if new == sudoku {
            Err(SudokuError::NoNewCell.into())
        } else if new.remaining() > 0 {
            new.solve()
        } else {
            Ok(new)
        }
    }
}

use std::fmt;

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lines = self.grid.iter()
            .map(|line| line.iter().map(ToString::to_string))
            .map(|line| line.collect::<Vec<String>>().join(""))
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", lines)
    }
}