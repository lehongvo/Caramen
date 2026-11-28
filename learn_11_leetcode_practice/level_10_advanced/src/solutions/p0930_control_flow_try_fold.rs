//! # Problem 0930: Short-Circuiting with `ControlFlow`
//!
//! - **Difficulty:** Medium
//! - **Topics:** `std::ops::ControlFlow`, `try_fold`, `try_for_each`, the `?` operator on `ControlFlow`, early exit from recursion
//! - **Source:** Original · [std: `ControlFlow`](https://doc.rust-lang.org/std/ops/enum.ControlFlow.html) · [std: `Iterator::try_fold`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.try_fold)
//!
//! ## Problem
//!
//! `Option` and `Result` short-circuit with "nothing" or "an error". Sometimes stopping early is
//! **success** (we found what we were looking for) or just a different outcome. `ControlFlow<B, C>`
//! says exactly that: `Continue(c)` keeps going, `Break(b)` stops with a value. `try_fold`,
//! `try_for_each` and `?` all understand it.
//!
//! Implement the first three **without** an explicit `for`/`while` loop (use `try_fold` or
//! `try_for_each`), and the last one with recursion and `?`:
//!
//! - `first_repeat(values: &[i32]) -> Option<i32>`: the first value that was already seen earlier.
//! - `sum_until_negative(values: &[i32]) -> (i64, Option<usize>)`: the sum of the values before the
//!   first negative one, and that negative value's index (`None` if there is none, in which case the
//!   sum covers everything).
//! - `checked_product(values: &[u32]) -> ControlFlow<usize, u64>`: `Continue(product)` (the product
//!   of all values as `u64`, `1` for an empty slice), or `Break(i)` where `i` is the index of the value
//!   whose multiplication overflowed `u64`.
//! - `find_path(graph: &[Vec<usize>], start: usize, target: usize) -> Option<Vec<usize>>`: a path from
//!   `start` to `target` (inclusive) found by depth-first search visiting neighbours in the listed
//!   order, never visiting a node twice. Write the search as a recursive helper returning
//!   `ControlFlow<Vec<usize>>` and use `?` to stop as soon as the target is found.
//!
//! ## Examples
//!
//! ```text
//! first_repeat(&[3, 1, 4, 1, 5, 3])       => Some(1)
//! sum_until_negative(&[5, 2, -1, 10])     => (7, Some(2))
//! checked_product(&[2, 3, 7])             => Continue(42)
//! checked_product(&[u32::MAX, u32::MAX, 2]) => Break(2)
//! graph = [[1, 2], [3], [3], []]
//! find_path(&graph, 0, 3)                 => Some([0, 1, 3])
//! ```
//!
//! ## Constraints
//!
//! - `graph` has at most `10^4` nodes; neighbours are valid node indices.
//!
//! ## Hints
//!
//! 1. `try_for_each(|v| if seen.insert(v) { Continue(()) } else { Break(v) })`, then
//!    `.break_value()`.
//! 2. `values.iter().enumerate().try_fold(0, |acc, (i, &v)| ...)` and `match` on the result.
//! 3. In the DFS helper, push the node on the current path, `?` each recursive call, then pop it.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** each function turns "stop early" into `Break`. `first_repeat` breaks on the first failed
//! `HashSet::insert`; `sum_until_negative` and `checked_product` break with the index; the DFS breaks
//! with a clone of the current path, and `?` propagates that `Break` through every level of the
//! recursion without any extra flags.
//!
//! **Complexity:** all are `O(n)` (`O(V + E)` for the DFS).
//!
//! **Rust concepts:**
//! - `ControlFlow` implements the (unstable to implement, stable to use) `Try` trait, so `?` and the
//!   `try_*` iterator methods work with it on stable Rust.
//! - `break_value()` turns a `ControlFlow<B, C>` into `Option<B>`; `ControlFlow<B>` defaults the
//!   continue type to `()`.
//! - Compared with `Result`, `ControlFlow` does not suggest that breaking is an error.
// @solution-end

use std::collections::HashSet;
use std::ops::ControlFlow;

pub fn first_repeat(values: &[i32]) -> Option<i32> {
    // @solution-start
    let mut seen = HashSet::new();
    values
        .iter()
        .try_for_each(|&v| {
            if seen.insert(v) {
                ControlFlow::Continue(())
            } else {
                ControlFlow::Break(v)
            }
        })
        .break_value()
    // @solution-end
}

