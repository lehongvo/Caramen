# Problem 0857: Network Delay Time

- **Difficulty:** Medium
- **Topics:** Dijkstra's algorithm, `BinaryHeap` as a min-heap with `std::cmp::Reverse`,
  lazy deletion of stale heap entries
- **Source:** [LeetCode 743. Network Delay Time](https://leetcode.com/problems/network-delay-time/)

## Problem

A network has `n` nodes labelled `1..=n`. `times[i] = [u, v, w]` is a **directed** link: a
signal sent from `u` reaches `v` after `w` time units. A signal is sent from node `k`. Return the
time it takes until **all** `n` nodes have received it, or `-1` if some node never receives it.

Implement `Solution::network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32`.

## Examples

```text
Input:  times = [[2, 1, 1], [2, 3, 1], [3, 4, 1]], n = 4, k = 2
Output: 2
Explanation: node 4 is reached last, through 2 -> 3 -> 4.

Input:  times = [[1, 2, 1]], n = 2, k = 1
Output: 1

Input:  times = [[1, 2, 1]], n = 2, k = 2
Output: -1
```

## Constraints

- `1 <= k <= n <= 100`
- `1 <= times.len() <= 6000`; `1 <= u, v <= n`, `u != v`; `0 <= w <= 100`
- All pairs `(u, v)` are distinct.

## Hints

1. Each node receives the signal at its shortest-path distance from `k`. The answer is the
   largest of those distances.
2. All weights are non-negative, so Dijkstra's algorithm applies: always settle the unsettled
   node with the smallest known distance.
3. `BinaryHeap` is a max-heap; wrap entries in `Reverse` to pop the smallest distance first.

## Starter code

Write your code in [`src/practice/p0857_network_delay_time.rs`](../src/practice/p0857_network_delay_time.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0857
```

```rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
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

    fn run(times: &[[i32; 3]], n: i32, k: i32) -> i32 {
        Solution::network_delay_time(times.iter().map(|t| t.to_vec()).collect(), n, k)
    }

    #[test]
    fn example_four_nodes() {
        assert_eq!(run(&[[2, 1, 1], [2, 3, 1], [3, 4, 1]], 4, 2), 2);
    }

    #[test]
    fn example_two_nodes() {
        assert_eq!(run(&[[1, 2, 1]], 2, 1), 1);
    }

    #[test]
    fn example_unreachable() {
        assert_eq!(run(&[[1, 2, 1]], 2, 2), -1);
    }

    #[test]
    fn detour_beats_direct_link() {
        assert_eq!(run(&[[1, 2, 10], [1, 3, 1], [3, 2, 1]], 3, 1), 2);
    }

    #[test]
    fn zero_weight_links() {
        assert_eq!(run(&[[1, 2, 0], [2, 3, 0]], 3, 1), 0);
        assert_eq!(run(&[[2, 1, 0], [1, 2, 7]], 2, 2), 0);
    }

    #[test]
    fn dense_graph() {
        // Complete digraph on 100 nodes with w(u, v) = |u - v|: the farthest node from 1 is 100.
        let mut times = Vec::new();
        for u in 1..=100_i32 {
            for v in 1..=100 {
                if u != v {
                    times.push([u, v, (u - v).abs()]);
                }
            }
        }
        assert_eq!(run(&times[..6000], 100, 1), 99);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** Dijkstra. `dist[k] = 0`, push `Reverse((0, k))`. Pop the smallest entry; if it is
stale (larger than the recorded `dist`), skip it. Otherwise relax every outgoing edge and push
improved distances. At the end, an unreachable node still has `dist = None`, giving `-1`;
otherwise the answer is the maximum distance.

**Complexity:** `O(E log E)` time, `O(V + E)` space.

**Rust concepts:**
- `BinaryHeap<Reverse<(i32, usize)>>` pops the tuple with the smallest distance; tuples compare
  lexicographically, so ties are broken by node index.
- Lazy deletion: instead of a "decrease key" operation, push a new entry and ignore outdated ones
  when popped.
- `dist: Vec<Option<i32>>` plus `iter().try_fold` (or `map` + `max`) turns "some node is
  unreachable" into `None` neatly.

```rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as usize;
        let mut adj: Vec<Vec<(usize, i32)>> = vec![Vec::new(); n + 1];
        for t in &times {
            adj[t[0] as usize].push((t[1] as usize, t[2]));
        }
        let mut dist: Vec<Option<i32>> = vec![None; n + 1];
        let k = k as usize;
        dist[k] = Some(0);
        let mut heap = BinaryHeap::from([Reverse((0, k))]);
        while let Some(Reverse((d, u))) = heap.pop() {
            if dist[u].is_some_and(|best| d > best) {
                continue; // stale entry
            }
            for &(v, w) in &adj[u] {
                let candidate = d + w;
                if dist[v].is_none_or(|best| candidate < best) {
                    dist[v] = Some(candidate);
                    heap.push(Reverse((candidate, v)));
                }
            }
        }
        dist[1..]
            .iter()
            .try_fold(0, |acc, d| d.map(|d| acc.max(d)))
            .unwrap_or(-1)
    }
}
```

Full tested solution: [`src/solutions/p0857_network_delay_time.rs`](../src/solutions/p0857_network_delay_time.rs)

</details>
