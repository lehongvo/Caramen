//! # Problem 0516: Online Stock Span
//!
//! - **Difficulty:** Medium
//! - **Topics:** design struct, monotonic stack of `(price, span)` pairs, amortized analysis, `Default`
//! - **Source:** [LeetCode 901. Online Stock Span](https://leetcode.com/problems/online-stock-span/)
//!
//! ## Problem
//!
//! Prices of a stock arrive one per day. For each new price, report its **span**: the number of
//! consecutive days, ending today and going backwards, on which the price was less than or equal to
//! today's price. Today always counts, so the span is at least `1`.
//!
//! Implement `StockSpanner`:
//!
//! - `StockSpanner::new() -> Self`
//! - `next(&mut self, price: i32) -> i32`: records today's `price` and returns its span.
//!
//! Also implement `Default`. This is a LeetCode *design* problem, so there is no `impl Solution`:
//! paste the struct and its `impl StockSpanner` block into LeetCode.
//!
//! ## Examples
//!
//! ```text
//! prices: 100, 80, 60, 70, 60, 75, 85
//! spans:    1,  1,  1,  2,  1,  4,  6
//! (75 covers 60, 70, 60 and itself; 85 covers everything except 100)
//!
//! prices: 5, 5, 5
//! spans:  1, 2, 3
//! ```
//!
//! ## Constraints
//!
//! - `1 <= price <= 10^5`
//! - At most `10^4` calls to `next`.
//!
//! ## Hints
//!
//! 1. Once a price is covered by a later, higher-or-equal price, it can never be the price that stops
//!    a future span: the later price stops it first.
//! 2. Keep a stack of `(price, span)` and merge every top whose price is `<=` today's price into
//!    today's span.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** a stack of `(price, span)` with strictly decreasing prices from bottom to top. For a new
//! price, pop every entry with a price `<=` it and add its span to today's span (those days are all
//! covered). Then push `(price, span)`.
//!
//! **Complexity:** amortized `O(1)` per call (every day is pushed and popped at most once), `O(n)`
//! space.
//!
//! **Rust concepts:**
//! - `while let Some(&(p, s)) = self.stack.last()` copies the top pair out (tuples of `Copy` values
//!   are `Copy`), so the borrow ends before `pop`.
//! - Storing the span in the stack means the struct never needs to keep the full price history.
// @solution-end

/// Computes stock spans online.
pub struct StockSpanner {
    // @solution-start remove
    stack: Vec<(i32, i32)>,
    // @solution-end
}

impl StockSpanner {
    pub fn new() -> Self {
        // @solution-start
        StockSpanner { stack: Vec::new() }
        // @solution-end
    }

    pub fn next(&mut self, price: i32) -> i32 {
        // @solution-start
        let mut span = 1;
        while let Some(&(p, s)) = self.stack.last() {
            if p > price {
                break;
            }
            span += s;
            self.stack.pop();
        }
        self.stack.push((price, span));
        span
        // @solution-end
    }
}

impl Default for StockSpanner {
    fn default() -> Self {
        // @solution-start
        Self::new()
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spans(prices: &[i32]) -> Vec<i32> {
        let mut s = StockSpanner::new();
        prices.iter().map(|&p| s.next(p)).collect()
    }

    #[test]
    fn example_sequence() {
        assert_eq!(
            spans(&[100, 80, 60, 70, 60, 75, 85]),
            vec![1, 1, 1, 2, 1, 4, 6]
        );
    }

    #[test]
    fn equal_prices_are_covered() {
        assert_eq!(spans(&[5, 5, 5]), vec![1, 2, 3]);
        let mut s = StockSpanner::default();
        assert_eq!(s.next(7), 1);
    }

    #[test]
    fn monotonic_sequences() {
        assert_eq!(spans(&[1, 2, 3, 4]), vec![1, 2, 3, 4]);
        assert_eq!(spans(&[4, 3, 2, 1]), vec![1, 1, 1, 1]);
    }

    #[test]
    fn matches_brute_force() {
        let prices: Vec<i32> = (0..10_000).map(|i| (i * 7_919 % 1_000) + 1).collect();
        let got = spans(&prices);
        for (i, &p) in prices.iter().enumerate().step_by(97) {
            let expected = prices[..=i].iter().rev().take_while(|&&q| q <= p).count() as i32;
            assert_eq!(got[i], expected, "day {i}");
        }
    }
}
