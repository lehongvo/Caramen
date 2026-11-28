# Problem 0331: Single Number

- **Difficulty:** Easy
- **Topics:** toggling membership in a `HashSet`, `remove`, `into_iter().next()`, XOR
- **Source:** [LeetCode 136. Single Number](https://leetcode.com/problems/single-number/)

## Problem

In the non-empty vector `nums`, every value appears **exactly twice**, except one value that
appears exactly once. Return that single value.

LeetCode's follow-up asks for linear time and constant extra memory. Solve it first with a
`HashSet`, then think about the follow-up (the Solution section shows both).

Implement `Solution::single_number(nums: Vec<i32>) -> i32`.

## Examples

```text
Input:  nums = [2, 2, 1]
Output: 1

Input:  nums = [4, 1, 2, 1, 2]
Output: 4

Input:  nums = [1]
Output: 1
```

## Constraints

- `1 <= nums.len() <= 3 * 10^4`
- `-3 * 10^4 <= nums[i] <= 3 * 10^4`
- Every element appears twice except for one element, which appears once.

## Hints

1. Keep a set of the values seen an **odd** number of times so far: insert a value the first
   time, remove it the second time.
2. At the end, exactly one value is left in the set.

## Starter code

Write your code in [`src/practice/p0331_single_number.rs`](../src/practice/p0331_single_number.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0331
```

```rust
use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
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
    fn example_one() {
        assert_eq!(Solution::single_number(vec![2, 2, 1]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4);
    }

    #[test]
    fn example_three_single_element() {
        assert_eq!(Solution::single_number(vec![1]), 1);
    }

    #[test]
    fn negative_and_zero() {
        assert_eq!(Solution::single_number(vec![0, -30_000, 0]), -30_000);
        assert_eq!(Solution::single_number(vec![-1, 0, -1]), 0);
    }

    #[test]
    fn large_input() {
        let mut nums: Vec<i32> = (0..7_499).flat_map(|i| [i, -i - 1]).collect();
        nums.extend((0..7_499).rev());
        nums.extend((0..7_499).map(|i| -i - 1));
        nums.push(29_999);
        assert_eq!(nums.len(), 4 * 7_499 + 1);
        assert_eq!(Solution::single_number(nums), 29_999);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** toggle every value in a set. `insert` returns `false` when the value is already
present, and in that case we remove it instead. Values seen twice cancel out; the single value
stays, and `into_iter().next()` takes it out.

**Complexity:** `O(n)` average time, `O(n)` space.

The constant-space follow-up uses XOR: `x ^ x == 0` and `x ^ 0 == x`, and XOR is commutative,
so `nums.iter().fold(0, |acc, &x| acc ^ x)` cancels every pair and leaves the single value.

**Rust concepts:**
- `if !set.insert(x) { set.remove(&x); }` uses the `bool` returned by `insert` to decide.
- `remove` takes a reference to the key (`&x`), because it only needs to look at it.
- `into_iter().next()` returns an `Option`; `unwrap_or_default()` avoids a panic even though
  the constraints guarantee one value remains.

```rust
use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut odd: HashSet<i32> = HashSet::new();
        for x in nums {
            if !odd.insert(x) {
                odd.remove(&x);
            }
        }
        odd.into_iter().next().unwrap_or_default()
    }
}
```

Full tested solution: [`src/solutions/p0331_single_number.rs`](../src/solutions/p0331_single_number.rs)

</details>
