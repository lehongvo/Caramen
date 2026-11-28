# Problem 0051: Number of Good Pairs

- **Difficulty:** Easy
- **Topics:** fixed-size array as a counter `[i32; 101]`, nested index loops, `as usize`
- **Source:** [LeetCode 1512. Number of Good Pairs](https://leetcode.com/problems/number-of-good-pairs/)

## Problem

Given a vector of integers `nums`, a pair of indices `(i, j)` is **good** if `nums[i] == nums[j]`
and `i < j`. Return the number of good pairs.

Implement `Solution::num_identical_pairs(nums: Vec<i32>) -> i32`.

## Examples

```text
Input:  nums = [1, 2, 3, 1, 1, 3]
Output: 4
Explanation: the good pairs are (0, 3), (0, 4), (3, 4) and (2, 5).

Input:  nums = [1, 1, 1, 1]
Output: 6
Explanation: every pair of the four positions is good.

Input:  nums = [1, 2, 3]
Output: 0
```

## Constraints

- `1 <= nums.len() <= 100`
- `1 <= nums[i] <= 100`

## Hints

1. The direct approach checks every pair with two nested loops: `for i in 0..n` and `for j in i + 1..n`.
2. Faster: when you reach a value, it forms a good pair with **every earlier copy** of the same value.
3. The values are small (`1..=100`), so an array `[0; 101]` can count how often each value was seen.

## Starter code

Write your code in [`src/practice/p0051_number_of_good_pairs.rs`](../src/practice/p0051_number_of_good_pairs.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0051
```

```rust
pub struct Solution;

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
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

    fn brute_force(nums: &[i32]) -> i32 {
        let mut count = 0;
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] == nums[j] {
                    count += 1;
                }
            }
        }
        count
    }

    #[test]
    fn example_mixed() {
        assert_eq!(Solution::num_identical_pairs(vec![1, 2, 3, 1, 1, 3]), 4);
    }

    #[test]
    fn example_all_equal() {
        assert_eq!(Solution::num_identical_pairs(vec![1, 1, 1, 1]), 6);
    }

    #[test]
    fn example_all_different() {
        assert_eq!(Solution::num_identical_pairs(vec![1, 2, 3]), 0);
        assert_eq!(Solution::num_identical_pairs(vec![100]), 0);
    }

    #[test]
    fn largest_input_all_equal() {
        // 100 equal values form 100 * 99 / 2 = 4950 pairs.
        assert_eq!(Solution::num_identical_pairs(vec![100; 100]), 4950);
    }

    #[test]
    fn matches_brute_force() {
        for modulus in 1..=20 {
            let nums: Vec<i32> = (0..100).map(|i| (i * 37 % modulus) + 1).collect();
            assert_eq!(
                Solution::num_identical_pairs(nums.clone()),
                brute_force(&nums)
            );
        }
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** scan left to right with `seen[v]` = number of earlier occurrences of `v`. The current
element pairs with each of them, so add `seen[v]` to the answer and then increment it.

**Complexity:** `O(n)` time, `O(1)` space (a fixed array of 101 counters).

**Rust concepts:**
- `[0; 101]` creates an array of 101 zeros on the stack; its length is part of its type
  (`[i32; 101]`). See [The Rust Book 3.2](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-array-type).
- Array indices are `usize`, so the value is converted with `x as usize` (safe: `1..=100`).
- Indexing an array out of bounds panics at run time instead of corrupting memory.
- The test compares the fast version with the nested-loop version on many inputs.

```rust
pub struct Solution;

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut seen = [0; 101];
        let mut pairs = 0;
        for &x in &nums {
            let v = x as usize;
            pairs += seen[v];
            seen[v] += 1;
        }
        pairs
    }
}
```

Full tested solution: [`src/solutions/p0051_number_of_good_pairs.rs`](../src/solutions/p0051_number_of_good_pairs.rs)

</details>
