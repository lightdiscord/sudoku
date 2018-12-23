use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

use failure::Fallible;

use sudoku::Sudoku;

fn main() -> Fallible<()> {
    println!("Hello, world! Here is a {} crate!", sudoku::SUDOKU);

    let mut file = File::open("./content/sudoku01/entry.txt")?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;

    println!("{:#?}", Sudoku::from_str(&buffer)?);

    Ok(())
}
