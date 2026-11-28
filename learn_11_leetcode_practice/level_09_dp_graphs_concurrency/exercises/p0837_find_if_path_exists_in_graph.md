# Problem 0837: Find if Path Exists in Graph

- **Difficulty:** Easy
- **Topics:** adjacency list `Vec<Vec<usize>>`, breadth-first search with `VecDeque`, visited
  vector
- **Source:** [LeetCode 1971. Find if Path Exists in Graph](https://leetcode.com/problems/find-if-path-exists-in-graph/)

## Problem

An **undirected** graph has `n` vertices labelled `0` to `n - 1`; `edges[i] = [u, v]` connects
`u` and `v`. There are no self-loops and no duplicate edges. Return `true` if there is a path
from `source` to `destination`, and `false` otherwise. A vertex always reaches itself.

Implement `Solution::valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool`.

## Examples

```text
Input:  n = 3, edges = [[0, 1], [1, 2], [2, 0]], source = 0, destination = 2
Output: true

Input:  n = 6, edges = [[0, 1], [0, 2], [3, 5], [5, 4], [4, 3]], source = 0, destination = 5
Output: false
Explanation: {0, 1, 2} and {3, 4, 5} are separate components.
```

## Constraints

- `1 <= n <= 2 * 10^5`
- `0 <= edges.len() <= 2 * 10^5`
- `0 <= u, v, source, destination < n`, `u != v`

## Hints

1. Turn the edge list into an adjacency list: `adj[u]` holds every neighbour of `u`. Add each
   undirected edge in both directions.
2. Explore from `source` with a queue (BFS) or a stack (DFS), marking vertices as visited when
   you first push them.
3. Avoid recursion here: a path graph with `2 * 10^5` vertices would overflow the call stack.

## Starter code

Write your code in [`src/practice/p0837_find_if_path_exists_in_graph.rs`](../src/practice/p0837_find_if_path_exists_in_graph.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0837
```

```rust
use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
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

    fn edges(list: &[[i32; 2]]) -> Vec<Vec<i32>> {
        list.iter().map(|e| e.to_vec()).collect()
    }

    #[test]
    fn example_triangle() {
        let e = edges(&[[0, 1], [1, 2], [2, 0]]);
        assert!(Solution::valid_path(3, e, 0, 2));
    }

    #[test]
    fn example_two_components() {
        let e = edges(&[[0, 1], [0, 2], [3, 5], [5, 4], [4, 3]]);
        assert!(!Solution::valid_path(6, e, 0, 5));
    }

    #[test]
    fn source_equals_destination() {
        assert!(Solution::valid_path(1, vec![], 0, 0));
        assert!(Solution::valid_path(3, vec![], 2, 2));
    }

    #[test]
    fn isolated_destination() {
        let e = edges(&[[0, 1], [1, 2]]);
        assert!(!Solution::valid_path(4, e, 0, 3));
    }

    #[test]
    fn long_path_graph() {
        let n = 200_000;
        let e: Vec<Vec<i32>> = (0..n - 1).map(|i| vec![i, i + 1]).collect();
        assert!(Solution::valid_path(n, e.clone(), 0, n - 1));
        assert!(Solution::valid_path(n, e, n - 1, 0));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** build `adj`, then run BFS from `source`. Stop as soon as `destination` is popped; if
the queue runs dry first, the destination is in another component.

**Complexity:** `O(n + e)` time and space.

**Rust concepts:**
- `vec![Vec::new(); n]` creates `n` empty neighbour lists (`Vec<usize>` is `Clone`).
- `VecDeque::push_back` / `pop_front` make a FIFO queue; `while let Some(u) = queue.pop_front()`
  loops until it is empty.
- Marking `seen[v] = true` when *pushing* (not when popping) keeps each vertex in the queue at
  most once.

```rust
use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let n = n as usize;
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
        for e in &edges {
            let (u, v) = (e[0] as usize, e[1] as usize);
            adj[u].push(v);
            adj[v].push(u);
        }
        let (source, destination) = (source as usize, destination as usize);
        let mut seen = vec![false; n];
        seen[source] = true;
        let mut queue = VecDeque::from([source]);
        while let Some(u) = queue.pop_front() {
            if u == destination {
                return true;
            }
            for &v in &adj[u] {
                if !seen[v] {
                    seen[v] = true;
                    queue.push_back(v);
                }
            }
        }
        false
    }
}
```

Full tested solution: [`src/solutions/p0837_find_if_path_exists_in_graph.rs`](../src/solutions/p0837_find_if_path_exists_in_graph.rs)

</details>
