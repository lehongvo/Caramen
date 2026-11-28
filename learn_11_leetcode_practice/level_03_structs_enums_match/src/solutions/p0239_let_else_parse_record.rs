//! # Problem 0239: Parse Records with let else
//!
//! - **Difficulty:** Easy
//! - **Topics:** `let ... else` with early return, refutable patterns, `split_once`, `parse::<u8>().ok()`, building a struct
//! - **Source:** Original · [The Rust Book 6.3](https://doc.rust-lang.org/book/ch06-03-if-let.html#staying-on-the-happy-path-with-letelse) · [Rust by Example: let-else](https://doc.rust-lang.org/rust-by-example/flow_control/let_else.html)
//!
//! ## Problem
//!
//! A sign-up form is exported as text lines `"name,age,email"`. Parse one line into
//!
//! ```text
//! struct Record { name: String, age: u8, email: String }
//! ```
//!
//! with `parse_record(line: &str) -> Option<Record>`, which returns `None` unless **all** of these hold:
//!
//! 1. the line has exactly three comma-separated fields;
//! 2. the name, after trimming spaces, is not empty (store it trimmed);
//! 3. the age, after trimming, parses as a `u8` (`0..=255`);
//! 4. the email, after trimming, contains exactly one `'@'`, with at least one character before it
//!    and at least one `'.'` somewhere after it (store it trimmed).
//!
//! Then write `parse_all(lines: &[&str]) -> (Vec<Record>, usize)`: the valid records in input order
//! and the number of rejected lines.
//!
//! ## Examples
//!
//! ```text
//! parse_record("Ana, 34, ana@example.com")  => Some(Record { name: "Ana", age: 34, email: "ana@example.com" })
//! parse_record("Bo,abc,bo@x.io")            => None   (age is not a number)
//! parse_record("Cy,300,cy@x.io")            => None   (300 does not fit in a u8)
//! parse_record(" ,20,d@x.io")               => None   (empty name)
//! parse_record("Ed,20,ed@@x.io")            => None   (two '@')
//! parse_record("Fy,20,fy@localhost")        => None   (no '.' after '@')
//! parse_record("Gus,20")                    => None   (only two fields)
//! ```
//!
//! ## Constraints
//!
//! - Lines are ASCII, at most 200 characters; at most `10^4` lines.
//!
//! ## Hints
//!
//! 1. `let [name, age, email] = fields[..] else { return None; };` checks the field count and names
//!    the parts in one statement (collect the `split(',')` result into a `Vec<&str>` first).
//! 2. `let Ok(age) = age.trim().parse::<u8>() else { return None; };` keeps going only on success.
//! 3. `email.split_once('@')` gives `Some((before, after))`; then check that `after` has no other `'@'`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** validate one rule per statement. Each `let ... else` either binds the next piece of
//! data or leaves the function with `None`, so the code reads top to bottom without nesting.
//!
//! **Rust concepts:**
//! - `let PATTERN = EXPR else { DIVERGE };` (stable since Rust 1.65) binds the variables of a
//!   **refutable** pattern for the rest of the block; the `else` block must leave (`return`,
//!   `break`, `continue` or panic).
//! - A slice pattern `[a, b, c]` only matches a slice of length exactly 3.
//! - When the pattern is `Some(x)` and the `else` block is just `return None`, the `?` operator is
//!   shorter, and clippy suggests it: `let (user, domain) = email.split_once('@')?;`.
//! - `split_once('@')` splits at the first `'@'`; any other `'@'` would remain in the second part.
//! - `parse_all` uses `match` on the `Option` to sort lines into "kept" and "rejected".
// @solution-end

/// One valid sign-up record.
#[derive(Debug, Clone, PartialEq)]
pub struct Record {
    pub name: String,
    pub age: u8,
    pub email: String,
}