pub fn sum_until_negative(values: &[i32]) -> (i64, Option<usize>) {
    // @solution-start
    let flow = values.iter().enumerate().try_fold(0_i64, |sum, (i, &v)| {
        if v < 0 {
            ControlFlow::Break((sum, i))
        } else {
            ControlFlow::Continue(sum + i64::from(v))
        }
    });
    match flow {
        ControlFlow::Continue(sum) => (sum, None),
        ControlFlow::Break((sum, i)) => (sum, Some(i)),
    }
    // @solution-end
}

pub fn checked_product(values: &[u32]) -> ControlFlow<usize, u64> {
    // @solution-start
    values
        .iter()
        .enumerate()
        .try_fold(1_u64, |product, (i, &v)| {
            match product.checked_mul(u64::from(v)) {
                Some(next) => ControlFlow::Continue(next),
                None => ControlFlow::Break(i),
            }
        })
    // @solution-end
}

// @solution-start remove
fn dfs(
    graph: &[Vec<usize>],
    node: usize,
    target: usize,
    visited: &mut [bool],
    path: &mut Vec<usize>,
) -> ControlFlow<Vec<usize>> {
    visited[node] = true;
    path.push(node);
    if node == target {
        return ControlFlow::Break(path.clone());
    }
    for &next in &graph[node] {
        if !visited[next] {
            dfs(graph, next, target, visited, path)?;
        }
    }
    path.pop();
    ControlFlow::Continue(())
}
// @solution-end

pub fn find_path(graph: &[Vec<usize>], start: usize, target: usize) -> Option<Vec<usize>> {
    // @solution-start
    let mut visited = vec![false; graph.len()];
    let mut path = Vec::new();
    dfs(graph, start, target, &mut visited, &mut path).break_value()
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_repeats() {
        assert_eq!(first_repeat(&[3, 1, 4, 1, 5, 3]), Some(1));
        assert_eq!(first_repeat(&[1, 2, 3]), None);
        assert_eq!(first_repeat(&[]), None);
        assert_eq!(first_repeat(&[7, 7]), Some(7));
    }

    #[test]
    fn sums_until_negative() {
        assert_eq!(sum_until_negative(&[5, 2, -1, 10]), (7, Some(2)));
        assert_eq!(sum_until_negative(&[1, 2, 3]), (6, None));
        assert_eq!(sum_until_negative(&[-4]), (0, Some(0)));
        assert_eq!(
            sum_until_negative(&[i32::MAX, i32::MAX]),
            (4_294_967_294, None)
        );
    }

    #[test]
    fn checked_products() {
        assert_eq!(checked_product(&[2, 3, 7]), ControlFlow::Continue(42));
        assert_eq!(checked_product(&[]), ControlFlow::Continue(1));
        assert_eq!(
            checked_product(&[u32::MAX, u32::MAX, 2]),
            ControlFlow::Break(2)
        );
        assert_eq!(
            checked_product(&[u32::MAX, u32::MAX]),
            ControlFlow::Continue(18_446_744_065_119_617_025)
        );
    }

    #[test]
    fn dfs_paths() {
        let graph = vec![vec![1, 2], vec![3], vec![3], vec![]];
        assert_eq!(find_path(&graph, 0, 3), Some(vec![0, 1, 3]));
        assert_eq!(find_path(&graph, 2, 2), Some(vec![2]));
        assert_eq!(find_path(&graph, 3, 0), None);
        // A cycle 0 -> 1 -> 0 must not loop forever.
        let cyclic = vec![vec![1], vec![0, 2], vec![]];
        assert_eq!(find_path(&cyclic, 0, 2), Some(vec![0, 1, 2]));
    }

    #[test]
    fn dfs_backtracks_out_of_dead_ends() {
        // 0 -> 1 -> 2 (dead end), 0 -> 3 -> 4.
        let graph = vec![vec![1, 3], vec![2], vec![], vec![4], vec![]];
        assert_eq!(find_path(&graph, 0, 4), Some(vec![0, 3, 4]));
        // A long chain.
        let chain: Vec<Vec<usize>> = (0..2_000)
            .map(|i| if i < 1_999 { vec![i + 1] } else { vec![] })
            .collect();
        assert_eq!(find_path(&chain, 0, 1_999).map(|p| p.len()), Some(2_000));
    }
}
