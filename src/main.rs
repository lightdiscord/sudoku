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

    let ku = Sudoku::from_str(&buffer)?;

    println!("{:#?}", ku.row(0));
    println!("{:#?}", ku.column(0));
    println!("{:#?}", ku.block(0, 0).values());
    println!("{}", ku.solve());

    Ok(())
}
