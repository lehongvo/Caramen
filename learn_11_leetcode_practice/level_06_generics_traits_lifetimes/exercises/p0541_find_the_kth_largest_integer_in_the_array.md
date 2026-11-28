# Problem 0541: Find the Kth Largest Integer in the Array

- **Difficulty:** Medium
- **Topics:** newtype `NumStr<'a>(&'a str)` with a manual `Ord`, numeric order vs string order, `select_nth_unstable`
- **Source:** [LeetCode 1985. Find the Kth Largest Integer in the Array](https://leetcode.com/problems/find-the-kth-largest-integer-in-the-array/)

## Problem

`nums` holds non-negative integers written as strings, without leading zeros, possibly far too long
for any integer type (up to 100 digits). Return the string of the `k`-th largest integer.
Duplicates count separately: in `["1", "2", "2"]` the largest is `"2"`, the 2nd largest is `"2"` and
the 3rd largest is `"1"`.

Implement `Solution::kth_largest_number(nums: Vec<String>, k: i32) -> String`.

Comparing the strings directly is wrong (`"10" < "9"` as strings). Wrap each string in a newtype
`NumStr<'a>(pub &'a str)` that borrows it, and implement `Ord` so that `NumStr` values compare as
**numbers**: a shorter string is a smaller number; strings of equal length compare character by
character.

## Examples

```text
Input:  nums = ["3", "6", "7", "10"], k = 4      Output: "3"
Input:  nums = ["2", "21", "12", "1"], k = 3     Output: "2"
Input:  nums = ["0", "0"], k = 2                 Output: "0"
```

## Constraints

- `1 <= k <= nums.len() <= 10^4`
- `1 <= nums[i].len() <= 100`, only digits, no leading zeros (except the number `"0"` itself)

## Hints

1. Without leading zeros, more digits means a bigger number.
2. `self.0.len().cmp(&other.0.len()).then_with(|| self.0.cmp(other.0))`.
3. Once `NumStr: Ord`, a vector of them can be sorted (or partially selected) with std methods.

## Starter code

Write your code in [`src/practice/p0541_find_the_kth_largest_integer_in_the_array.rs`](../src/practice/p0541_find_the_kth_largest_integer_in_the_array.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0541
```

```rust
use std::cmp::Ordering;

pub struct Solution;

/// A decimal number stored as a string, ordered numerically.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumStr<'a>(pub &'a str);

impl Ord for NumStr<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        todo!("Write your solution here")
    }
}

impl PartialOrd for NumStr<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        todo!("Write your solution here")
    }
}

impl Solution {
    pub fn kth_largest_number(nums: Vec<String>, k: i32) -> String {
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

    fn strings(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn example_inputs() {
        assert_eq!(
            Solution::kth_largest_number(strings(&["3", "6", "7", "10"]), 4),
            "3"
        );
        assert_eq!(
            Solution::kth_largest_number(strings(&["2", "21", "12", "1"]), 3),
            "2"
        );
        assert_eq!(Solution::kth_largest_number(strings(&["0", "0"]), 2), "0");
    }

    #[test]
    fn numeric_order_differs_from_string_order() {
        assert!(NumStr("10") > NumStr("9"));
        assert!("10" < "9");
        assert!(NumStr("123") < NumStr("124"));
        assert_eq!(NumStr("77").cmp(&NumStr("77")), Ordering::Equal);
        let mut v = vec![NumStr("100"), NumStr("9"), NumStr("25"), NumStr("0")];
        v.sort();
        assert_eq!(
            v,
            vec![NumStr("0"), NumStr("9"), NumStr("25"), NumStr("100")]
        );
    }

    #[test]
    fn huge_numbers_and_duplicates() {
        let big = format!("9{}", "0".repeat(99));
        let bigger = format!("1{}", "0".repeat(99));
        let nums = vec![
            big.clone(),
            "5".to_string(),
            bigger.clone(),
            "5".to_string(),
        ];
        assert_eq!(Solution::kth_largest_number(nums.clone(), 1), big);
        assert_eq!(Solution::kth_largest_number(nums.clone(), 2), bigger);
        assert_eq!(Solution::kth_largest_number(nums.clone(), 3), "5");
        assert_eq!(Solution::kth_largest_number(nums, 4), "5");
    }

    #[test]
    fn many_values() {
        let nums: Vec<String> = (0..10_000_u64)
            .map(|i| (i * 7_919 % 10_000).to_string())
            .collect();
        // 7919 and 10000 are coprime, so every number 0..10000 appears exactly once.
        assert_eq!(Solution::kth_largest_number(nums.clone(), 1), "9999");
        assert_eq!(Solution::kth_largest_number(nums.clone(), 10_000), "0");
        assert_eq!(Solution::kth_largest_number(nums, 1_234), "8766");
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** order the numeric strings by `(length, text)`. Wrap every input string in a borrowing
`NumStr`, then use `select_nth_unstable_by` to put the `k`-th largest at index `k - 1` of a
descending order without sorting everything.

**Complexity:** `O(n * d)` average time for `n` strings of up to `d` digits (selection is linear on
average, each comparison is `O(d)`); `O(n)` extra space for the wrappers.

**Rust concepts:**
- A newtype gives an existing type (`&str`) a **different** `Ord` without touching `str`'s own
  ordering; the wrapper costs nothing at runtime.
- `NumStr<'a>` holds a borrowed `&'a str`, so building the wrappers copies no characters; the
  lifetime ties every wrapper to `nums`.
- `PartialEq`/`Eq` are derived (comparing the strings), which is consistent with this `Ord` because
  two strings are `Equal` under it exactly when they are identical.

```rust
use std::cmp::Ordering;

pub struct Solution;

/// A decimal number stored as a string, ordered numerically.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumStr<'a>(pub &'a str);

impl Ord for NumStr<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0
            .len()
            .cmp(&other.0.len())
            .then_with(|| self.0.cmp(other.0))
    }
}

impl PartialOrd for NumStr<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn kth_largest_number(nums: Vec<String>, k: i32) -> String {
        let mut wrapped: Vec<NumStr> = nums.iter().map(|s| NumStr(s)).collect();
        let index = k as usize - 1;
        let (_, kth, _) = wrapped.select_nth_unstable_by(index, |a, b| b.cmp(a));
        kth.0.to_string()
    }
}
```

Full tested solution: [`src/solutions/p0541_find_the_kth_largest_integer_in_the_array.rs`](../src/solutions/p0541_find_the_kth_largest_integer_in_the_array.rs)

</details>
