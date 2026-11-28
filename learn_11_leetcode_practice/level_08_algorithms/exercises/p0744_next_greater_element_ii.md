# Problem 0744: Next Greater Element II

- **Difficulty:** Medium
- **Topics:** circular monotonic stack, two passes with `i % n`, `pop_if`
- **Source:** [LeetCode 503. Next Greater Element II](https://leetcode.com/problems/next-greater-element-ii/)

## Problem

Given a **circular** vector `nums` (the element after the last one is `nums[0]`), return for
every element its next greater element: the first strictly greater value found by walking
forward from it, wrapping around the end. If no element is greater, the answer is `-1`.

Implement `Solution::next_greater_elements(nums: Vec<i32>) -> Vec<i32>`.

## Examples

```text
Input:  nums = [1, 2, 1]
Output: [2, -1, 2]      (the last 1 wraps around and finds 2)

Input:  nums = [1, 2, 3, 4, 3]
Output: [2, 3, 4, -1, 4]
```

## Constraints

- `1 <= nums.len() <= 10^4`
- `-10^9 <= nums[i] <= 10^9`

## Hints

1. Without the wrap-around this is the monotonic stack of the previous problems.
2. Walking over the array **twice** lets every element see all the others after it.

## Starter code

Write your code in [`src/practice/p0744_next_greater_element_ii.rs`](../src/practice/p0744_next_greater_element_ii.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0744
```

```rust
pub struct Solution;

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
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
    fn example_wrap_around() {
        assert_eq!(
            Solution::next_greater_elements(vec![1, 2, 1]),
            vec![2, -1, 2]
        );
    }

    #[test]
    fn example_longer() {
        assert_eq!(
            Solution::next_greater_elements(vec![1, 2, 3, 4, 3]),
            vec![2, 3, 4, -1, 4]
        );
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::next_greater_elements(vec![5]), vec![-1]);
    }

    #[test]
    fn all_equal() {
        assert_eq!(
            Solution::next_greater_elements(vec![3, 3, 3]),
            vec![-1, -1, -1]
        );
    }

    #[test]
    fn decreasing_wraps_to_the_front() {
        assert_eq!(
            Solution::next_greater_elements(vec![5, 4, 3, 2, 1]),
            vec![-1, 5, 5, 5, 5]
        );
    }

    #[test]
    fn extreme_values_and_long_input() {
        let mut nums = vec![-1_000_000_000; 9_999];
        nums.insert(0, 1_000_000_000);
        let answer = Solution::next_greater_elements(nums);
        assert_eq!(answer[0], -1);
        assert!(answer[1..].iter().all(|&x| x == 1_000_000_000));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** circular monotonic stack: iterate `k` over `0..2 * n` and use `i = k % n`, which
visits the array twice. The stack holds indices waiting for a greater value; whenever
`nums[i]` is greater than the value on top, that index gets its answer. Indices are pushed only
during the first pass (`k < n`): the second pass only serves as "the values after the end".
Anything left on the stack after both passes has no greater element and keeps `-1`.

**Complexity:** `O(n)` time (each index pushed once, popped at most once), `O(n)` space.

**Rust concepts:**
- `(0..2 * n).map(|k| k % n)` models the circular walk without copying the array.
- The answer vector is initialised with `-1`, the value for elements that never get popped.

```rust
pub struct Solution;

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut answer = vec![-1; n];
        let mut waiting: Vec<usize> = Vec::with_capacity(n);
        for k in 0..2 * n {
            let i = k % n;
            while let Some(j) = waiting.pop_if(|j| nums[*j] < nums[i]) {
                answer[j] = nums[i];
            }
            if k < n {
                waiting.push(i);
            }
        }
        answer
    }
}
```

Full tested solution: [`src/solutions/p0744_next_greater_element_ii.rs`](../src/solutions/p0744_next_greater_element_ii.rs)

</details>
