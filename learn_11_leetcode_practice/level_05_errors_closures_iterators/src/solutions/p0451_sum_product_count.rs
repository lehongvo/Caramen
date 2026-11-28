//! # Problem 0451: Sum, Product and Count
//!
//! - **Difficulty:** Easy
//! - **Topics:** `Iterator::sum`, `Iterator::product`, `Iterator::count`, `map`, `filter`, `filter_map`, type annotations for `sum`
//! - **Source:** Original · [The Rust Book 13.2: Methods that consume the iterator](https://doc.rust-lang.org/book/ch13-02-iterators.html#methods-that-consume-the-iterator) · [std `Iterator::sum`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.sum)
//!
//! ## Problem
//!
//! Iterator *adapters* (`map`, `filter`, ...) describe a transformation; *consumers* (`sum`,
//! `product`, `count`, ...) run the iterator to the end and produce a single value. Most loops that
//! accumulate a number are one adapter chain followed by one consumer.
//!
//! Implement, each with an iterator chain and no explicit loop:
//!
//! - `sum_of_squares(nums: &[i64]) -> i64`.
//! - `product_of_odds(nums: &[i64]) -> i64`: the product of the odd numbers (`1` if there are none).
//! - `count_multiples(nums: &[i64], k: i64) -> usize`: how many numbers are divisible by `k`
//!   (`k != 0`).
//! - `average(nums: &[f64]) -> Option<f64>`: the arithmetic mean, `None` for an empty slice.
//! - `digit_sum(n: u64) -> u32`: the sum of the decimal digits of `n`.
//!
//! ## Examples
//!
//! ```text
//! sum_of_squares(&[1, 2, 3])          -> 14
//! product_of_odds(&[2, 3, 4, 5])      -> 15
//! product_of_odds(&[2, 4])            -> 1
//! count_multiples(&[3, 6, 7, 9], 3)   -> 3
//! average(&[1.0, 2.0, 6.0])           -> Some(3.0)
//! average(&[])                        -> None
//! digit_sum(9045)                     -> 18
//! ```
//!
//! ## Constraints
//!
//! - Slices have at most `10^5` elements; the tests never overflow.
//!
//! ## Hints
//!
//! 1. `sum` and `product` are generic over their result type; the function's return type usually
//!    tells the compiler which one to use. Otherwise write `.sum::<i64>()`.
//! 2. The product of an empty iterator is `1` and the sum is `0`, which is exactly what we want.
//! 3. For the digits, `n.to_string().chars().filter_map(|c| c.to_digit(10))`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** describe *what* to compute: "square each number, then sum", "keep the odd ones, then
//! multiply", "keep the multiples, then count". Iterators are lazy, so nothing runs until the
//! consumer at the end pulls the values through the chain.
//!
//! **Rust concepts:**
//! - `sum` and `product` use the `Sum` and `Product` traits; their output type must be known, from
//!   the return type, a `let` annotation or a turbofish.
//! - `count` returns a `usize` and consumes the iterator.
//! - `x % 2 != 0` (not `== 1`) detects odd numbers correctly for negative values too.
//! - `average` returns `None` instead of dividing by zero; `as f64` converts the length.
// @solution-end

pub fn sum_of_squares(nums: &[i64]) -> i64 {
    // @solution-start
    nums.iter().map(|x| x * x).sum()
    // @solution-end
}

pub fn product_of_odds(nums: &[i64]) -> i64 {
    // @solution-start
    nums.iter().filter(|&&x| x % 2 != 0).product()
    // @solution-end
}

pub fn count_multiples(nums: &[i64], k: i64) -> usize {
    // @solution-start
    nums.iter().filter(|&&x| x % k == 0).count()
    // @solution-end
}

pub fn average(nums: &[f64]) -> Option<f64> {
    // @solution-start
    if nums.is_empty() {
        None
    } else {
        Some(nums.iter().sum::<f64>() / nums.len() as f64)
    }
    // @solution-end
}

pub fn digit_sum(n: u64) -> u32 {
    // @solution-start
    n.to_string().chars().filter_map(|c| c.to_digit(10)).sum()
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn squares() {
        assert_eq!(sum_of_squares(&[1, 2, 3]), 14);
        assert_eq!(sum_of_squares(&[]), 0);
        assert_eq!(sum_of_squares(&[-4]), 16);
    }

    #[test]
    fn odd_products() {
        assert_eq!(product_of_odds(&[2, 3, 4, 5]), 15);
        assert_eq!(product_of_odds(&[2, 4]), 1);
        assert_eq!(product_of_odds(&[-3, 7]), -21);
    }

    #[test]
    fn multiples() {
        assert_eq!(count_multiples(&[3, 6, 7, 9], 3), 3);
        assert_eq!(count_multiples(&[0, -4, 5], 4), 2);
        assert_eq!(count_multiples(&[], 7), 0);
    }

    #[test]
    fn averages() {
        assert_eq!(average(&[1.0, 2.0, 6.0]), Some(3.0));
        assert_eq!(average(&[]), None);
        assert_eq!(average(&[-1.5]), Some(-1.5));
    }

    #[test]
    fn digits_and_large_input() {
        assert_eq!(digit_sum(9045), 18);
        assert_eq!(digit_sum(0), 0);
        assert_eq!(digit_sum(u64::MAX), 87);
        let nums: Vec<i64> = (1..=100_000).collect();
        assert_eq!(sum_of_squares(&nums), 333_338_333_350_000);
        assert_eq!(count_multiples(&nums, 7), 14_285);
    }
}
