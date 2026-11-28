// ==================================================================
// PRACTICE 0065: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0065_next_power_of_two.md
// Run the tests:          cargo test practice::p0065
// ==================================================================

//! # Problem 0065: Next Power of Two
//!
//! - **Difficulty:** Easy
//! - **Topics:** `loop` with a `break` value, `checked_mul`, `Option`, halving with `/=`
//! - **Source:** Original · [The Rust Book 3.5](https://doc.rust-lang.org/book/ch03-05-control-flow.html#returning-values-from-loops)
//!
//! ## Problem
//!
//! Powers of two (`1, 2, 4, 8, ...`) appear everywhere: buffer sizes, hash table capacities, bit
//! masks. Write two functions:
//!
//! 1. `next_power_of_two(n: u32) -> Option<u32>` returns the smallest power of two that is
//!    `>= n` (for `n = 0` that is `1`), or `None` if that power does not fit in a `u32`
//!    (that is, when `n > 2^31`).
//! 2. `floor_log2(n: u32) -> Option<u32>` returns the exponent `k` of the largest power of two with
//!    `2^k <= n`, or `None` for `n = 0` (no power of two is `<= 0`).
//!
//! Write both with a `loop` that **returns its result through `break`**.
//!
//! ## Examples
//!
//! ```text
//! next_power_of_two(0)          -> Some(1)
//! next_power_of_two(5)          -> Some(8)
//! next_power_of_two(64)         -> Some(64)
//! next_power_of_two(2147483649) -> None        2^31 + 1 needs 2^32
//!
//! floor_log2(1)    -> Some(0)
//! floor_log2(10)   -> Some(3)       2^3 = 8 <= 10 < 16
//! floor_log2(0)    -> None
//! ```
//!
//! ## Constraints
//!
//! - Any `u32`.
//!
//! ## Hints
//!
//! 1. `let x = loop { ... break value; ... };` gives `x` the value passed to `break`.
//! 2. Start with `p = 1` and double it. Doubling `2^31` overflows: `p.checked_mul(2)` returns `None` then.
//! 3. For `floor_log2`, count how many times you can halve `n` before it becomes 1.

pub fn next_power_of_two(n: u32) -> Option<u32> {
    todo!("Write your solution here")
}

pub fn floor_log2(n: u32) -> Option<u32> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_power_examples() {
        assert_eq!(next_power_of_two(0), Some(1));
        assert_eq!(next_power_of_two(1), Some(1));
        assert_eq!(next_power_of_two(5), Some(8));
        assert_eq!(next_power_of_two(64), Some(64));
        assert_eq!(next_power_of_two(65), Some(128));
    }

    #[test]
    fn next_power_at_the_limit() {
        assert_eq!(next_power_of_two(1 << 31), Some(1 << 31));
        assert_eq!(next_power_of_two((1 << 31) + 1), None);
        assert_eq!(next_power_of_two(u32::MAX), None);
    }

    #[test]
    fn floor_log2_examples() {
        assert_eq!(floor_log2(0), None);
        assert_eq!(floor_log2(1), Some(0));
        assert_eq!(floor_log2(10), Some(3));
        assert_eq!(floor_log2(1024), Some(10));
        assert_eq!(floor_log2(u32::MAX), Some(31));
    }

    #[test]
    fn matches_the_standard_library() {
        for n in (0..=u32::MAX).step_by(65_537).chain(0..5000) {
            assert_eq!(
                next_power_of_two(n),
                n.checked_next_power_of_two(),
                "n = {n}"
            );
            let expected = if n == 0 { None } else { Some(n.ilog2()) };
            assert_eq!(floor_log2(n), expected, "n = {n}");
        }
    }
}
