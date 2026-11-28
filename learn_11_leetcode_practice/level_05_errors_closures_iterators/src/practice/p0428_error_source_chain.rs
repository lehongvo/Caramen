// ==================================================================
// PRACTICE 0428: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0428_error_source_chain.md
// Run the tests:          cargo test practice::p0428
// ==================================================================

//! # Problem 0428: Walk an Error's source Chain
//!
//! - **Difficulty:** Medium
//! - **Topics:** layered errors, `Error::source`, `&(dyn Error + 'static)`, `iter::successors`, `enumerate` + `map_err`
//! - **Source:** Original · [std `Error::source`](https://doc.rust-lang.org/std/error/trait.Error.html#method.source) · [std `iter::successors`](https://doc.rust-lang.org/std/iter/fn.successors.html)
//!
//! ## Problem
//!
//! Real programs wrap low-level errors in higher-level ones: "could not load `nums.txt`" is caused
//! by "line 3 is not a number", which is caused by "invalid digit found in string". Each layer
//! keeps the one below it and exposes it through `Error::source()`, forming a chain that a
//! reporter can walk.
//!
//! Implement:
//!
//! - `Display` and `Error::source` for `LineError` (message `"line N is not a number"`, source:
//!   the `ParseIntError`) and for `LoadError` (message ``"could not load `FILE`"``, source: the
//!   `LineError`).
//! - `load_numbers(file: &str, text: &str) -> Result<Vec<i64>, LoadError>`: every line of `text`,
//!   trimmed, is parsed as an `i64`. Blank lines are **not** skipped. On the first bad line return a
//!   `LoadError` for `file` whose source is a `LineError` with the **1-based** line number.
//! - `error_chain(err: &(dyn Error + 'static)) -> Vec<String>`: the messages of `err`, its source,
//!   the source's source, and so on.
//! - `report(err: &(dyn Error + 'static)) -> String`: the chain joined with `": "`.
//! - `root_cause(err: &(dyn Error + 'static)) -> String`: the message of the last error in the chain.
//!
//! ## Examples
//!
//! ```text
//! load_numbers("nums.txt", "10\n20\n3x")   -> Err(LoadError { file: "nums.txt", source: LineError { line: 3, .. } })
//! error_chain(&that_error)  -> ["could not load `nums.txt`", "line 3 is not a number", "invalid digit found in string"]
//! report(&that_error)       -> "could not load `nums.txt`: line 3 is not a number: invalid digit found in string"
//! root_cause(&that_error)   -> "invalid digit found in string"
//! load_numbers("a", "1\n-2") -> Ok([1, -2])
//! ```
//!
//! ## Constraints
//!
//! - `text` has at most `10^4` lines.
//!
//! ## Hints
//!
//! 1. `fn source(&self) -> Option<&(dyn Error + 'static)> { Some(&self.source) }`
//! 2. `std::iter::successors(Some(err), |&e| e.source())` yields the error and then each source.
//! 3. In `load_numbers`, combine `lines().enumerate()`, `map_err` and `collect::<Result<Vec<_>, _>>()`.

use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
pub struct LineError {
    pub line: usize,
    pub source: ParseIntError,
}

#[derive(Debug)]
pub struct LoadError {
    pub file: String,
    pub source: LineError,
}

impl fmt::Display for LineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("Write your solution here")
    }
}

impl Error for LineError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        todo!("Write your solution here")
    }
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("Write your solution here")
    }
}

impl Error for LoadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        todo!("Write your solution here")
    }
}

pub fn load_numbers(file: &str, text: &str) -> Result<Vec<i64>, LoadError> {
    todo!("Write your solution here")
}

pub fn error_chain(err: &(dyn Error + 'static)) -> Vec<String> {
    todo!("Write your solution here")
}

pub fn report(err: &(dyn Error + 'static)) -> String {
    todo!("Write your solution here")
}

pub fn root_cause(err: &(dyn Error + 'static)) -> String {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_valid_numbers() {
        assert_eq!(load_numbers("a", "1\n-2").unwrap(), vec![1, -2]);
        assert_eq!(load_numbers("a", " 7 ").unwrap(), vec![7]);
        assert_eq!(load_numbers("a", "").unwrap(), Vec::<i64>::new());
    }

    #[test]
    fn load_error_records_file_and_line() {
        let err = load_numbers("nums.txt", "10\n20\n3x\nalso bad").unwrap_err();
        assert_eq!(err.file, "nums.txt");
        assert_eq!(err.source.line, 3);
        let err = load_numbers("b.txt", "1\n\n2").unwrap_err();
        assert_eq!(err.source.line, 2);
    }

    #[test]
    fn chain_of_three_messages() {
        let err = load_numbers("nums.txt", "10\n20\n3x").unwrap_err();
        assert_eq!(
            error_chain(&err),
            vec![
                "could not load `nums.txt`".to_string(),
                "line 3 is not a number".to_string(),
                "invalid digit found in string".to_string(),
            ]
        );
    }

    #[test]
    fn report_and_root_cause() {
        let err = load_numbers("nums.txt", "10\n20\n3x").unwrap_err();
        assert_eq!(
            report(&err),
            "could not load `nums.txt`: line 3 is not a number: invalid digit found in string"
        );
        assert_eq!(root_cause(&err), "invalid digit found in string");
    }

    #[test]
    fn works_on_any_error() {
        let plain = "".parse::<i32>().unwrap_err();
        assert_eq!(
            error_chain(&plain),
            vec!["cannot parse integer from empty string".to_string()]
        );
        assert_eq!(root_cause(&plain), "cannot parse integer from empty string");
        let line = LineError {
            line: 9,
            source: "z".parse::<i64>().unwrap_err(),
        };
        assert_eq!(
            report(&line),
            "line 9 is not a number: invalid digit found in string"
        );
    }
}
