# Problem 0863: Min Cost to Connect All Points

- **Difficulty:** Medium
- **Topics:** minimum spanning tree with Prim's algorithm on a dense graph (`O(n^2)`, no heap),
  Manhattan distance, `i64` sums
- **Source:** [LeetCode 1584. Min Cost to Connect All Points](https://leetcode.com/problems/min-cost-to-connect-all-points/)

## Problem

You are given `points[i] = [x, y]` on a plane. Connecting points `a` and `b` costs their
**Manhattan distance** `|xa - xb| + |ya - yb|`. Return the minimum total cost to connect all
points so that there is exactly one simple path between any two points (that is, the total
weight of a minimum spanning tree).

Implement `Solution::min_cost_connect_points(points: Vec<Vec<i32>>) -> i32`.

## Examples

```text
Input:  points = [[0, 0], [2, 2], [3, 10], [5, 2], [7, 0]]
Output: 20

Input:  points = [[3, 12], [-2, 5], [-4, 1]]
Output: 18
```

## Constraints

- `1 <= points.len() <= 1000`
- `-10^6 <= x, y <= 10^6`; all points are distinct
- The answer fits in an `i32`.

## Hints

1. Every pair of points is a potential edge, so the graph is complete: `n^2 / 2` edges. Sorting
   them for Kruskal works but is heavy.
2. Prim's algorithm grows one tree: repeatedly add the outside point that is closest to the tree.
3. Keep `best[v]` = cheapest connection from the tree to `v`; after adding a point, update
   `best` for the others. No heap is needed for a dense graph.

## Starter code

Write your code in [`src/practice/p0863_min_cost_to_connect_all_points.rs`](../src/practice/p0863_min_cost_to_connect_all_points.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0863
```

```rust
pub struct Solution;

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
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

    fn run(points: &[[i32; 2]]) -> i32 {
        Solution::min_cost_connect_points(points.iter().map(|p| p.to_vec()).collect())
    }

    #[test]
    fn example_five_points() {
        assert_eq!(run(&[[0, 0], [2, 2], [3, 10], [5, 2], [7, 0]]), 20);
    }

    #[test]
    fn example_three_points() {
        assert_eq!(run(&[[3, 12], [-2, 5], [-4, 1]]), 18);
    }

    #[test]
    fn single_point() {
        assert_eq!(run(&[[0, 0]]), 0);
    }

    #[test]
    fn small_square() {
        assert_eq!(run(&[[0, 0], [1, 1], [1, 0], [-1, 1]]), 4);
    }

    #[test]
    fn thousand_points() {
        let pts: Vec<[i32; 2]> = (0..1000_i64)
            .map(|i| {
                let x = (i * 7919) % 2_000_001 - 1_000_000;
                let y = (i * 104_729) % 2_000_001 - 1_000_000;
                [x as i32, y as i32]
            })
            .collect();
        assert_eq!(run(&pts), 64_727_094);
        let line: Vec<[i32; 2]> = (0..1000).map(|i| [i, 0]).collect();
        assert_eq!(run(&line), 999);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** Prim with an array. Start with point `0` (`best[0] = 0`). `n` times: pick the unused
point `u` with the smallest `best[u]`, add that cost, mark it used, and lower `best[v]` for every
unused `v` with `dist(u, v)`. Each step is `O(n)`, so the total is `O(n^2)`.

**Complexity:** `O(n^2)` time, `O(n)` space.

**Rust concepts:**
- `(0..n).filter(|&v| !used[v]).min_by_key(|&v| best[v])` selects the next point.
- A single coordinate difference can reach `2 * 10^6` and a distance `4 * 10^6`, which fits in
  `i32`, but the running total is accumulated in `i64` for safety and converted at the end.

```rust
pub struct Solution;

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let dist = |a: usize, b: usize| {
            (points[a][0] - points[b][0]).abs() + (points[a][1] - points[b][1]).abs()
        };
        let mut best = vec![i32::MAX; n];
        let mut used = vec![false; n];
        best[0] = 0;
        let mut total: i64 = 0;
        for _ in 0..n {
            let Some(u) = (0..n).filter(|&v| !used[v]).min_by_key(|&v| best[v]) else {
                break;
            };
            used[u] = true;
            total += i64::from(best[u]);
            for v in 0..n {
                if !used[v] {
                    best[v] = best[v].min(dist(u, v));
                }
            }
        }
        total as i32
    }
}
```

Full tested solution: [`src/solutions/p0863_min_cost_to_connect_all_points.rs`](../src/solutions/p0863_min_cost_to_connect_all_points.rs)

</details>
