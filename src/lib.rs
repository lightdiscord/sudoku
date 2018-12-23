use std::io;

pub const SUDOKU: &'static str = "Sudoku";

#[derive(Debug, Copy, Clone)]
pub enum Value {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,

    None
}

impl ::std::str::FromStr for Value {
    type Err = io::Error;

    fn from_str(input: &str) -> io::Result<Self> {
        Ok(match input {
            "1" => Value::One,
            "2" => Value::Two,
            "3" => Value::Three,
            "4" => Value::Four,
            "5" => Value::Five,
            "6" => Value::Six,
            "7" => Value::Seven,
            "8" => Value::Eight,
            "9" => Value::Nine,
            "0" | " " => Value::None,
            _ => {
                return Err(io::ErrorKind::InvalidInput.into());
            },
        })
    }
}

#[derive(Debug)]
pub struct Sudoku {
    grid: [[Value; 9]; 9]
}

impl ::std::str::FromStr for Sudoku {
    type Err = io::Error;

    fn from_str(input: &str) -> io::Result<Self> {
        let mut grid = [[Value::None; 9]; 9];

        let lines = input.lines()
            .take(9)
            .enumerate();

        for (i, line) in lines {

            let columns: io::Result<Vec<Value>> = line.chars()
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