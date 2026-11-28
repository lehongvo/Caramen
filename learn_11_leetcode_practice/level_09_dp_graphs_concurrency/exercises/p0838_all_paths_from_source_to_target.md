# Problem 0838: All Paths From Source to Target

- **Difficulty:** Medium
- **Topics:** depth-first search with backtracking on a DAG, recursive helper with `&mut Vec`
- **Source:** [LeetCode 797. All Paths From Source to Target](https://leetcode.com/problems/all-paths-from-source-to-target/)

## Problem

You are given a **directed acyclic graph** (DAG) with `n` nodes labelled `0` to `n - 1`, as an
adjacency list: `graph[i]` lists the nodes `j` with an edge `i -> j`. Return every path from
node `0` to node `n - 1`. Each path is the list of its nodes, starting with `0` and ending with
`n - 1`. The paths may be returned in any order.

Implement `Solution::all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>>`.

## Examples

```text
Input:  graph = [[1, 2], [3], [3], []]
Output: [[0, 1, 3], [0, 2, 3]]

Input:  graph = [[4, 3, 1], [3, 2, 4], [3], [4], []]
Output: [[0, 4], [0, 3, 4], [0, 1, 3, 4], [0, 1, 2, 3, 4], [0, 1, 4]]
```

## Constraints

- `2 <= n <= 15`
- `graph[i]` contains distinct values `j` with `0 <= j < n` and `j != i`.
- The graph is acyclic.

## Hints

1. Because there is no cycle, you never need a visited set: a path cannot come back to a node.
2. Keep the current path in one vector. Push a node before exploring it and pop it afterwards
   (backtracking). When the last node is reached, store a copy of the path.

## Starter code

Write your code in [`src/practice/p0838_all_paths_from_source_to_target.rs`](../src/practice/p0838_all_paths_from_source_to_target.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0838
```

```rust
pub struct Solution;

impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
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

    fn sorted(mut paths: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        paths.sort();
        paths
    }

    #[test]
    fn example_diamond() {
        let graph = vec![vec![1, 2], vec![3], vec![3], vec![]];
        let got = sorted(Solution::all_paths_source_target(graph));
        assert_eq!(got, vec![vec![0, 1, 3], vec![0, 2, 3]]);
    }

    #[test]
    fn example_five_nodes() {
        let graph = vec![vec![4, 3, 1], vec![3, 2, 4], vec![3], vec![4], vec![]];
        let got = sorted(Solution::all_paths_source_target(graph));
        let want = sorted(vec![
            vec![0, 4],
            vec![0, 3, 4],
            vec![0, 1, 3, 4],
            vec![0, 1, 2, 3, 4],
            vec![0, 1, 4],
        ]);
        assert_eq!(got, want);
    }

    #[test]
    fn two_nodes() {
        let graph = vec![vec![1], vec![]];
        assert_eq!(Solution::all_paths_source_target(graph), vec![vec![0, 1]]);
    }

    #[test]
    fn target_unreachable() {
        let graph = vec![vec![1], vec![], vec![]];
        assert!(Solution::all_paths_source_target(graph).is_empty());
    }

    #[test]
    fn complete_dag_of_fifteen() {
        // Edge i -> j for every i < j: each of the 13 middle nodes is either on a path or not.
        let n = 15;
        let graph: Vec<Vec<i32>> = (0..n).map(|i| (i + 1..n).collect()).collect();
        let paths = Solution::all_paths_source_target(graph);
        assert_eq!(paths.len(), 1 << 13);
        assert!(
            paths
                .iter()
                .all(|p| p[0] == 0 && *p.last().unwrap() == n - 1)
        );
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** recursive DFS from `0` carrying the current `path`. At node `n - 1` push
`path.clone()` into the answer; otherwise for each successor push it, recurse and pop it.

**Complexity:** up to `O(2^n * n)` time and output size (a complete DAG has `2^(n-2)` paths),
`O(n)` extra space for the recursion and the current path.

**Rust concepts:**
- The helper takes `&[Vec<i32>]` for the graph and `&mut Vec<i32>` for the shared path, so one
  buffer is reused for the whole search; only finished paths are cloned.
- Recursion depth is at most `n <= 15`, so a recursive helper is fine here.

```rust
pub struct Solution;

impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut paths = Vec::new();
        let mut path = vec![0];
        Self::dfs(&graph, 0, &mut path, &mut paths);
        paths
    }

    fn dfs(graph: &[Vec<i32>], node: usize, path: &mut Vec<i32>, paths: &mut Vec<Vec<i32>>) {
        if node == graph.len() - 1 {
            paths.push(path.clone());
            return;
        }
        for &next in &graph[node] {
            path.push(next);
            Self::dfs(graph, next as usize, path, paths);
            path.pop();
        }
    }
}
```

Full tested solution: [`src/solutions/p0838_all_paths_from_source_to_target.rs`](../src/solutions/p0838_all_paths_from_source_to_target.rs)

</details>
