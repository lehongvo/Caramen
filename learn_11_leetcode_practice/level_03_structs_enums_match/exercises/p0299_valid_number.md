# Problem 0299: Valid Number

- **Difficulty:** Hard
- **Topics:** finite state machine with an `enum`, classifying characters into an `enum`, `match (state, class)` transition table, accepting states with `matches!`
- **Source:** [LeetCode 65. Valid Number](https://leetcode.com/problems/valid-number/)

## Problem

Decide whether the string `s` is a valid number. A valid number is

- an **integer** or a **decimal**, optionally followed by an **exponent**: `e` or `E` and then
  an integer.

where

- an **integer** is an optional sign (`+` or `-`) followed by one or more digits;
- a **decimal** is an optional sign followed by one of: digits then `.` (`"4."`), digits then `.`
  then digits (`"4.5"`), or `.` then digits (`".5"`).

Nothing else is allowed: no spaces, no letters other than one `e`/`E`.

Implement `Solution::is_number(s: String) -> bool`.

## Examples

```text
Valid:   "2", "0089", "-0.1", "+3.14", "4.", "-.9", "2e10", "-90E3", "3e+7", "+6e-1", "53.5e93", "-123.456e789"
Invalid: "abc", "1a", "1e", "e3", "99e2.5", "--6", "-+3", "95a54e53", ".", "+", "1e+"
```

## Constraints

- `1 <= s.len() <= 20`; `s` contains English letters, digits, `+`, `-` and `.`.

## Hints

1. Classify each character first: `enum Class { Digit, Sign, Dot, Exp, Other }`.
2. Draw the states: start, after a sign, in the integer digits, after a dot with or without
   digits before it, in the fraction digits, after `e`, after the exponent sign, in the exponent
   digits. List the allowed transitions; everything else rejects.
3. Only some states may end the string: those right after at least one digit of the mantissa
   (or `"4."`), and the exponent digits.

## Starter code

Write your code in [`src/practice/p0299_valid_number.rs`](../src/practice/p0299_valid_number.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0299
```

```rust
pub struct Solution;

impl Solution {
    pub fn is_number(s: String) -> bool {
        todo!("Write your solution here")
    }
}
```

## Tests

<details>
<summary>Show the tests used to check your solution</summary>

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn valid(s: &str) -> bool {
        Solution::is_number(s.to_string())
    }

    #[test]
    fn valid_examples() {
        for s in [
            "2",
            "0089",
            "-0.1",
            "+3.14",
            "4.",
            "-.9",
            "2e10",
            "-90E3",
            "3e+7",
            "+6e-1",
            "53.5e93",
            "-123.456e789",
        ] {
            assert!(valid(s), "{s} should be valid");
        }
    }

    #[test]
    fn invalid_examples() {
        for s in [
            "abc", "1a", "1e", "e3", "99e2.5", "--6", "-+3", "95a54e53", ".", "+", "1e+",
        ] {
            assert!(!valid(s), "{s} should be invalid");
        }
    }

    #[test]
    fn dots_and_exponents() {
        assert!(valid(".5"));
        assert!(valid("5.e3"));
        assert!(!valid(".e3"));
        assert!(!valid("5..3"));
        assert!(!valid("5e3e3"));
        assert!(!valid("5e3.0"));
        assert!(!valid("1.2.3"));
    }

    #[test]
    fn signs_only_at_the_start_of_parts() {
        assert!(!valid("1+2"));
        assert!(!valid("+-1"));
        assert!(valid("-1E-0"));
        assert!(!valid("1e--1"));
        assert!(!valid("-"));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** a deterministic finite automaton. Each state remembers exactly what has been read so
far; each character class either moves to a new state or rejects. At the end, the input is
valid if the automaton stopped in an accepting state.

**Complexity:** `O(n)` time, `O(1)` space.

**Rust concepts:**
- Two small enums (`State`, `Class`) and one `match (state, class)` form a readable transition
  table; rows the table does not mention fall into `_ => return false`.
- Alternatives in a tuple pattern (`(State::Start | State::Sign, Class::Digit)`) merge rows with
  the same target.
- `matches!(state, State::Int | State::DotAfterInt | State::Frac | State::ExpInt)` lists the
  accepting states.
- `try_fold` could replace the loop, but an explicit `for` with early `return false` is clearer.

```rust
pub struct Solution;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Class {
    Digit,
    Sign,
    Dot,
    Exp,
    Other,
}

impl Class {
    fn of(c: char) -> Class {
        match c {
            '0'..='9' => Class::Digit,
            '+' | '-' => Class::Sign,
            '.' => Class::Dot,
            'e' | 'E' => Class::Exp,
            _ => Class::Other,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Start,
    /// Read a leading sign.
    Sign,
    /// Read integer digits.
    Int,
    /// Read a dot with no digits before it (".").
    DotNoInt,
    /// Read digits and then a dot ("4.").
    DotAfterInt,
    /// Read fraction digits.
    Frac,
    /// Read `e` or `E`.
    Exp,
    /// Read the exponent's sign.
    ExpSign,
    /// Read exponent digits.
    ExpInt,
}

impl Solution {
    pub fn is_number(s: String) -> bool {
        let mut state = State::Start;
        for c in s.chars() {
            state = match (state, Class::of(c)) {
                (State::Start, Class::Sign) => State::Sign,
                (State::Start | State::Sign | State::Int, Class::Digit) => State::Int,
                (State::Start | State::Sign, Class::Dot) => State::DotNoInt,
                (State::Int, Class::Dot) => State::DotAfterInt,
                (State::DotNoInt | State::DotAfterInt | State::Frac, Class::Digit) => State::Frac,
                (State::Int | State::DotAfterInt | State::Frac, Class::Exp) => State::Exp,
                (State::Exp, Class::Sign) => State::ExpSign,
                (State::Exp | State::ExpSign | State::ExpInt, Class::Digit) => State::ExpInt,
                _ => return false,
            };
        }
        matches!(
            state,
            State::Int | State::DotAfterInt | State::Frac | State::ExpInt
        )
    }
}
```

Full tested solution: [`src/solutions/p0299_valid_number.rs`](../src/solutions/p0299_valid_number.rs)

</details>
