# Problem 0759: Gas Station

- **Difficulty:** Medium
- **Topics:** greedy restart, running sums, `zip` + `enumerate`
- **Source:** [LeetCode 134. Gas Station](https://leetcode.com/problems/gas-station/)

## Problem

There are `n` gas stations on a circular road. Station `i` provides `gas[i]` units of fuel, and
driving from station `i` to station `i + 1` (or from the last station to station `0`) costs
`cost[i]` units. Your car has an unlimited tank and starts empty at a station of your choice.

Return the index of the starting station from which you can drive around the whole circle
once, clockwise, without the tank ever going negative. Return `-1` if it is impossible. If a
solution exists, it is unique.

Implement `Solution::can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32`.

## Examples

```text
Input:  gas = [1, 2, 3, 4, 5], cost = [3, 4, 5, 1, 2]
Output: 3
        (start at 3 with 4 fuel: 4-1+5 = 8, 8-2+1 = 7, 7-3+2 = 6, 6-4+3 = 5, 5-5 = 0 back at 3)

Input:  gas = [2, 3, 4], cost = [3, 4, 3]
Output: -1            (total gas 9 < total cost 10)
```

## Constraints

- `n == gas.len() == cost.len()` and `1 <= n <= 10^5`
- `0 <= gas[i], cost[i] <= 10^4`

## Hints

1. If the total gas is smaller than the total cost, no start works. Otherwise some start does.
2. If you start at `a` and run dry before reaching `b`, then no station between `a` and `b`
   can be the start either. Where should you try next?

## Starter code

Write your code in [`src/practice/p0759_gas_station.rs`](../src/practice/p0759_gas_station.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0759
```

```rust
pub struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
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
    fn example_start_at_three() {
        assert_eq!(
            Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
            3
        );
    }

    #[test]
    fn example_impossible() {
        assert_eq!(
            Solution::can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]),
            -1
        );
    }

    #[test]
    fn single_station() {
        assert_eq!(Solution::can_complete_circuit(vec![5], vec![4]), 0);
        assert_eq!(Solution::can_complete_circuit(vec![4], vec![5]), -1);
        assert_eq!(Solution::can_complete_circuit(vec![0], vec![0]), 0);
    }

    #[test]
    fn start_at_zero() {
        assert_eq!(
            Solution::can_complete_circuit(vec![3, 1, 1], vec![1, 2, 2]),
            0
        );
    }

    #[test]
    fn start_at_the_last_station() {
        assert_eq!(
            Solution::can_complete_circuit(vec![1, 1, 5], vec![2, 2, 1]),
            2
        );
    }

    #[test]
    fn long_circle() {
        // Every station loses 1 unit except the last, which gains 10_000: only the last one works.
        let n = 10_001;
        let mut gas = vec![0; n];
        gas[n - 1] = 10_000;
        let mut cost = vec![1; n];
        cost[n - 1] = 0;
        assert_eq!(Solution::can_complete_circuit(gas, cost), (n - 1) as i32);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** greedy restart: drive from a candidate start keeping `tank`, the fuel balance. If
the tank goes negative right after station `i`, the candidate fails, and so does every station
between the candidate and `i`: each of them would arrive at `i` with no more fuel than the
candidate had (the candidate reached each of them with a non-negative tank). So the next
candidate is `i + 1`, with an empty tank. Separately, sum all `gas[i] - cost[i]`: when the total
is negative there is no answer; otherwise the last candidate is the answer (the fuel saved
before it covers the deficit after it).

**Complexity:** `O(n)` time, `O(1)` space.

**Rust concepts:**
- `gas.iter().zip(&cost).enumerate()` walks both vectors in lockstep with the index.
- The total fits in `i32`: at most `10^5 * 10^4 = 10^9` in absolute value.

```rust
pub struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let (mut total, mut tank, mut start) = (0, 0, 0);
        for (i, (&g, &c)) in gas.iter().zip(&cost).enumerate() {
            total += g - c;
            tank += g - c;
            if tank < 0 {
                start = i + 1; // no station up to i can be the start
                tank = 0;
            }
        }
        if total < 0 { -1 } else { start as i32 }
    }
}
```

Full tested solution: [`src/solutions/p0759_gas_station.rs`](../src/solutions/p0759_gas_station.rs)

</details>
