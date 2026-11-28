//! # Problem 0421: Parse a Duration like 1h30m
//!
//! - **Difficulty:** Medium
//! - **Topics:** character-by-character parsing, `char::to_digit`, `checked_mul` / `checked_add`, `Option::take`, `?` converting `&str` into `String`
//! - **Source:** Original · [The Rust Book 9.2: Recoverable errors with `Result`](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
//!
//! ## Problem
//!
//! Command-line tools often accept durations such as `1h30m` or `90s`. Write the parser.
//!
//! `parse_duration(s: &str) -> Result<u32, String>` returns the total number of **seconds**.
//! A duration is one or more segments `<number><unit>`, written without spaces, where the number
//! is one or more ASCII digits and the unit is `h` (3600 s), `m` (60 s) or `s` (1 s). Units must
//! appear in the order `h`, `m`, `s`, each at most once, but any of them may be left out.
//!
//! Return these errors (scan from left to right and report the first problem):
//!
//! | problem                                           | error message                    |
//! | ------------------------------------------------- | -------------------------------- |
//! | the input is empty                                | `"empty duration"`               |
//! | a character that is not a digit, `h`, `m` or `s`  | ``"unexpected character `C`"``   |
//! | a unit with no digits right before it             | ``"missing number before `U`"``  |
//! | a unit repeated or after a smaller unit           | ``"unit `U` is out of order"``   |
//! | digits at the end with no unit                    | ``"missing unit after `N`"`` (`N` = the number's value) |
//! | the total (or a number) does not fit in a `u32`   | `"duration too long"`            |
//!
//! Also implement `format_duration(secs: u32) -> String`, the inverse: write the hours, minutes
//! and seconds parts that are not zero, in that order (`3725` -> `"1h2m5s"`, `3600` -> `"1h"`).
//! Zero seconds is written `"0s"`.
//!
//! ## Examples
//!
//! ```text
//! parse_duration("1h30m")  -> Ok(5400)
//! parse_duration("90s")    -> Ok(90)
//! parse_duration("2h5s")   -> Ok(7205)
//! parse_duration("")       -> Err("empty duration")
//! parse_duration("1d")     -> Err("unexpected character `d`")
//! parse_duration("h")      -> Err("missing number before `h`")
//! parse_duration("5m1h")   -> Err("unit `h` is out of order")
//! parse_duration("1h30")   -> Err("missing unit after `30`")
//! format_duration(3725)    -> "1h2m5s"
//! format_duration(0)       -> "0s"
//! ```
//!
//! ## Constraints
//!
//! - `0 <= s.len() <= 100`.
//!
//! ## Hints
//!
//! 1. Keep the digits read so far in an `Option<u32>`: `None` means "no digits since the last unit".
//!    `number.take()` reads it and resets it to `None` in one step.
//! 2. Give each unit a rank (`h` = 1, `m` = 2, `s` = 3) and remember the last rank you saw.
//! 3. `x.checked_mul(10).and_then(|v| v.checked_add(d)).ok_or("duration too long")?` works even
//!    though the function returns `String` errors: `?` converts the `&str` with `From`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** a single left-to-right scan with two pieces of state: the number being read and the
//! rank of the last unit. A digit extends the number; a unit consumes it, checks the order and adds
//! `number * factor` to the total. Every arithmetic step is checked, so a huge input becomes an
//! error instead of a wrong answer.
//!
//! **Rust concepts:**
//! - `char::to_digit(10)` returns `Some(0..=9)` only for ASCII digits.
//! - `Option::take()` moves the value out and leaves `None`, which is exactly "consume the number".
//! - `?` applies `From::from` to the error: `Result<_, &str>` can be propagated from a function
//!   returning `Result<_, String>` because `String: From<&str>`.
//! - `format_duration` builds the text with an array of `(value, unit)` pairs, `filter` and
//!   `collect::<String>()`.
// @solution-end

