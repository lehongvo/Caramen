# Problem 0012: Minimum Cuts to Divide a Circle

- **Difficulty:** Easy
- **Topics:** `if` / `else if` / `else` as an expression, `%`, case analysis
- **Source:** [LeetCode 2481. Minimum Cuts to Divide a Circle](https://leetcode.com/problems/minimum-cuts-to-divide-a-circle/)

## Problem

You want to cut a round cake into `n` slices of exactly the same size and shape. Two kinds of
straight cuts are allowed:

- a **full cut**: a line through the center that touches the edge at two opposite points;
- a **half cut**: a line from the center to a single point on the edge.

Return the minimum number of cuts needed to obtain `n` equal slices.

Implement `Solution::number_of_cuts(n: i32) -> i32`.

## Examples

```text
Input:  n = 4
Output: 2
Explanation: two full cuts at right angles give 4 equal slices.

Input:  n = 3
Output: 3
Explanation: an odd number of slices cannot use full cuts symmetrically, so 3 half cuts are needed.

Input:  n = 1
Output: 0
Explanation: the whole cake is already one slice.
```

## Constraints

- `1 <= n <= 100`

## Hints

1. A full cut creates two slice boundaries at once; a half cut creates one.
2. There are three cases: `n == 1`, `n` even, `n` odd.

## Starter code

Write your code in [`src/practice/p0012_minimum_cuts_to_divide_a_circle.rs`](../src/practice/p0012_minimum_cuts_to_divide_a_circle.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0012
```

```rust
pub struct Solution;

impl Solution {
    pub fn number_of_cuts(n: i32) -> i32 {
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
    fn example_four_slices() {
        assert_eq!(Solution::number_of_cuts(4), 2);
    }

    #[test]
    fn example_three_slices() {
        assert_eq!(Solution::number_of_cuts(3), 3);
    }

    #[test]
    fn example_one_slice() {
        assert_eq!(Solution::number_of_cuts(1), 0);
    }

    #[test]
    fn two_slices_need_one_full_cut() {
        assert_eq!(Solution::number_of_cuts(2), 1);
    }

    #[test]
    fn largest_values() {
        assert_eq!(Solution::number_of_cuts(100), 50);
        assert_eq!(Solution::number_of_cuts(99), 99);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** `n` equal slices need `n` boundaries from the center. When `n` is even the boundaries
come in opposite pairs, so `n / 2` full cuts suffice. When `n` is odd no boundary has an
opposite partner, so `n` half cuts are needed. One slice needs no cut at all.

**Complexity:** `O(1)` time, `O(1)` space.

**Rust concepts:**
- `if ... else if ... else` is one expression; each branch yields an `i32`, and the value of
  the chosen branch is the function's result.
- The special case `n == 1` must be checked first, because 1 is odd and the odd branch would
  return 1.

```rust
pub struct Solution;

impl Solution {
    pub fn number_of_cuts(n: i32) -> i32 {
        if n == 1 {
            0
        } else if n % 2 == 0 {
            n / 2
        } else {
            n
        }
    }
}
```

Full tested solution: [`src/solutions/p0012_minimum_cuts_to_divide_a_circle.rs`](../src/solutions/p0012_minimum_cuts_to_divide_a_circle.rs)

</details>
