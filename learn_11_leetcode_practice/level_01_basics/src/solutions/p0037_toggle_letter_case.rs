//! # Problem 0037: Toggle Letter Case
//!
//! - **Difficulty:** Easy
//! - **Topics:** `char` methods (`is_ascii_lowercase`, `to_ascii_uppercase`, ...), slices `&[char]`, tuples
//! - **Source:** Original · [`char::to_ascii_uppercase`](https://doc.rust-lang.org/std/primitive.char.html#method.to_ascii_uppercase)
//!
//! ## Problem
//!
//! You receive text as a slice of characters. Write two functions:
//!
//! 1. `toggle_case(chars: &[char]) -> Vec<char>` returns a new vector in which every ASCII
//!    lowercase letter became uppercase and every ASCII uppercase letter became lowercase. All other
//!    characters are copied unchanged.
//! 2. `count_cases(chars: &[char]) -> (usize, usize)` returns `(lowercase_count, uppercase_count)`,
//!    counting ASCII letters only.
//!
//! ## Examples
//!
//! ```text
//! toggle_case(&['R', 'u', 's', 't'])     -> ['r', 'U', 'S', 'T']
//! toggle_case(&['a', '1', 'B', '?'])     -> ['A', '1', 'b', '?']
//! count_cases(&['R', 'u', 's', 't'])     -> (3, 1)
//! count_cases(&['4', '2'])               -> (0, 0)
//! ```
//!
//! ## Constraints
//!
//! - `0 <= chars.len() <= 10^5`
//!
//! ## Hints
//!
//! 1. `c.is_ascii_lowercase()` and `c.is_ascii_uppercase()` answer the case questions.
//! 2. `c.to_ascii_uppercase()` returns a **new** `char`; it does not change `c` (a `char` is `Copy`).
//! 3. Two `mut` counters returned together as a tuple solve the second part.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** for each character, use an `if` / `else if` / `else` expression to pick the converted
//! character, and push it into the output. For counting, increment one of two counters.
//!
//! **Rust concepts:**
//! - `char` is `Copy`, so `for &c in chars` copies each character out of the slice.
//! - The `ascii` methods only touch `a-z` / `A-Z`, which makes them predictable. The Unicode-aware
//!   `to_uppercase` returns an **iterator**, because some characters become several characters
//!   (`'ß'` becomes `"SS"`).
//! - Returning `(lower, upper)` as a tuple groups two related results without defining a struct.
// @solution-end

pub fn toggle_case(chars: &[char]) -> Vec<char> {
    // @solution-start
    let mut out = Vec::with_capacity(chars.len());
    for &c in chars {
        let toggled = if c.is_ascii_lowercase() {
            c.to_ascii_uppercase()
        } else if c.is_ascii_uppercase() {
            c.to_ascii_lowercase()
        } else {
            c
        };
        out.push(toggled);
    }
    out
    // @solution-end
}

pub fn count_cases(chars: &[char]) -> (usize, usize) {
    // @solution-start
    let mut lower = 0;
    let mut upper = 0;
    for &c in chars {
        if c.is_ascii_lowercase() {
            lower += 1;
        } else if c.is_ascii_uppercase() {
            upper += 1;
        }
    }
    (lower, upper)
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toggle_examples() {
        assert_eq!(toggle_case(&['R', 'u', 's', 't']), vec!['r', 'U', 'S', 'T']);
        assert_eq!(toggle_case(&['a', '1', 'B', '?']), vec!['A', '1', 'b', '?']);
    }

    #[test]
    fn non_ascii_is_unchanged() {
        assert_eq!(toggle_case(&['Ω', 'ß', ' ']), vec!['Ω', 'ß', ' ']);
        assert_eq!(count_cases(&['Ω', 'ß']), (0, 0));
    }

    #[test]
    fn empty_input() {
        assert_eq!(toggle_case(&[]), Vec::<char>::new());
        assert_eq!(count_cases(&[]), (0, 0));
    }

    #[test]
    fn count_examples() {
        assert_eq!(count_cases(&['R', 'u', 's', 't']), (3, 1));
        assert_eq!(count_cases(&['4', '2']), (0, 0));
    }

    #[test]
    fn toggling_twice_restores_the_text() {
        let text: Vec<char> = "The Quick Brown Fox, 2024!".chars().collect();
        let once = toggle_case(&text);
        assert_eq!(
            count_cases(&once),
            (count_cases(&text).1, count_cases(&text).0)
        );
        assert_eq!(toggle_case(&once), text);
    }
}