pub fn parse_duration(s: &str) -> Result<u32, String> {
    // @solution-start
    if s.is_empty() {
        return Err("empty duration".to_string());
    }
    let mut total: u32 = 0;
    let mut number: Option<u32> = None;
    let mut last_rank = 0;
    for c in s.chars() {
        if let Some(d) = c.to_digit(10) {
            let n = number
                .unwrap_or(0)
                .checked_mul(10)
                .and_then(|v| v.checked_add(d))
                .ok_or("duration too long")?;
            number = Some(n);
            continue;
        }
        let (rank, factor) = match c {
            'h' => (1, 3600),
            'm' => (2, 60),
            's' => (3, 1),
            _ => return Err(format!("unexpected character `{c}`")),
        };
        let n = number
            .take()
            .ok_or_else(|| format!("missing number before `{c}`"))?;
        if rank <= last_rank {
            return Err(format!("unit `{c}` is out of order"));
        }
        last_rank = rank;
        total = n
            .checked_mul(factor)
            .and_then(|v| total.checked_add(v))
            .ok_or("duration too long")?;
    }
    match number {
        Some(n) => Err(format!("missing unit after `{n}`")),
        None => Ok(total),
    }
    // @solution-end
}

pub fn format_duration(secs: u32) -> String {
    // @solution-start
    let parts = [
        (secs / 3600, 'h'),
        (secs % 3600 / 60, 'm'),
        (secs % 60, 's'),
    ];
    let text: String = parts
        .iter()
        .filter(|(value, _)| *value > 0)
        .map(|(value, unit)| format!("{value}{unit}"))
        .collect();
    if text.is_empty() {
        "0s".to_string()
    } else {
        text
    }
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_durations() {
        assert_eq!(parse_duration("1h30m"), Ok(5400));
        assert_eq!(parse_duration("90s"), Ok(90));
        assert_eq!(parse_duration("2h5s"), Ok(7205));
        assert_eq!(parse_duration("0s"), Ok(0));
        assert_eq!(parse_duration("1h1m1s"), Ok(3661));
        assert_eq!(parse_duration("007m"), Ok(420));
    }

    #[test]
    fn structural_errors() {
        assert_eq!(parse_duration(""), Err("empty duration".to_string()));
        assert_eq!(
            parse_duration("1d"),
            Err("unexpected character `d`".to_string())
        );
        assert_eq!(
            parse_duration("1h 30m"),
            Err("unexpected character ` `".to_string())
        );
        assert_eq!(
            parse_duration("h"),
            Err("missing number before `h`".to_string())
        );
        assert_eq!(
            parse_duration("1hm"),
            Err("missing number before `m`".to_string())
        );
    }

    #[test]
    fn order_and_trailing_number_errors() {
        assert_eq!(
            parse_duration("5m1h"),
            Err("unit `h` is out of order".to_string())
        );
        assert_eq!(
            parse_duration("1m2m"),
            Err("unit `m` is out of order".to_string())
        );
        assert_eq!(
            parse_duration("1h30"),
            Err("missing unit after `30`".to_string())
        );
        assert_eq!(
            parse_duration("42"),
            Err("missing unit after `42`".to_string())
        );
    }

    #[test]
    fn overflow_is_an_error() {
        assert_eq!(parse_duration("4294967295s"), Ok(u32::MAX));
        assert_eq!(
            parse_duration("4294967296s"),
            Err("duration too long".to_string())
        );
        assert_eq!(
            parse_duration("1193047h"),
            Err("duration too long".to_string())
        );
        assert_eq!(
            parse_duration("1193046h28m16s"),
            Err("duration too long".to_string())
        );
    }

    #[test]
    fn format_and_round_trip() {
        assert_eq!(format_duration(3725), "1h2m5s");
        assert_eq!(format_duration(3600), "1h");
        assert_eq!(format_duration(61), "1m1s");
        assert_eq!(format_duration(0), "0s");
        for secs in [0, 1, 59, 60, 3599, 3600, 86_399, 1_000_000] {
            assert_eq!(parse_duration(&format_duration(secs)), Ok(secs));
        }
    }
}
