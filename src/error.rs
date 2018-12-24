use failure::Fail;

#[derive(Debug, Fail)]
pub enum SudokuError {
    #[fail(display = "No new cell discovered")]
    NoNewCell
}