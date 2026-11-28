//! # Problem 0420: Overflow-Safe Sum and Product with try_fold
//!
//! - **Difficulty:** Easy
//! - **Topics:** `Iterator::try_fold`, `checked_add`, `checked_mul`, `?` inside closures, `Option::filter`, `ok_or`
//! - **Source:** Original · [std `Iterator::try_fold`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.try_fold)
//!
//! ## Problem
//!
//! `fold` combines all items of an iterator into one value. `try_fold` does the same, but its
//! closure returns an `Option` or a `Result`: as soon as the closure returns `None` / `Err`, the
//! whole fold stops and returns it. That is perfect for arithmetic that may overflow.
//!
//! Implement:
//!
//! - `checked_sum(nums: &[i32]) -> Option<i32>`: the sum, or `None` if any partial sum overflows.
//! - `checked_product(nums: &[i32]) -> Option<i32>`: the product (empty slice -> `Some(1)`), or
//!   `None` on overflow.
//! - `sum_until_limit(nums: &[u32], limit: u32) -> Result<u32, usize>`: add the numbers from left
//!   to right. If the running total would become larger than `limit` (or overflow a `u32`), return
//!   `Err(i)` where `i` is the index of the number that broke the limit. Otherwise return the total.
//! - `parse_and_sum(items: &[&str]) -> Result<i64, String>`: parse each item as an `i64` and add it.
//!   A bad item gives ``"bad number `ITEM`"``; an overflowing sum gives `"overflow"`. The first
//!   problem (from the left) wins.
//!
//! ## Examples
//!
//! ```text
//! checked_sum(&[1, 2, 3])                     -> Some(6)
//! checked_sum(&[i32::MAX, 1, -5])             -> None   (the first partial sum already overflows)
//! checked_product(&[])                        -> Some(1)
//! checked_product(&[65536, 65536])            -> None
//! sum_until_limit(&[5, 10, 20], 30)           -> Err(2) (5 + 10 + 20 = 35 > 30)
//! sum_until_limit(&[5, 10, 15], 30)           -> Ok(30)
//! parse_and_sum(&["1", "x", "9223372036854775807"]) -> Err("bad number `x`")
//! ```
//!
//! ## Constraints
//!
//! - Slices have at most `10^5` elements.
//!
//! ## Hints
//!
//! 1. `nums.iter().try_fold(0i32, |acc, &x| acc.checked_add(x))`.
//! 2. For `sum_until_limit`, combine `enumerate()`, `checked_add`, `Option::filter` and `ok_or(i)`.
//! 3. A closure that returns `Result` may use `?` inside its body.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** every function is a `try_fold` whose closure computes the next accumulator or reports
//! why it cannot. Because `try_fold` stops at the first failure, the remaining items are never
//! touched.
//!
//! **Rust concepts:**
//! - `checked_add` / `checked_mul` return `Option`, which fits `try_fold` directly.
//! - The accumulator type is fixed by the initial value (`0i32`, `1i32`, `0i64`, `0u32`).
//! - `Option::filter(|&s| s <= limit)` turns "too big" into `None`, and `ok_or(i)` attaches the index.
//! - Inside a closure, `?` returns from the **closure**, not from the enclosing function; here that
//!   is exactly the value `try_fold` needs.
// @solution-end

pub fn checked_sum(nums: &[i32]) -> Option<i32> {
    // @solution-start
    nums.iter().try_fold(0i32, |acc, &x| acc.checked_add(x))
    // @solution-end
}

pub fn checked_product(nums: &[i32]) -> Option<i32> {
    // @solution-start
    nums.iter().try_fold(1i32, |acc, &x| acc.checked_mul(x))
    // @solution-end
}

pub fn sum_until_limit(nums: &[u32], limit: u32) -> Result<u32, usize> {
    // @solution-start
    nums.iter().enumerate().try_fold(0u32, |acc, (i, &x)| {
        acc.checked_add(x).filter(|&total| total <= limit).ok_or(i)
    })
    // @solution-end
}

pub fn parse_and_sum(items: &[&str]) -> Result<i64, String> {
    // @solution-start
    items.iter().try_fold(0i64, |acc, item| {
        let n: i64 = item.parse().map_err(|_| format!("bad number `{item}`"))?;
        acc.checked_add(n).ok_or_else(|| "overflow".to_string())
    })
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checked_sum_cases() {
        assert_eq!(checked_sum(&[1, 2, 3]), Some(6));
        assert_eq!(checked_sum(&[]), Some(0));
        assert_eq!(checked_sum(&[i32::MAX, 1, -5]), None);
        assert_eq!(checked_sum(&[i32::MAX, -5, 5]), Some(i32::MAX));
        assert_eq!(checked_sum(&[i32::MIN, -1]), None);
    }

    #[test]
    fn checked_product_cases() {
        assert_eq!(checked_product(&[]), Some(1));
        assert_eq!(checked_product(&[2, -3, 7]), Some(-42));
        assert_eq!(checked_product(&[65_536, 65_536]), None);
        assert_eq!(checked_product(&[0, 65_536, 65_536]), Some(0));
    }

    #[test]
    fn sum_until_limit_cases() {
        assert_eq!(sum_until_limit(&[5, 10, 20], 30), Err(2));
        assert_eq!(sum_until_limit(&[5, 10, 15], 30), Ok(30));
        assert_eq!(sum_until_limit(&[31], 30), Err(0));
        assert_eq!(sum_until_limit(&[], 0), Ok(0));
        assert_eq!(sum_until_limit(&[u32::MAX, 1], u32::MAX), Err(1));
    }

    #[test]
    fn parse_and_sum_cases() {
        assert_eq!(parse_and_sum(&["1", "2", "-3"]), Ok(0));
        assert_eq!(
            parse_and_sum(&["1", "x", "9223372036854775807"]),
            Err("bad number `x`".to_string())
        );
        assert_eq!(
            parse_and_sum(&["1", "9223372036854775807", "x"]),
            Err("overflow".to_string())
        );
    }

    #[test]
    fn large_input() {
        let nums = vec![1i32; 100_000];
        assert_eq!(checked_sum(&nums), Some(100_000));
        assert_eq!(checked_product(&nums), Some(1));
        let small = vec![1u32; 100_000];
        assert_eq!(sum_until_limit(&small, 50_000), Err(50_000));
    }
}
