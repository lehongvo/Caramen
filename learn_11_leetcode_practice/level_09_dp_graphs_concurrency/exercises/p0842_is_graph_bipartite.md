# Problem 0842: Is Graph Bipartite?

- **Difficulty:** Medium
- **Topics:** BFS 2-coloring, `Vec<Option<bool>>` as "uncolored / color", several components
- **Source:** [LeetCode 785. Is Graph Bipartite?](https://leetcode.com/problems/is-graph-bipartite/)

## Problem

An undirected graph with `n` nodes `0..n` is given as an adjacency list: `graph[u]` lists the
neighbours of `u` (if `v` is in `graph[u]`, then `u` is in `graph[v]`; no self-loops, no
duplicates). The graph may be disconnected.

The graph is **bipartite** if its nodes can be split into two sets `A` and `B` such that every
edge connects a node of `A` with a node of `B`. Return `true` if the graph is bipartite.

Implement `Solution::is_bipartite(graph: Vec<Vec<i32>>) -> bool`.

## Examples

```text
Input:  graph = [[1, 2, 3], [0, 2], [0, 1, 3], [0, 2]]
Output: false
Explanation: 0, 1, 2 form a triangle; a triangle cannot be 2-colored.

Input:  graph = [[1, 3], [0, 2], [1, 3], [0, 2]]
Output: true
Explanation: A = {0, 2}, B = {1, 3}.
```

## Constraints

- `1 <= n <= 100`
- `0 <= graph[u].len() < n`; `graph[u]` does not contain `u` or duplicates.

## Hints

1. Color the start node, then give each neighbour the opposite color, level by level (BFS).
2. If an edge ever connects two nodes of the same color, the graph is not bipartite.
3. Repeat from every node that is still uncolored, because the graph may have several
   components.

## Starter code

Write your code in [`src/practice/p0842_is_graph_bipartite.rs`](../src/practice/p0842_is_graph_bipartite.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0842
```

```rust
use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
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

    #[test]
    fn example_triangle_inside() {
        let g = vec![vec![1, 2, 3], vec![0, 2], vec![0, 1, 3], vec![0, 2]];
        assert!(!Solution::is_bipartite(g));
    }

    #[test]
    fn example_square() {
        let g = vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]];
        assert!(Solution::is_bipartite(g));
    }

    #[test]
    fn single_node_and_no_edges() {
        assert!(Solution::is_bipartite(vec![vec![]]));
        assert!(Solution::is_bipartite(vec![vec![], vec![], vec![]]));
    }

    #[test]
    fn odd_cycle_in_second_component() {
        // Component {0, 1} is fine; component {2, 3, 4} is a triangle.
        let g = vec![vec![1], vec![0], vec![3, 4], vec![2, 4], vec![2, 3]];
        assert!(!Solution::is_bipartite(g));
    }

    #[test]
    fn even_and_odd_large_cycles() {
        let cycle = |n: i32| -> Vec<Vec<i32>> {
            (0..n).map(|i| vec![(i + n - 1) % n, (i + 1) % n]).collect()
        };
        assert!(Solution::is_bipartite(cycle(100)));
        assert!(!Solution::is_bipartite(cycle(99)));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** `color[u]` is `None` (unvisited) or `Some(side)`. For every uncolored start node, BFS:
a neighbour that is uncolored gets `Some(!side)` and joins the queue; a neighbour with the same
color proves an odd cycle, so return `false`.

**Complexity:** `O(n + e)` time, `O(n)` space.

**Rust concepts:**
- `Option<bool>` encodes three states (unvisited, side A, side B) without magic numbers.
- `match color[v] { None => ..., Some(c) if c == side => ..., Some(_) => {} }` uses a match
  guard for the conflict case.

```rust
use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let n = graph.len();
        let mut color: Vec<Option<bool>> = vec![None; n];
        for start in 0..n {
            if color[start].is_some() {
                continue;
            }
            color[start] = Some(false);
            let mut queue = VecDeque::from([start]);
            while let Some(u) = queue.pop_front() {
                let side = color[u] == Some(true);
                for &v in &graph[u] {
                    let v = v as usize;
                    match color[v] {
                        None => {
                            color[v] = Some(!side);
                            queue.push_back(v);
                        }
                        Some(c) if c == side => return false,
                        Some(_) => {}
                    }
                }
            }
        }
        true
    }
}
```

Full tested solution: [`src/solutions/p0842_is_graph_bipartite.rs`](../src/solutions/p0842_is_graph_bipartite.rs)

</details>
