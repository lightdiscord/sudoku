pub mod entities;
pub mod error;

pub use crate::entities::Sudoku;
use arrayvec::ArrayVec;

pub type Ninerray<T> = ArrayVec<[T; 9]>;
pub type Threerray<T> = ArrayVec<[T; 3]>;

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
        let entry = entry.solve()?;

        let result = "./content/sudoku01/result.txt";
        let result = fetch_sudoku(result)?;

        assert_eq!(entry, result);

        Ok(())
    }

}