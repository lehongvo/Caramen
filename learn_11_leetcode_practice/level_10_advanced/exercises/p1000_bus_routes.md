# Problem 1000: Bus Routes

- **Difficulty:** Hard
- **Topics:** BFS over **routes** instead of stops, `HashMap<stop, Vec<route>>` as an inverted index, visited sets for both routes and stops
- **Source:** [LeetCode 815. Bus Routes](https://leetcode.com/problems/bus-routes/)

## Problem

`routes[i]` lists the stops of bus `i`, which drives them in a loop forever (so you can travel
between any two stops of the same route on that bus). Starting at stop `source` (not on any bus
yet), return the minimum number of **buses** you must take to reach stop `target`, or `-1` if it
is impossible. If `source == target`, the answer is `0`.

Implement `Solution::num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32`.

## Examples

```text
routes = [[1, 2, 7], [3, 6, 7]], source = 1, target = 6                 => 2
routes = [[7, 12], [4, 5, 15], [6], [15, 19], [9, 12, 13]],
         source = 15, target = 12                                         => -1
```

## Constraints

- `1 <= routes.len() <= 500`, `1 <= routes[i].len() <= 10^5`, total stops `<= 10^5`
- The stops of one route are distinct; `0 <= stop, source, target < 10^6`.

## Hints

1. Build `stop -> list of routes` once.
2. BFS where one step is "take one more bus": the first layer is every route through `source`.
3. Expanding a route visits all its stops; mark routes **and** stops as visited so every route and
   stop is processed at most once (otherwise long routes are scanned again and again).

## Starter code

Write your code in [`src/practice/p1000_bus_routes.rs`](../src/practice/p1000_bus_routes.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p1000
```

```rust
use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
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

    fn run(routes: &[&[i32]], source: i32, target: i32) -> i32 {
        Solution::num_buses_to_destination(
            routes.iter().map(|r| r.to_vec()).collect(),
            source,
            target,
        )
    }

    #[test]
    fn examples() {
        assert_eq!(run(&[&[1, 2, 7], &[3, 6, 7]], 1, 6), 2);
        assert_eq!(
            run(
                &[&[7, 12], &[4, 5, 15], &[6], &[15, 19], &[9, 12, 13]],
                15,
                12
            ),
            -1
        );
    }

    #[test]
    fn trivial_cases() {
        assert_eq!(run(&[&[1, 2]], 5, 5), 0);
        assert_eq!(run(&[&[1, 2]], 1, 2), 1);
        assert_eq!(run(&[&[1, 2]], 3, 2), -1);
        assert_eq!(run(&[&[1, 2]], 1, 3), -1);
    }

    #[test]
    fn transfers_along_a_chain() {
        let routes: Vec<Vec<i32>> = (0..500).map(|i| vec![i, i + 1]).collect();
        let refs: Vec<&[i32]> = routes.iter().map(Vec::as_slice).collect();
        assert_eq!(run(&refs, 0, 500), 500);
        assert_eq!(run(&refs, 250, 0), 250);
        assert_eq!(run(&refs, 499, 500), 1);
    }

    #[test]
    fn one_huge_route() {
        let big: Vec<i32> = (0..100_000).collect();
        assert_eq!(run(&[&big, &[99_999, 500_000]], 0, 500_000), 2);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** BFS on routes. Layer `k` contains the routes reachable with exactly `k` buses. For each
route in the current layer, if it serves `target` the answer is `k`; otherwise each of its unseen
stops contributes its unseen routes to the next layer.

**Complexity:** `O(total stops)` time and space: every route and every stop is expanded once.

**Rust concepts:**
- `entry(stop).or_default().push(route)` builds the inverted index.
- `HashSet::insert` returns `false` for an already-visited element, which doubles as the check.
- `std::mem::take(&mut next)` swaps in an empty vector for the next BFS layer.

```rust
use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        if source == target {
            return 0;
        }
        let mut by_stop: HashMap<i32, Vec<usize>> = HashMap::new();
        for (route, stops) in routes.iter().enumerate() {
            for &stop in stops {
                by_stop.entry(stop).or_default().push(route);
            }
        }
        let mut seen_routes = vec![false; routes.len()];
        let mut seen_stops: HashSet<i32> = HashSet::from([source]);
        let mut layer: Vec<usize> = by_stop.get(&source).cloned().unwrap_or_default();
        for &route in &layer {
            seen_routes[route] = true;
        }
        let mut buses = 1;
        while !layer.is_empty() {
            let mut next = Vec::new();
            for &route in &layer {
                for &stop in &routes[route] {
                    if stop == target {
                        return buses;
                    }
                    if !seen_stops.insert(stop) {
                        continue;
                    }
                    for &other in by_stop.get(&stop).into_iter().flatten() {
                        if !seen_routes[other] {
                            seen_routes[other] = true;
                            next.push(other);
                        }
                    }
                }
            }
            layer = std::mem::take(&mut next);
            buses += 1;
        }
        -1
    }
}
```

Full tested solution: [`src/solutions/p1000_bus_routes.rs`](../src/solutions/p1000_bus_routes.rs)

</details>