/// Parses `"name,age,email"`, or returns `None` if any rule fails.
pub fn parse_record(line: &str) -> Option<Record> {
    // @solution-start
    let fields: Vec<&str> = line.split(',').collect();
    let [name, age, email] = fields[..] else {
        return None;
    };
    let name = name.trim();
    if name.is_empty() {
        return None;
    }
    let Ok(age) = age.trim().parse::<u8>() else {
        return None;
    };
    let email = email.trim();
    // For an `Option` whose `else` would only `return None`, `?` says the same thing.
    let (user, domain) = email.split_once('@')?;
    if user.is_empty() || domain.contains('@') || !domain.contains('.') {
        return None;
    }
    Some(Record {
        name: name.to_string(),
        age,
        email: email.to_string(),
    })
    // @solution-end
}

/// The valid records and the number of rejected lines.
pub fn parse_all(lines: &[&str]) -> (Vec<Record>, usize) {
    // @solution-start
    let mut records = Vec::new();
    let mut rejected = 0;
    for line in lines {
        match parse_record(line) {
            Some(r) => records.push(r),
            None => rejected += 1,
        }
    }
    (records, rejected)
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rec(name: &str, age: u8, email: &str) -> Record {
        Record {
            name: name.to_string(),
            age,
            email: email.to_string(),
        }
    }

    #[test]
    fn valid_line_is_trimmed() {
        assert_eq!(
            parse_record("Ana, 34, ana@example.com"),
            Some(rec("Ana", 34, "ana@example.com"))
        );
        assert_eq!(
            parse_record("  Li Wei ,0,l@w.cn"),
            Some(rec("Li Wei", 0, "l@w.cn"))
        );
    }

    #[test]
    fn bad_ages() {
        assert_eq!(parse_record("Bo,abc,bo@x.io"), None);
        assert_eq!(parse_record("Cy,300,cy@x.io"), None);
        assert_eq!(parse_record("Cy,-1,cy@x.io"), None);
        assert_eq!(
            parse_record("Cy,255,cy@x.io"),
            Some(rec("Cy", 255, "cy@x.io"))
        );
    }

    #[test]
    fn bad_names_and_field_counts() {
        assert_eq!(parse_record(" ,20,d@x.io"), None);
        assert_eq!(parse_record("Gus,20"), None);
        assert_eq!(parse_record("Gus,20,g@x.io,extra"), None);
        assert_eq!(parse_record(""), None);
    }

    #[test]
    fn bad_emails() {
        assert_eq!(parse_record("Ed,20,ed@@x.io"), None);
        assert_eq!(parse_record("Fy,20,fy@localhost"), None);
        assert_eq!(parse_record("Hi,20,@x.io"), None);
        assert_eq!(parse_record("Hi,20,hi.x.io"), None);
        assert_eq!(
            parse_record("Hi,20,a.b@c.d"),
            Some(rec("Hi", 20, "a.b@c.d"))
        );
    }

    #[test]
    fn parse_many_lines() {
        let lines = [
            "Ana,34,ana@example.com",
            "broken",
            "Bo,9,bo@x.io",
            "Cy,x,c@x.io",
        ];
        let (records, rejected) = parse_all(&lines);
        assert_eq!(
            records,
            vec![rec("Ana", 34, "ana@example.com"), rec("Bo", 9, "bo@x.io")]
        );
        assert_eq!(rejected, 2);
        assert_eq!(parse_all(&[]), (vec![], 0));
    }

    #[test]
    fn ten_thousand_lines() {
        let owned: Vec<String> = (0..10_000)
            .map(|i| format!("u{i},{},u{i}@m.org", i % 300))
            .collect();
        let lines: Vec<&str> = owned.iter().map(String::as_str).collect();
        let (records, rejected) = parse_all(&lines);
        // Ages 256..=299 are rejected: 44 out of every 300 lines.
        let expected_rejected = (0..10_000).filter(|i| i % 300 > 255).count();
        assert_eq!(rejected, expected_rejected);
        assert_eq!(records.len() + rejected, 10_000);
    }
}
