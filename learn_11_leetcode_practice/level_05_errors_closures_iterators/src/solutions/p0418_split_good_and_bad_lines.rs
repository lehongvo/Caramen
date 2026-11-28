//! # Problem 0418: Split Good and Bad Lines
//!
//! - **Difficulty:** Easy
//! - **Topics:** `filter_map(Result::ok)`, `Result::err`, `Iterator::partition`, `Result::is_ok`
//! - **Source:** Original · [Rust By Example: Iterating over Results](https://doc.rust-lang.org/rust-by-example/error/iter_result.html)
//!
//! ## Problem
//!
//! Sometimes one bad record should **not** stop the whole job: a log importer keeps the good
//! lines and reports the bad ones. Each line of a score file looks like `name:score`.
//!
//! Implement:
//!
//! - `parse_line(line: &str) -> Result<(String, u32), String>` with these checks, in order:
//!   1. the line must contain `':'`, else ``"`LINE`: missing `:`"``;
//!   2. the name (before the first `':'`, trimmed) must not be empty, else ``"`LINE`: empty name"``;
//!   3. the score (after it, trimmed) must parse as a `u32`, else ``"`LINE`: invalid score"``.
//!
//!   `LINE` is the line exactly as given.
//! - `good_entries(lines: &[&str]) -> Vec<(String, u32)>`: the successfully parsed lines, in order.
//! - `bad_lines(lines: &[&str]) -> Vec<String>`: the error messages of the failing lines, in order.
//! - `split_results(lines: &[&str]) -> (Vec<(String, u32)>, Vec<String>)`: both lists at once,
//!   parsing every line only once.
//!
//! ## Examples
//!
//! ```text
//! lines = ["ann: 90", "bob 75", ":12", "cy:7", "dee:x"]
//! good_entries(lines) -> [("ann", 90), ("cy", 7)]
//! bad_lines(lines)    -> ["`bob 75`: missing `:`", "`:12`: empty name", "`dee:x`: invalid score"]
//! split_results(lines) -> (the two lists above)
//! ```
//!
//! ## Constraints
//!
//! - At most `10^4` lines.
//!
//! ## Hints
//!
//! 1. `Result::ok()` keeps the success value as an `Option`; `Result::err()` keeps the error.
//!    Both are made for `filter_map`.
//! 2. `partition(Result::is_ok)` splits an iterator of `Result`s into two vectors of `Result`s.
//!    You can then unpack each vector with `filter_map` again, without any `unwrap`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** parse every line into a `Result`; then decide what to keep. `filter_map(Result::ok)`
//! keeps the good values, `filter_map(Result::err)` keeps the errors, and `partition` does both in a
//! single pass.
//!
//! **Rust concepts:**
//! - Passing method paths (`Result::ok`, `Result::err`, `Result::is_ok`) instead of closures.
//! - `partition` needs the target collection type, here `(Vec<_>, Vec<_>)`; the items keep their
//!   type (`Result<...>`), they are only distributed into two collections.
//! - After `partition`, every element of the first vector is `Ok`, so `filter_map(Result::ok)`
//!   removes nothing; it just unwraps safely, with no `unwrap()` call that could panic.
// @solution-end

pub fn parse_line(line: &str) -> Result<(String, u32), String> {
    // @solution-start
    let (name, score) = line
        .split_once(':')
        .ok_or_else(|| format!("`{line}`: missing `:`"))?;
    let name = name.trim();
    if name.is_empty() {
        return Err(format!("`{line}`: empty name"));
    }
    let score = score
        .trim()
        .parse::<u32>()
        .map_err(|_| format!("`{line}`: invalid score"))?;
    Ok((name.to_string(), score))
    // @solution-end
}

pub fn good_entries(lines: &[&str]) -> Vec<(String, u32)> {
    // @solution-start
    lines.iter().filter_map(|l| parse_line(l).ok()).collect()
    // @solution-end
}

pub fn bad_lines(lines: &[&str]) -> Vec<String> {
    // @solution-start
    lines.iter().filter_map(|l| parse_line(l).err()).collect()
    // @solution-end
}

pub fn split_results(lines: &[&str]) -> (Vec<(String, u32)>, Vec<String>) {
    // @solution-start
    let (oks, errs): (Vec<_>, Vec<_>) =
        lines.iter().map(|l| parse_line(l)).partition(Result::is_ok);
    let good = oks.into_iter().filter_map(Result::ok).collect();
    let bad = errs.into_iter().filter_map(Result::err).collect();
    (good, bad)
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    const LINES: [&str; 5] = ["ann: 90", "bob 75", ":12", "cy:7", "dee:x"];

    fn entry(name: &str, score: u32) -> (String, u32) {
        (name.to_string(), score)
    }

    #[test]
    fn parse_line_checks() {
        assert_eq!(parse_line("ann: 90"), Ok(entry("ann", 90)));
        assert_eq!(
            parse_line("bob 75"),
            Err("`bob 75`: missing `:`".to_string())
        );
        assert_eq!(parse_line(" :12"), Err("` :12`: empty name".to_string()));
        assert_eq!(
            parse_line("dee:-1"),
            Err("`dee:-1`: invalid score".to_string())
        );
    }

    #[test]
    fn good_entries_keep_order() {
        assert_eq!(good_entries(&LINES), vec![entry("ann", 90), entry("cy", 7)]);
        assert_eq!(good_entries(&[]), vec![]);
    }

    #[test]
    fn bad_lines_keep_order() {
        assert_eq!(
            bad_lines(&LINES),
            vec![
                "`bob 75`: missing `:`".to_string(),
                "`:12`: empty name".to_string(),
                "`dee:x`: invalid score".to_string(),
            ]
        );
    }

    #[test]
    fn split_results_matches_both_lists() {
        let (good, bad) = split_results(&LINES);
        assert_eq!(good, good_entries(&LINES));
        assert_eq!(bad, bad_lines(&LINES));
        assert_eq!(good.len() + bad.len(), LINES.len());
    }

    #[test]
    fn split_all_good_or_all_bad() {
        assert_eq!(
            split_results(&["a:1", "b:2"]),
            (vec![entry("a", 1), entry("b", 2)], vec![])
        );
        let (good, bad) = split_results(&["a", "b"]);
        assert!(good.is_empty());
        assert_eq!(bad.len(), 2);
    }
}
