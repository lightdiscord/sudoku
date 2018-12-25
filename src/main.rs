use std::io::{ self, Read };
use std::str::FromStr;
use failure::Fallible;
use sudoku::Sudoku;

fn main() -> Fallible<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let sudoku = Sudoku::from_str(&buffer)?;
    let sudoku = sudoku.solve()?;

    println!("{}", sudoku);

    Ok(())
}
