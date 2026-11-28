# Problem 0429: Parse a CSV Record into a Struct

- **Difficulty:** Medium
- **Topics:** error enum with data, slice patterns with `let ... else`, generic helper over `FromStr`, `lines().enumerate().skip(1)`, collecting `Result`s
- **Source:** Original · [std `str::FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) · [The Rust Book 9.2](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)

## Problem

A CSV export of employees looks like this:

```text
name,age,email,salary
Ann Lee, 34, ann@corp.com, 72000
Bob Stone,29,bob@corp.com,58000
```

Implement `parse_record(line: &str) -> Result<Employee, RecordError>` for one data line.
Split on `','` and trim every field. Then check, **in this order**:

1. there are exactly 4 fields, else `WrongFieldCount { expected: 4, found }`;
2. `name` is not empty, else `EmptyField("name")`;
3. `age` is not empty (`EmptyField("age")`) and parses as a `u8`, else
   `InvalidNumber { field: "age", value }`;
4. `email` is not empty (`EmptyField("email")`) and contains exactly one `'@'` with a non-empty
   part on each side, else `InvalidEmail(email)`;
5. `salary` is not empty (`EmptyField("salary")`) and parses as a `u32`, else
   `InvalidNumber { field: "salary", value }`.

Then implement `parse_csv(text: &str) -> Result<Vec<Employee>, (usize, RecordError)>`: the first
line is a header and is always skipped; blank lines are skipped; every other line is parsed with
`parse_record`. The error carries the **1-based line number** in `text`.

Finally implement `Display` for `RecordError`:

```text
WrongFieldCount { expected: 4, found: 3 }          -> "expected 4 fields, found 3"
EmptyField("email")                                -> "field `email` is empty"
InvalidNumber { field: "age", value: "old" }       -> "field `age` has invalid number `old`"
InvalidEmail("ann.corp.com")                       -> "`ann.corp.com` is not a valid email"
```

## Examples

```text
parse_record("Ann Lee, 34, ann@corp.com, 72000")
    -> Ok(Employee { name: "Ann Lee", age: 34, email: "ann@corp.com", salary: 72000 })
parse_record("Ann, 34, ann@corp.com")  -> Err(WrongFieldCount { expected: 4, found: 3 })
parse_record("Ann, 300, a@b.c, 1")     -> Err(InvalidNumber { field: "age", value: "300" })
parse_record("Ann, 34, a@@b.c, 1")     -> Err(InvalidEmail("a@@b.c"))
parse_csv("name,age,email,salary\nAnn,34,a@b.c,1\nBob,x,b@c.d,2")
    -> Err((3, InvalidNumber { field: "age", value: "x" }))
```

## Constraints

- Fields never contain quoted commas. At most `10^4` lines.

## Hints

1. `let &[name, age, email, salary] = fields.as_slice() else { return Err(...) };`
2. A generic helper `fn number<T: FromStr>(field: &'static str, value: &str) -> Result<T, RecordError>`
   can parse both the `u8` and the `u32`.
3. In `parse_csv`, `enumerate()` *before* `skip(1)` so that the indexes still match the lines.

## Starter code

Write your code in [`src/practice/p0429_parse_csv_record.rs`](../src/practice/p0429_parse_csv_record.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0429
```

```rust
use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Employee {
    pub name: String,
    pub age: u8,
    pub email: String,
    pub salary: u32,
}

#[derive(Debug, PartialEq)]
pub enum RecordError {
    WrongFieldCount { expected: usize, found: usize },
    EmptyField(&'static str),
    InvalidNumber { field: &'static str, value: String },
    InvalidEmail(String),
}

impl fmt::Display for RecordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("Write your solution here")
    }
}

impl Error for RecordError {}

pub fn parse_record(line: &str) -> Result<Employee, RecordError> {
    todo!("Write your solution here")
}

pub fn parse_csv(text: &str) -> Result<Vec<Employee>, (usize, RecordError)> {
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

    fn ann() -> Employee {
        Employee {
            name: "Ann Lee".to_string(),
            age: 34,
            email: "ann@corp.com".to_string(),
            salary: 72_000,
        }
    }

    #[test]
    fn valid_record() {
        assert_eq!(parse_record("Ann Lee, 34, ann@corp.com, 72000"), Ok(ann()));
        assert_eq!(parse_record("Ann Lee,34,ann@corp.com,72000"), Ok(ann()));
    }

    #[test]
    fn shape_and_empty_fields() {
        assert_eq!(
            parse_record("Ann, 34, ann@corp.com"),
            Err(RecordError::WrongFieldCount {
                expected: 4,
                found: 3
            })
        );
        assert_eq!(
            parse_record(""),
            Err(RecordError::WrongFieldCount {
                expected: 4,
                found: 1
            })
        );
        assert_eq!(
            parse_record(" , 34, a@b.c, 1"),
            Err(RecordError::EmptyField("name"))
        );
        assert_eq!(
            parse_record("Ann, , a@b.c, 1"),
            Err(RecordError::EmptyField("age"))
        );
        assert_eq!(
            parse_record("Ann, 3, , 1"),
            Err(RecordError::EmptyField("email"))
        );
        assert_eq!(
            parse_record("Ann, 3, a@b.c, "),
            Err(RecordError::EmptyField("salary"))
        );
    }

    #[test]
    fn invalid_values() {
        assert_eq!(
            parse_record("Ann, 300, a@b.c, 1"),
            Err(RecordError::InvalidNumber {
                field: "age",
                value: "300".to_string()
            })
        );
        assert_eq!(
            parse_record("Ann, 34, a@@b.c, 1"),
            Err(RecordError::InvalidEmail("a@@b.c".to_string()))
        );
        assert_eq!(
            parse_record("Ann, 34, @b.c, 1"),
            Err(RecordError::InvalidEmail("@b.c".to_string()))
        );
        assert_eq!(
            parse_record("Ann, 34, a@b.c, -1"),
            Err(RecordError::InvalidNumber {
                field: "salary",
                value: "-1".to_string()
            })
        );
    }

    #[test]
    fn display_messages() {
        let cases = [
            (
                RecordError::WrongFieldCount {
                    expected: 4,
                    found: 3,
                },
                "expected 4 fields, found 3",
            ),
            (RecordError::EmptyField("email"), "field `email` is empty"),
            (
                RecordError::InvalidNumber {
                    field: "age",
                    value: "old".to_string(),
                },
                "field `age` has invalid number `old`",
            ),
            (
                RecordError::InvalidEmail("ann.corp.com".to_string()),
                "`ann.corp.com` is not a valid email",
            ),
        ];
        for (err, message) in cases {
            assert_eq!(err.to_string(), message);
        }
    }

    #[test]
    fn whole_file() {
        let text = "name,age,email,salary\nAnn Lee, 34, ann@corp.com, 72000\n\nBob,29,bob@corp.com,58000\n";
        let all = parse_csv(text).unwrap();
        assert_eq!(all.len(), 2);
        assert_eq!(all[0], ann());
        assert_eq!(all[1].salary, 58_000);
        assert_eq!(
            parse_csv("name,age,email,salary\nAnn,34,a@b.c,1\nBob,x,b@c.d,2"),
            Err((
                3,
                RecordError::InvalidNumber {
                    field: "age",
                    value: "x".to_string()
                }
            ))
        );
        assert_eq!(parse_csv("header only"), Ok(vec![]));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** split, check the shape with a slice pattern, then validate the fields one by one with
two tiny helpers (`required` for emptiness, `number` for parsing). Every failure maps to a
precise variant carrying the data a caller needs. `parse_csv` attaches line numbers with
`map_err` and collects into a `Result`, which stops at the first bad line.

**Rust concepts:**
- Error variants with data (`InvalidNumber { field, value }`) make errors both matchable and
  informative; `&'static str` is enough for field names known at compile time.
- A generic function bounded by `T: FromStr` works for every number type.
- `matches!(email.split_once('@'), Some((user, domain)) if ...)` tests a pattern with a guard.
- The order of `enumerate` and `skip` matters: `enumerate().skip(1)` keeps original indexes.

```rust
use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Employee {
    pub name: String,
    pub age: u8,
    pub email: String,
    pub salary: u32,
}

#[derive(Debug, PartialEq)]
pub enum RecordError {
    WrongFieldCount { expected: usize, found: usize },
    EmptyField(&'static str),
    InvalidNumber { field: &'static str, value: String },
    InvalidEmail(String),
}

impl fmt::Display for RecordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RecordError::WrongFieldCount { expected, found } => {
                write!(f, "expected {expected} fields, found {found}")
            }
            RecordError::EmptyField(field) => write!(f, "field `{field}` is empty"),
            RecordError::InvalidNumber { field, value } => {
                write!(f, "field `{field}` has invalid number `{value}`")
            }
            RecordError::InvalidEmail(email) => write!(f, "`{email}` is not a valid email"),
        }
    }
}

impl Error for RecordError {}

fn required<'a>(field: &'static str, value: &'a str) -> Result<&'a str, RecordError> {
    if value.is_empty() {
        Err(RecordError::EmptyField(field))
    } else {
        Ok(value)
    }
}

fn number<T: FromStr>(field: &'static str, value: &str) -> Result<T, RecordError> {
    required(field, value)?
        .parse::<T>()
        .map_err(|_| RecordError::InvalidNumber {
            field,
            value: value.to_string(),
        })
}

pub fn parse_record(line: &str) -> Result<Employee, RecordError> {
    let fields: Vec<&str> = line.split(',').map(str::trim).collect();
    let &[name, age, email, salary] = fields.as_slice() else {
        return Err(RecordError::WrongFieldCount {
            expected: 4,
            found: fields.len(),
        });
    };
    let name = required("name", name)?;
    let age: u8 = number("age", age)?;
    let email = required("email", email)?;
    let valid_email = email.matches('@').count() == 1
        && matches!(email.split_once('@'), Some((user, domain)) if !user.is_empty() && !domain.is_empty());
    if !valid_email {
        return Err(RecordError::InvalidEmail(email.to_string()));
    }
    let salary: u32 = number("salary", salary)?;
    Ok(Employee {
        name: name.to_string(),
        age,
        email: email.to_string(),
        salary,
    })
}

pub fn parse_csv(text: &str) -> Result<Vec<Employee>, (usize, RecordError)> {
    text.lines()
        .enumerate()
        .skip(1)
        .filter(|(_, line)| !line.trim().is_empty())
        .map(|(i, line)| parse_record(line).map_err(|e| (i + 1, e)))
        .collect()
}
```

Full tested solution: [`src/solutions/p0429_parse_csv_record.rs`](../src/solutions/p0429_parse_csv_record.rs)

</details>
