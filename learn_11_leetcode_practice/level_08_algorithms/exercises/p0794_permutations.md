# Problem 0794: Permutations

- **Difficulty:** Medium
- **Topics:** backtracking with a `used` mask, `Vec<bool>`, recursion depth `n`
- **Source:** [LeetCode 46. Permutations](https://leetcode.com/problems/permutations/)

## Problem

Given a vector `nums` of **distinct** integers, return all possible permutations (orderings) of
its values, in any order.

Implement `Solution::permute(nums: Vec<i32>) -> Vec<Vec<i32>>`.

## Examples

```text
Input:  nums = [1, 2, 3]
Output: [[1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2], [3, 2, 1]]

Input:  nums = [0, 1]
Output: [[0, 1], [1, 0]]

Input:  nums = [1]
Output: [[1]]
```

## Constraints

- `1 <= nums.len() <= 6`
- `-10 <= nums[i] <= 10`, all distinct

## Hints

1. Unlike subsets and combinations, order matters: any unused value may come next.
2. Keep a `used` flag per position, set it before recursing and clear it afterwards.

## Starter code

Write your code in [`src/practice/p0794_permutations.rs`](../src/practice/p0794_permutations.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0794
```

```rust
pub struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
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

    fn sorted(mut perms: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        perms.sort();
        perms
    }

    #[test]
    fn example_three_values() {
        assert_eq!(
            sorted(Solution::permute(vec![1, 2, 3])),
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ]
        );
    }

    #[test]
    fn example_two_values() {
        assert_eq!(
            sorted(Solution::permute(vec![0, 1])),
            vec![vec![0, 1], vec![1, 0]]
        );
    }

    #[test]
    fn example_single_value() {
        assert_eq!(Solution::permute(vec![1]), vec![vec![1]]);
    }

    #[test]
    fn six_values_give_720_distinct_permutations() {
        let perms = sorted(Solution::permute(vec![-10, -3, 0, 4, 7, 10]));
        assert_eq!(perms.len(), 720);
        let mut deduped = perms.clone();
        deduped.dedup();
        assert_eq!(deduped.len(), 720);
        assert!(perms.iter().all(|p| {
            let mut s = p.clone();
            s.sort_unstable();
            s == vec![-10, -3, 0, 4, 7, 10]
        }));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** backtracking with a `used` mask: build the permutation one position at a time. For
the next position, try every value that is not used yet: mark it, push it, recurse, then pop
it and unmark it. When `current` holds all `n` values, record a copy. The recursion tree has
`n` choices at the first level, `n - 1` at the second, and so on: `n!` leaves.

**Complexity:** `O(n * n!)` time, `O(n)` extra space besides the output.

**Rust concepts:**
- `used: &mut [bool]` is a mutable slice shared by all calls; the choose / un-choose steps keep
  it consistent.
- `nums.iter().enumerate()` gives both the position (for `used`) and the value.

```rust
pub struct Solution;

fn backtrack(nums: &[i32], used: &mut [bool], current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    if current.len() == nums.len() {
        result.push(current.clone());
        return;
    }
    for (i, &x) in nums.iter().enumerate() {
        if used[i] {
            continue;
        }
        used[i] = true;
        current.push(x);
        backtrack(nums, used, current, result);
        current.pop();
        used[i] = false;
    }
}

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut used = vec![false; nums.len()];
        backtrack(
            &nums,
            &mut used,
            &mut Vec::with_capacity(nums.len()),
            &mut result,
        );
        result
    }
}
```

Full tested solution: [`src/solutions/p0794_permutations.rs`](../src/solutions/p0794_permutations.rs)

</details>
