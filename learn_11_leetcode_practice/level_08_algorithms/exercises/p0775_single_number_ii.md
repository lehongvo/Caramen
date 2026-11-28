# Problem 0775: Single Number II

- **Difficulty:** Medium
- **Topics:** counting bits modulo 3, arithmetic shift of negative numbers, `fold`
- **Source:** [LeetCode 137. Single Number II](https://leetcode.com/problems/single-number-ii/)

## Problem

In the vector `nums`, every value appears **exactly three times**, except one value that
appears exactly once. Return that single value. Use linear time and constant extra memory (no
hash map, no sorting).

Implement `Solution::single_number(nums: Vec<i32>) -> i32`.

## Examples

```text
Input:  nums = [2, 2, 3, 2]
Output: 3

Input:  nums = [0, 1, 0, 1, 0, 1, 99]
Output: 99
```

## Constraints

- `1 <= nums.len() <= 3 * 10^4`
- `-2^31 <= nums[i] <= 2^31 - 1`
- Every value appears three times except one, which appears once.

## Hints

1. XOR alone does not help here: three equal values XOR to the value itself.
2. Look at one bit position. How many numbers have that bit set? What is that count modulo 3?

## Starter code

Write your code in [`src/practice/p0775_single_number_ii.rs`](../src/practice/p0775_single_number_ii.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0775
```

```rust
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
    fn example_small() {
        assert_eq!(Solution::single_number(vec![2, 2, 3, 2]), 3);
    }

    #[test]
    fn example_with_zeros() {
        assert_eq!(Solution::single_number(vec![0, 1, 0, 1, 0, 1, 99]), 99);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::single_number(vec![7]), 7);
    }

    #[test]
    fn negative_values() {
        assert_eq!(
            Solution::single_number(vec![-2, -2, 1, 1, 4, 1, 4, 4, -4, -2]),
            -4
        );
    }

    #[test]
    fn extreme_values() {
        assert_eq!(
            Solution::single_number(vec![i32::MAX, i32::MIN, i32::MAX, i32::MAX]),
            i32::MIN
        );
        assert_eq!(
            Solution::single_number(vec![i32::MIN, i32::MAX, i32::MIN, i32::MIN]),
            i32::MAX
        );
    }

    #[test]
    fn long_input() {
        let mut nums: Vec<i32> = (0..9_999).flat_map(|i| [i * 3 - 7_000; 3]).collect();
        nums.push(123_456_789);
        assert_eq!(Solution::single_number(nums), 123_456_789);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** count bits modulo 3: fix a bit position. Each value that appears three times
contributes `0` or `3` to the number of elements with that bit set, so that count modulo 3 is
exactly the bit of the single value. Do this for all 32 positions and assemble the answer.
Negative numbers work too: bit 31 is the sign bit, and setting it with `1 << 31` produces the
negative two's-complement value.

A cleverer constant-space version keeps two masks `ones` and `twos` (bits seen once and twice
modulo 3) and updates them per element with `ones = (ones ^ x) & !twos;
twos = (twos ^ x) & !ones;`, but the counting version is easier to reason about.

**Complexity:** `O(32 * n)` = `O(n)` time, `O(1)` space.

**Rust concepts:**
- `>>` on a signed `i32` is an **arithmetic** shift (it copies the sign bit), and `& 1` then
  isolates the wanted bit, so negative values need no special case.
- `1i32 << 31` is `i32::MIN`: shifting a bit into the sign position is allowed (only shift
  amounts of 32 or more are errors).
- `fold` builds the result bit by bit from an initial `0`.

```rust
pub struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        (0..32).fold(0i32, |result, bit| {
            let count = nums.iter().filter(|&&x| (x >> bit) & 1 == 1).count();
            if count % 3 == 1 {
                result | (1 << bit)
            } else {
                result
            }
        })
    }
}
```

Full tested solution: [`src/solutions/p0775_single_number_ii.rs`](../src/solutions/p0775_single_number_ii.rs)

</details>
