//! # Problem 0495: Different Ways to Add Parentheses
//!
//! - **Difficulty:** Medium
//! - **Topics:** divide and conquer with recursion, `char_indices` + `filter`, nested `flat_map` (Cartesian product), a `move` closure choosing the operator
//! - **Source:** [LeetCode 241. Different Ways to Add Parentheses](https://leetcode.com/problems/different-ways-to-add-parentheses/)
//!
//! ## Problem
//!
//! `expression` contains non-negative integers and the operators `'+'`, `'-'` and `'*'` (no spaces,
//! no parentheses). Return the results of **every** way to fully parenthesise it, in any order. Two
//! different groupings that give the same value both appear in the output.
//!
//! Implement `Solution::diff_ways_to_compute(expression: String) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  expression = "2-1-1"
//! Output: [0, 2]
//! Explanation: ((2-1)-1) = 0 and (2-(1-1)) = 2.
//!
//! Input:  expression = "2*3-4*5"
//! Output: [-34, -14, -10, -10, 10]
//! Explanation: (2*(3-(4*5))) = -34, ((2*3)-(4*5)) = -14, ((2*(3-4))*5) = -10,
//!              (2*((3-4)*5)) = -10, (((2*3)-4)*5) = 10.
//! ```
//!
//! ## Constraints
//!
//! - `1 <= expression.len() <= 20`
//! - Numbers are in `0..=99`; no leading `'-'` or `'+'`.
//! - Every intermediate and final value fits in an `i32`; there are at most `10^4` results.
//!
//! ## Hints
//!
//! 1. The **last** operator applied splits the expression into a left part and a right part. Try
//!    every operator as that last one.
//! 2. Recursively compute all results of the left part and all results of the right part, then
//!    combine every left result with every right result.
//! 3. An expression without operators is just a number.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** divide and conquer. For each operator position, the results are the Cartesian product
//! of the left results and the right results, combined with that operator. The base case is a
//! plain number.
//!
//! **Complexity:** exponential in the number of operators (it grows like the Catalan numbers), which
//! is fine for at most 10 operators. Memoising sub-expressions would avoid repeated work.
//!
//! **Rust concepts:**
//! - `char_indices().filter(...)` finds the operators together with their byte positions, so the
//!   expression can be sliced into `&expr[..i]` and `&expr[i + 1..]`.
//! - `left.iter().flat_map(|&a| right.iter().map(move |&b| apply(a, b)))` is a Cartesian product.
//! - `apply` is a closure that captures the operator `op` (a `char`, which is `Copy`), so it is
//!   itself `Copy` and can be moved into the inner closure.
//! - `parse().map(|n| vec![n]).unwrap_or_default()` handles the base case without `unwrap`.
// @solution-end

pub struct Solution;

// @solution-start remove
fn ways(expr: &str) -> Vec<i32> {
    let results: Vec<i32> = expr
        .char_indices()
        .filter(|&(_, c)| matches!(c, '+' | '-' | '*'))
        .flat_map(|(i, op)| {
            let left = ways(&expr[..i]);
            let right = ways(&expr[i + 1..]);
            let apply = move |a: i32, b: i32| match op {
                '+' => a + b,
                '-' => a - b,
                _ => a * b,
            };
            left.iter()
                .flat_map(|&a| right.iter().map(move |&b| apply(a, b)))
                .collect::<Vec<i32>>()
        })
        .collect();
    if results.is_empty() {
        expr.parse().map(|n| vec![n]).unwrap_or_default()
    } else {
        results
    }
}
// @solution-end

impl Solution {
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        // @solution-start
        ways(&expression)
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(expr: &str) -> Vec<i32> {
        let mut out = Solution::diff_ways_to_compute(expr.to_string());
        out.sort();
        out
    }

    #[test]
    fn example_two_minus() {
        assert_eq!(run("2-1-1"), vec![0, 2]);
    }

    #[test]
    fn example_mixed_operators() {
        assert_eq!(run("2*3-4*5"), vec![-34, -14, -10, -10, 10]);
    }

    #[test]
    fn single_number_and_single_operator() {
        assert_eq!(run("42"), vec![42]);
        assert_eq!(run("0"), vec![0]);
        assert_eq!(run("99*99"), vec![9801]);
    }

    #[test]
    fn count_matches_catalan_numbers() {
        // 6 operators -> Catalan(6) = 132 groupings.
        assert_eq!(run("1+2+3+4+5+6+7").len(), 132);
        assert!(run("1+2+3+4+5+6+7").iter().all(|&v| v == 28));
        assert_eq!(run("1-1-1-1").len(), 5);
    }
}
