# Problem 0861: Cheapest Flights Within K Stops

- **Difficulty:** Medium
- **Topics:** Bellman-Ford limited to `k + 1` rounds, copying the previous round with `clone`,
  `Option<i32>` costs
- **Source:** [LeetCode 787. Cheapest Flights Within K Stops](https://leetcode.com/problems/cheapest-flights-within-k-stops/)

## Problem

There are `n` cities `0..n`. `flights[i] = [from, to, price]` is a one-way flight. Return the
cheapest price to travel from `src` to `dst` with **at most `k` stops** in between (so at most
`k + 1` flights), or `-1` if no such route exists.

Implement `Solution::find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32`.

## Examples

```text
Input:  n = 4, flights = [[0, 1, 100], [1, 2, 100], [2, 0, 100], [1, 3, 600], [2, 3, 200]],
        src = 0, dst = 3, k = 1
Output: 700
Explanation: 0 -> 1 -> 3 costs 700. 0 -> 1 -> 2 -> 3 costs only 400 but has 2 stops.

Input:  n = 3, flights = [[0, 1, 100], [1, 2, 100], [0, 2, 500]], src = 0, dst = 2, k = 1
Output: 200

Input:  same flights, k = 0
Output: 500
```

## Constraints

- `1 <= n <= 100`; `0 <= flights.len() <= n * (n - 1) / 2`
- `0 <= from, to < n`, `from != to`; `1 <= price <= 10^4`; no duplicate flights
- `0 <= src, dst, k < n`, `src != dst`

## Hints

1. Plain Dijkstra minimizes cost but ignores the number of flights: a cheap route with too many
   stops can hide a valid, more expensive one.
2. Bellman-Ford's round `r` finds the cheapest costs using at most `r` edges. Run exactly
   `k + 1` rounds.
3. Inside a round, read only the costs from the **previous** round; otherwise one round could
   chain several flights.

## Starter code

Write your code in [`src/practice/p0861_cheapest_flights_within_k_stops.rs`](../src/practice/p0861_cheapest_flights_within_k_stops.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0861
```

```rust
pub struct Solution;

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
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

    fn run(n: i32, flights: &[[i32; 3]], src: i32, dst: i32, k: i32) -> i32 {
        let f = flights.iter().map(|x| x.to_vec()).collect();
        Solution::find_cheapest_price(n, f, src, dst, k)
    }

    #[test]
    fn example_stop_limit_matters() {
        let f = [
            [0, 1, 100],
            [1, 2, 100],
            [2, 0, 100],
            [1, 3, 600],
            [2, 3, 200],
        ];
        assert_eq!(run(4, &f, 0, 3, 1), 700);
    }

    #[test]
    fn example_one_stop() {
        assert_eq!(
            run(3, &[[0, 1, 100], [1, 2, 100], [0, 2, 500]], 0, 2, 1),
            200
        );
    }

    #[test]
    fn example_no_stop() {
        assert_eq!(
            run(3, &[[0, 1, 100], [1, 2, 100], [0, 2, 500]], 0, 2, 0),
            500
        );
    }

    #[test]
    fn unreachable_destination() {
        assert_eq!(run(3, &[[0, 1, 100]], 0, 2, 2), -1);
        assert_eq!(run(2, &[], 0, 1, 1), -1);
    }

    #[test]
    fn long_chain_versus_expensive_direct_flight() {
        // 0 -> 1 -> ... -> 99 costs 99 with 98 stops; the direct flight costs 1000.
        let mut f: Vec<[i32; 3]> = (0..99).map(|i| [i, i + 1, 1]).collect();
        f.push([0, 99, 1000]);
        assert_eq!(run(100, &f, 0, 99, 98), 99);
        assert_eq!(run(100, &f, 0, 99, 97), 1000);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** `cost[src] = Some(0)`. Repeat `k + 1` times: `next = cost.clone()`, then for every
flight `(a, b, p)` with `cost[a] = Some(c)`, lower `next[b]` to `c + p`. Then `cost = next`.
After the last round, `cost[dst]` is the answer.

**Complexity:** `O(k * (n + flights))` time, `O(n)` space.

**Rust concepts:**
- `Option<i32>` for "not reachable yet" avoids adding to a sentinel such as `i32::MAX`.
- `cost.clone()` snapshots the previous round; assigning `cost = next` moves the new vector in
  (no copy).
- `next[b] = Some(next[b].map_or(candidate, |old| old.min(candidate)))` updates a cell in one
  expression.

```rust
pub struct Solution;

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut cost: Vec<Option<i32>> = vec![None; n as usize];
        cost[src as usize] = Some(0);
        for _ in 0..=k {
            let mut next = cost.clone();
            for f in &flights {
                let (a, b, price) = (f[0] as usize, f[1] as usize, f[2]);
                if let Some(c) = cost[a] {
                    let candidate = c + price;
                    next[b] = Some(next[b].map_or(candidate, |old| old.min(candidate)));
                }
            }
            cost = next;
        }
        cost[dst as usize].unwrap_or(-1)
    }
}
```

Full tested solution: [`src/solutions/p0861_cheapest_flights_within_k_stops.rs`](../src/solutions/p0861_cheapest_flights_within_k_stops.rs)

</details>
