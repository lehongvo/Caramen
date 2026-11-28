# Problem 0428: Walk an Error's source Chain

- **Difficulty:** Medium
- **Topics:** layered errors, `Error::source`, `&(dyn Error + 'static)`, `iter::successors`, `enumerate` + `map_err`
- **Source:** Original · [std `Error::source`](https://doc.rust-lang.org/std/error/trait.Error.html#method.source) · [std `iter::successors`](https://doc.rust-lang.org/std/iter/fn.successors.html)

## Problem

Real programs wrap low-level errors in higher-level ones: "could not load `nums.txt`" is caused
by "line 3 is not a number", which is caused by "invalid digit found in string". Each layer
keeps the one below it and exposes it through `Error::source()`, forming a chain that a
reporter can walk.

Implement:

- `Display` and `Error::source` for `LineError` (message `"line N is not a number"`, source:
  the `ParseIntError`) and for `LoadError` (message ``"could not load `FILE`"``, source: the
  `LineError`).
- `load_numbers(file: &str, text: &str) -> Result<Vec<i64>, LoadError>`: every line of `text`,
  trimmed, is parsed as an `i64`. Blank lines are **not** skipped. On the first bad line return a
  `LoadError` for `file` whose source is a `LineError` with the **1-based** line number.
- `error_chain(err: &(dyn Error + 'static)) -> Vec<String>`: the messages of `err`, its source,
  the source's source, and so on.
- `report(err: &(dyn Error + 'static)) -> String`: the chain joined with `": "`.
- `root_cause(err: &(dyn Error + 'static)) -> String`: the message of the last error in the chain.

## Examples

```text
load_numbers("nums.txt", "10\n20\n3x")   -> Err(LoadError { file: "nums.txt", source: LineError { line: 3, .. } })
error_chain(&that_error)  -> ["could not load `nums.txt`", "line 3 is not a number", "invalid digit found in string"]
report(&that_error)       -> "could not load `nums.txt`: line 3 is not a number: invalid digit found in string"
root_cause(&that_error)   -> "invalid digit found in string"
load_numbers("a", "1\n-2") -> Ok([1, -2])
```

## Constraints

- `text` has at most `10^4` lines.

## Hints

1. `fn source(&self) -> Option<&(dyn Error + 'static)> { Some(&self.source) }`
2. `std::iter::successors(Some(err), |&e| e.source())` yields the error and then each source.
3. In `load_numbers`, combine `lines().enumerate()`, `map_err` and `collect::<Result<Vec<_>, _>>()`.

## Starter code

Write your code in [`src/practice/p0428_error_source_chain.rs`](../src/practice/p0428_error_source_chain.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0428
```

```rust
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
```

## Tests

<details>
<summary>Show the tests used to check your solution</summary>

```rust
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
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** each layer stores its cause in a field and returns it from `source()`. Walking the
chain is then a textbook use of `iter::successors`: start with the outer error and repeatedly
ask for `source()` until it returns `None`. `report` and `root_cause` are just two ways of
consuming that iterator.

**Rust concepts:**
- `source()` returns `Option<&(dyn Error + 'static)>`; a `&ParseIntError` or `&LineError`
  coerces to it automatically.
- `successors(first, next)` is a lazy iterator: `next` receives a reference to the previous
  item and returns the following one (or `None` to stop). The items here are already references
  (`&dyn Error`), so the closure gets a `&&dyn Error`; the pattern `|&e|` copies the inner
  reference out, so that `e.source()` borrows from the error itself and not from the short-lived
  outer reference (writing `|e|` fails with "lifetime may not live long enough").
- `Iterator::last()` consumes the iterator and returns its final item.
- Keeping the cause as a typed field (not a `String`) preserves all information for the caller.

```rust
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
        write!(f, "line {} is not a number", self.line)
    }
}

impl Error for LineError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "could not load `{}`", self.file)
    }
}

impl Error for LoadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

pub fn load_numbers(file: &str, text: &str) -> Result<Vec<i64>, LoadError> {
    text.lines()
        .enumerate()
        .map(|(i, line)| {
            line.trim().parse::<i64>().map_err(|source| LoadError {
                file: file.to_string(),
                source: LineError {
                    line: i + 1,
                    source,
                },
            })
        })
        .collect()
}

pub fn error_chain(err: &(dyn Error + 'static)) -> Vec<String> {
    std::iter::successors(Some(err), |&e| e.source())
        .map(|e| e.to_string())
        .collect()
}

pub fn report(err: &(dyn Error + 'static)) -> String {
    error_chain(err).join(": ")
}

pub fn root_cause(err: &(dyn Error + 'static)) -> String {
    std::iter::successors(Some(err), |&e| e.source())
        .last()
        .map(|e| e.to_string())
        .unwrap_or_default()
}
```

Full tested solution: [`src/solutions/p0428_error_source_chain.rs`](../src/solutions/p0428_error_source_chain.rs)

</details>
