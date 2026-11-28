// ==================================================================
// PRACTICE 0281: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0281_parse_robot_commands.md
// Run the tests:          cargo test practice::p0281
// ==================================================================

//! # Problem 0281: Parse Robot Commands
//!
//! - **Difficulty:** Medium
//! - **Topics:** parsing text into `Option<Command>`, slice patterns over words (`["say", rest @ ..]`), guards on slices, `enumerate` for line numbers
//! - **Source:** Original · [The Rust Book 19.3](https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html) · [The Rust Reference: Slice patterns](https://doc.rust-lang.org/reference/patterns.html#slice-patterns)
//!
//! ## Problem
//!
//! A robot is controlled by a small text language, one command per line. Words are separated by
//! whitespace, and keywords are lowercase:
//!
//! | Line | Command |
//! | --- | --- |
//! | `move` | `Move { steps: 1 }` |
//! | `move <n>` with `1 <= n <= 100` | `Move { steps: n }` |
//! | `turn left` / `turn right` | `Turn(Side::Left)` / `Turn(Side::Right)` |
//! | `say <word> <word> ...` (at least one word) | `Say(text)`, the words joined by single spaces |
//! | `wait <seconds>` (any `u32`) | `Wait(seconds)` |
//! | `stop` | `Stop` |
//!
//! Anything else (unknown keyword, missing or extra words, numbers out of range) is invalid.
//!
//! Implement:
//!
//! - `parse_command(line: &str) -> Option<Command>`;
//! - `parse_script(script: &str) -> (Vec<Command>, Vec<usize>)`: parse every line of `script`.
//!   Lines that are empty (after trimming) or start with `#` are skipped silently. Return the valid
//!   commands in order and the **1-based** numbers of the invalid lines.
//!
//! ## Examples
//!
//! ```text
//! parse_command("move 3")          => Some(Move { steps: 3 })
//! parse_command("move")            => Some(Move { steps: 1 })
//! parse_command("move 0")          => None      (out of range)
//! parse_command("turn  left")      => Some(Turn(Left))
//! parse_command("turn back")       => None
//! parse_command("say hello   world") => Some(Say("hello world"))
//! parse_command("say")             => None
//! parse_command("stop now")        => None      (extra word)
//!
//! parse_script("# demo\nmove 2\n\njump\nturn right\nwait x\nstop")
//!   => ([Move { steps: 2 }, Turn(Right), Stop], [4, 6])
//! ```
//!
//! ## Constraints
//!
//! - Scripts have at most `10^4` lines.
//!
//! ## Hints
//!
//! 1. `let words: Vec<&str> = line.split_whitespace().collect();` then `match words.as_slice()`.
//! 2. Arms such as `["turn", "left"]` and `["move", n]` match exact word lists; `["say", rest @ ..]`
//!    binds all remaining words (as a `&[&str]`).
//! 3. A guard can validate a bound value: `["move", n] => n.parse().ok().filter(|s| (1..=100).contains(s))...`.

/// A turning direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

/// One robot command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Move { steps: u32 },
    Turn(Side),
    Say(String),
    Wait(u32),
    Stop,
}

/// Parses one line, or returns `None` if it is not a valid command.
pub fn parse_command(line: &str) -> Option<Command> {
    todo!("Write your solution here")
}

/// Parses a script; returns the commands and the 1-based numbers of invalid lines.
pub fn parse_script(script: &str) -> (Vec<Command>, Vec<usize>) {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::Command::*;
    use super::*;

    #[test]
    fn moves() {
        assert_eq!(parse_command("move 3"), Some(Move { steps: 3 }));
        assert_eq!(parse_command("move"), Some(Move { steps: 1 }));
        assert_eq!(parse_command("move 100"), Some(Move { steps: 100 }));
        assert_eq!(parse_command("move 0"), None);
        assert_eq!(parse_command("move 101"), None);
        assert_eq!(parse_command("move -1"), None);
        assert_eq!(parse_command("move 1 2"), None);
    }

    #[test]
    fn turns_waits_and_stop() {
        assert_eq!(parse_command("turn  left"), Some(Turn(Side::Left)));
        assert_eq!(parse_command("  turn right "), Some(Turn(Side::Right)));
        assert_eq!(parse_command("turn back"), None);
        assert_eq!(parse_command("turn"), None);
        assert_eq!(parse_command("wait 4294967295"), Some(Wait(u32::MAX)));
        assert_eq!(parse_command("wait"), None);
        assert_eq!(parse_command("stop"), Some(Stop));
        assert_eq!(parse_command("stop now"), None);
        assert_eq!(parse_command("STOP"), None);
    }

    #[test]
    fn say_joins_words() {
        assert_eq!(
            parse_command("say hello   world"),
            Some(Say("hello world".to_string()))
        );
        assert_eq!(parse_command("say 42"), Some(Say("42".to_string())));
        assert_eq!(parse_command("say"), None);
        assert_eq!(parse_command(""), None);
    }

    #[test]
    fn script_example() {
        let script = "# demo\nmove 2\n\njump\nturn right\nwait x\nstop";
        assert_eq!(
            parse_script(script),
            (vec![Move { steps: 2 }, Turn(Side::Right), Stop], vec![4, 6])
        );
    }

    #[test]
    fn long_script() {
        let script: String = (0..10_000)
            .map(|i| if i % 10 == 0 { "fly\n" } else { "move 5\n" })
            .collect();
        let (commands, errors) = parse_script(&script);
        assert_eq!(commands.len(), 9_000);
        assert_eq!(errors.len(), 1_000);
        assert_eq!(errors[0], 1);
        assert_eq!(errors[999], 9_991);
    }
}
