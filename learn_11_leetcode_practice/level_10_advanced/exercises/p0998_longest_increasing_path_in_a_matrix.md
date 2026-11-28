# Problem 0998: Longest Increasing Path in a Matrix

- **Difficulty:** Hard
- **Topics:** DFS with memoisation on a DAG, or topological "peeling" with a `VecDeque` (used here to avoid deep recursion), out-degrees
- **Source:** [LeetCode 329. Longest Increasing Path in a Matrix](https://leetcode.com/problems/longest-increasing-path-in-a-matrix/)

## Problem

In an `m x n` integer matrix, a path moves between horizontally or vertically adjacent cells
(no diagonals, no wrap-around) and must be **strictly increasing**. Return the length (number of
cells) of the longest such path.

Implement `Solution::longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32`.

## Examples

```text
matrix = [[9, 9, 4],
          [6, 6, 8],
          [2, 1, 1]]     => 4     (1 -> 2 -> 6 -> 9)

matrix = [[3, 4, 5],
          [3, 2, 6],
          [2, 2, 1]]     => 4     (3 -> 4 -> 5 -> 6)
```

## Constraints

- `1 <= m, n <= 200`, `0 <= matrix[i][j] <= 2^31 - 1`

## Hints

1. Edges go from smaller to larger neighbours, so the cells form a directed **acyclic** graph.
2. The longest path in a DAG can be found by peeling layers: first remove every cell with no
   larger neighbour (out-degree 0), then the cells that only pointed at those, and so on.
3. The number of layers is the answer. (A memoised DFS works too, but recursion depth can reach
   `40 000` on a snake-shaped matrix.)

## Starter code

Write your code in [`src/practice/p0998_longest_increasing_path_in_a_matrix.rs`](../src/practice/p0998_longest_increasing_path_in_a_matrix.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0998
```

```rust
use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
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

    fn naive(matrix: &[Vec<i32>]) -> i32 {
        fn longest(m: &[Vec<i32>], r: usize, c: usize) -> i32 {
            let mut best = 1;
            let moves = [
                (r.wrapping_sub(1), c),
                (r + 1, c),
                (r, c.wrapping_sub(1)),
                (r, c + 1),
            ];
            for (nr, nc) in moves {
                if nr < m.len() && nc < m[0].len() && m[nr][nc] > m[r][c] {
                    best = best.max(1 + longest(m, nr, nc));
                }
            }
            best
        }
        (0..matrix.len())
            .flat_map(|r| (0..matrix[0].len()).map(move |c| (r, c)))
            .map(|(r, c)| longest(matrix, r, c))
            .max()
            .unwrap_or(0)
    }

    #[test]
    fn examples() {
        assert_eq!(
            Solution::longest_increasing_path(vec![vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]]),
            4
        );
        assert_eq!(
            Solution::longest_increasing_path(vec![vec![3, 4, 5], vec![3, 2, 6], vec![2, 2, 1]]),
            4
        );
        assert_eq!(Solution::longest_increasing_path(vec![vec![1]]), 1);
    }

    #[test]
    fn flat_and_line() {
        assert_eq!(Solution::longest_increasing_path(vec![vec![7; 5]; 5]), 1);
        assert_eq!(
            Solution::longest_increasing_path(vec![vec![1, 2, 3, 4, 5]]),
            5
        );
        assert_eq!(
            Solution::longest_increasing_path(vec![vec![i32::MAX], vec![0]]),
            2
        );
    }

    #[test]
    fn matches_exhaustive_search() {
        let matrix: Vec<Vec<i32>> = (0..6)
            .map(|r: i32| (0..7).map(|c: i32| (r * 17 + c * 29) % 13).collect())
            .collect();
        assert_eq!(
            Solution::longest_increasing_path(matrix.clone()),
            naive(&matrix)
        );
    }

    #[test]
    fn snake_through_the_whole_matrix() {
        // Row-wise "boustrophedon" numbering: one increasing path through all 40_000 cells.
        let n = 200;
        let matrix: Vec<Vec<i32>> = (0..n)
            .map(|r| {
                (0..n)
                    .map(|c| {
                        let col = if r % 2 == 0 { c } else { n - 1 - c };
                        r * n + col
                    })
                    .collect()
            })
            .collect();
        assert_eq!(Solution::longest_increasing_path(matrix), 40_000);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** compute every cell's out-degree (number of strictly larger neighbours). Put the cells of
out-degree 0 in a queue; process the queue layer by layer: removing a cell decrements the
out-degree of its smaller neighbours, and those reaching 0 join the next layer. Each layer
extends the longest path ending at the peaks by one.

**Complexity:** `O(m * n)` time and space.

**Rust concepts:**
- A neighbour helper returns an iterator of in-bounds coordinates with `filter_map` over four
  direction offsets (`checked_add_signed` handles the bounds at 0).
- Processing a `VecDeque` one layer at a time: remember `queue.len()` before the inner loop.

```rust
use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (matrix.len(), matrix[0].len());
        let neighbors = move |r: usize, c: usize| {
            [(-1_isize, 0_isize), (1, 0), (0, -1), (0, 1)]
                .into_iter()
                .filter_map(move |(dr, dc)| {
                    let nr = r.checked_add_signed(dr)?;
                    let nc = c.checked_add_signed(dc)?;
                    (nr < m && nc < n).then_some((nr, nc))
                })
        };
        let mut out_degree = vec![vec![0_u8; n]; m];
        let mut queue = VecDeque::new();
        for r in 0..m {
            for c in 0..n {
                out_degree[r][c] = neighbors(r, c)
                    .filter(|&(nr, nc)| matrix[nr][nc] > matrix[r][c])
                    .count() as u8;
                if out_degree[r][c] == 0 {
                    queue.push_back((r, c));
                }
            }
        }
        let mut layers = 0;
        while !queue.is_empty() {
            layers += 1;
            for _ in 0..queue.len() {
                let Some((r, c)) = queue.pop_front() else {
                    break;
                };
                for (nr, nc) in neighbors(r, c) {
                    if matrix[nr][nc] < matrix[r][c] {
                        out_degree[nr][nc] -= 1;
                        if out_degree[nr][nc] == 0 {
                            queue.push_back((nr, nc));
                        }
                    }
                }
            }
        }
        layers
    }
}
```

Full tested solution: [`src/solutions/p0998_longest_increasing_path_in_a_matrix.rs`](../src/solutions/p0998_longest_increasing_path_in_a_matrix.rs)

</details>
