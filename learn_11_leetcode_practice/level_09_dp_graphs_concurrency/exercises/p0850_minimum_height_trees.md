# Problem 0850: Minimum Height Trees

- **Difficulty:** Medium
- **Topics:** trees as undirected graphs, peeling leaves layer by layer (topological trimming),
  `std::mem::take`
- **Source:** [LeetCode 310. Minimum Height Trees](https://leetcode.com/problems/minimum-height-trees/)

## Problem

A tree with `n` nodes `0..n` is given by its `n - 1` undirected `edges`. Choosing any node as
the root turns it into a rooted tree; its **height** is the number of edges on the longest path
from the root down to a leaf. Return every node whose choice as root gives the **minimum**
height, in any order.

Implement `Solution::find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32>`.

## Examples

```text
Input:  n = 4, edges = [[1, 0], [1, 2], [1, 3]]
Output: [1]
Explanation: rooted at 1 the height is 1; any other root gives 2.

Input:  n = 6, edges = [[3, 0], [3, 1], [3, 2], [3, 4], [5, 4]]
Output: [3, 4]
```

## Constraints

- `1 <= n <= 2 * 10^4`
- `edges.len() == n - 1`; the edges form a tree (connected, no cycle, no repeated edge).

## Hints

1. Running BFS from every node costs `O(n^2)`. Think about where the best roots must be: in the
   "middle" of the longest path.
2. Remove all current leaves at once, then the leaves of what is left, and so on, like peeling
   an onion.
3. Stop when at most 2 nodes remain; those are the answer (a tree has one or two centers).

## Starter code

Write your code in [`src/practice/p0850_minimum_height_trees.rs`](../src/practice/p0850_minimum_height_trees.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0850
```

```rust
pub struct Solution;

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
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

    fn run(n: i32, edges: &[[i32; 2]]) -> Vec<i32> {
        let mut got =
            Solution::find_min_height_trees(n, edges.iter().map(|e| e.to_vec()).collect());
        got.sort_unstable();
        got
    }

    #[test]
    fn example_star() {
        assert_eq!(run(4, &[[1, 0], [1, 2], [1, 3]]), vec![1]);
    }

    #[test]
    fn example_two_centers() {
        assert_eq!(
            run(6, &[[3, 0], [3, 1], [3, 2], [3, 4], [5, 4]]),
            vec![3, 4]
        );
    }

    #[test]
    fn one_and_two_nodes() {
        assert_eq!(run(1, &[]), vec![0]);
        assert_eq!(run(2, &[[0, 1]]), vec![0, 1]);
    }

    #[test]
    fn path_graphs() {
        assert_eq!(run(3, &[[0, 1], [1, 2]]), vec![1]);
        let n = 20_000;
        let edges: Vec<[i32; 2]> = (0..n - 1).map(|i| [i, i + 1]).collect();
        assert_eq!(run(n, &edges), vec![9_999, 10_000]);
    }

    #[test]
    fn uneven_branches() {
        // 0-1-2-3-4-5 with a short branch 2-6: the longest path 0..5 has centers 2 and 3.
        let edges = [[0, 1], [1, 2], [2, 3], [3, 4], [4, 5], [2, 6]];
        assert_eq!(run(7, &edges), vec![2, 3]);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** compute degrees and collect the leaves (degree `1`). While more than 2 nodes remain,
remove the whole current layer of leaves: for each leaf, decrement the degree of its one
remaining neighbour; neighbours that drop to degree `1` form the next layer. The last layer is
the set of centers.

**Complexity:** `O(n)` time and space.

**Rust concepts:**
- `std::mem::take(&mut leaves)` moves the current layer out and leaves an empty `Vec` in its
  place, ready to collect the next layer.
- A tree with one node has no edges; it is handled before the loop.

```rust
pub struct Solution;

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        if n == 1 {
            return vec![0];
        }
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
        for e in &edges {
            let (a, b) = (e[0] as usize, e[1] as usize);
            adj[a].push(b);
            adj[b].push(a);
        }
        let mut degree: Vec<usize> = adj.iter().map(Vec::len).collect();
        let mut leaves: Vec<usize> = (0..n).filter(|&v| degree[v] == 1).collect();
        let mut remaining = n;
        while remaining > 2 {
            remaining -= leaves.len();
            for leaf in std::mem::take(&mut leaves) {
                for &nb in &adj[leaf] {
                    if degree[nb] > 1 {
                        degree[nb] -= 1;
                        if degree[nb] == 1 {
                            leaves.push(nb);
                        }
                    }
                }
            }
        }
        leaves.into_iter().map(|v| v as i32).collect()
    }
}
```

Full tested solution: [`src/solutions/p0850_minimum_height_trees.rs`](../src/solutions/p0850_minimum_height_trees.rs)

</details>
