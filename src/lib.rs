pub mod entities;

use self::entities::values::{ VALUES, Value };

use std::io;

pub const SUDOKU: &'static str = "Sudoku";


#[derive(Debug, Clone, PartialEq)]
pub struct Sudoku {
    grid: [[Value; 9]; 9]
}

#[derive(Debug)]
pub struct Block([[Value; 3]; 3]);

#[derive(Debug, Default, Copy, Clone)]
pub struct Cell {
    row: usize,
    column: usize,
    value: Value
}

impl ::std::str::FromStr for Sudoku {
    type Err = io::Error;

    fn from_str(input: &str) -> io::Result<Self> {
        let mut grid = [[Value::None; 9]; 9];

        let rows = input.lines()
            .take(9)
            .enumerate();

        for (i, row) in rows {

            let columns: io::Result<Vec<Value>> = row.chars()
                .map(|c| Value::from_str(&c.to_string()))
                .collect();

            let mut array = [Value::None; 9];
            array.copy_from_slice(&columns?[..9]);
            grid[i] = array;
        }

        Ok(Sudoku {
            grid
        })
    }
}

impl Sudoku {
    pub fn row(&self, row: usize) -> [Value; 9] {
        self.grid[row]
    }

    pub fn column(&self, column: usize) -> [Value; 9] {
        let columns: Vec<Value> = self.grid.iter()
            .map(|row| row[column])
            .collect();

        let mut array = [Value::None; 9];
        array.copy_from_slice(&columns[..9]);
        array
    }

    pub fn block(&self, row: usize, column: usize) -> Block {
        let row = (row / 3) * 3;
        let column = (column / 3) * 3;

        let rows = &self.grid[row..row+3];

        let rows: Vec<[Value; 3]> = rows.iter()
            .map(|row| {
                let mut array = [Value::None; 3];
                array.copy_from_slice(&row[column..column+3]);
                array
            })
            .collect();

        let mut array = [[Value::None; 3]; 3];
        array.copy_from_slice(&rows[..3]);

        Block(array)
    }

    pub fn cells(&self) -> [Cell; 81] {
        fn map_row(value: (usize, &[Value; 9])) -> Vec<Cell> {
            let (y, row) = value;

            row.iter()
                .enumerate()
                .map(|(x, value)| Cell {
                    row: y,
                    column: x,
                    value: *value
                })
                .collect()
        }

        let cells: Vec<Cell> = self.grid.iter()
            .enumerate()
            .map(map_row)
            .flatten()
            .collect();

        let mut array = [Default::default(); 81];
        array.copy_from_slice(&cells[..81]);
        array
    }

    pub fn remaining(&self) -> usize {
        self.grid.iter()
            .flatten()
            .filter(|&&value| value == Value::None)
            .count()
    }

    pub fn solve(&self) -> Sudoku {
        let mut current = self.clone();

        while current.remaining() > 0 {
            let cells: Vec<Value> = current.cells()
                .iter()
                .map(|&cell| {
                    let available = cell.available(&current);

                    if available.len() == 1 {
                        available[0]
                    } else {
                        cell.value
                    }
                })
                .collect();

            let grid: Vec<[Value; 9]> = cells.chunks(9)
                .map(|row| {
                    let mut array = [Value::None; 9];
                    array.copy_from_slice(&row[..9]);
                    array
                })
                .collect();

            let mut array = [[Value::None; 9]; 9];
            array.copy_from_slice(&grid[..9]);

            current = (Sudoku {
                grid: array
            }).clone();
        }

        current
    }
}

use std::fmt;

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lines = self.grid.iter()
            .map(|line| line.iter().map(|cell| format!("{}", cell)))
            .map(|line| line.collect::<Vec<String>>().join(""))
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", lines)
    }
}

use std::collections::HashSet;

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

impl Block {
    pub fn values(&self) -> [Value; 9] {
        let Block(rows) = self;

        let values: Vec<Value> = rows.iter()
            .flatten()
            .cloned()
            .collect();

        let mut array = [Value::None; 9];
        array.copy_from_slice(&values[..9]);

        array
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Read;
    use std::str::FromStr;
    use failure::Fallible;
    use std::fs::File;
    use std::path::Path;

    fn fetch_sudoku<P: AsRef<Path>>(path: P) -> Fallible<Sudoku> {
        let mut file = File::open(path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        Ok(Sudoku::from_str(&buffer)?)
    }

    #[test]
    fn solve_sudoku01() -> Fallible<()> {
        let entry = "./content/sudoku01/entry.txt";
        let entry = fetch_sudoku(entry)?;
        let entry = entry.solve();

        let result = "./content/sudoku01/result.txt";
        let result = fetch_sudoku(result)?;

        assert_eq!(entry, result);

        Ok(())
    }

}