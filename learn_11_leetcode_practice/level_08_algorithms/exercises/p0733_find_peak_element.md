# Problem 0733: Find Peak Element

- **Difficulty:** Medium
- **Topics:** binary search on a slope (unsorted input), invariants, checking "any valid answer"
- **Source:** [LeetCode 162. Find Peak Element](https://leetcode.com/problems/find-peak-element/)

## Problem

A **peak** is an element strictly greater than its neighbours. Given a vector `nums` in which
no two adjacent values are equal, return the index of **any** peak. Imagine
`nums[-1] = nums[n] = -infinity`, so the first and last elements only need to beat their single
real neighbour.

Your algorithm must run in `O(log n)` time.

Implement `Solution::find_peak_element(nums: Vec<i32>) -> i32`.

## Examples

```text
Input:  nums = [1, 2, 3, 1]
Output: 2             (3 is a peak)

Input:  nums = [1, 2, 1, 3, 5, 6, 4]
Output: 1 or 5        (2 and 6 are both peaks; either index is accepted)
```

## Constraints

- `1 <= nums.len() <= 1000`
- `-2^31 <= nums[i] <= 2^31 - 1`
- `nums[i] != nums[i + 1]` for every valid `i`.

## Hints

1. The array is not sorted, but look at `nums[mid]` and `nums[mid + 1]`. If you walk uphill,
   must you eventually reach a peak?
2. Keep a range that is guaranteed to contain a peak, and halve it.

## Starter code

Write your code in [`src/practice/p0733_find_peak_element.rs`](../src/practice/p0733_find_peak_element.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0733
```

```rust
pub struct Solution;

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
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

    /// Checks that `index` is a peak of `nums`.
    fn assert_peak(nums: &[i32], index: i32) {
        let i = usize::try_from(index).expect("index must be non-negative");
        assert!(i < nums.len(), "index {i} out of bounds");
        assert!(
            i == 0 || nums[i - 1] < nums[i],
            "left neighbour is not smaller at {i}"
        );
        assert!(
            i + 1 == nums.len() || nums[i + 1] < nums[i],
            "right neighbour is not smaller at {i}"
        );
    }

    fn check(nums: Vec<i32>) {
        let index = Solution::find_peak_element(nums.clone());
        assert_peak(&nums, index);
    }

    #[test]
    fn example_single_peak() {
        assert_eq!(Solution::find_peak_element(vec![1, 2, 3, 1]), 2);
    }

    #[test]
    fn example_two_peaks() {
        check(vec![1, 2, 1, 3, 5, 6, 4]);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::find_peak_element(vec![5]), 0);
    }

    #[test]
    fn monotonic_inputs() {
        assert_eq!(Solution::find_peak_element(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(Solution::find_peak_element(vec![5, 4, 3, 2, 1]), 0);
    }

    #[test]
    fn extreme_values() {
        check(vec![i32::MIN, i32::MAX, i32::MIN]);
        check(vec![i32::MAX, i32::MIN]);
    }

    #[test]
    fn zigzag_input() {
        let nums: Vec<i32> = (0..1000).map(|i| if i % 2 == 0 { i } else { -i }).collect();
        check(nums);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** binary search on the slope: compare `nums[mid]` with its right neighbour. If
`nums[mid] < nums[mid + 1]`, the values rise to the right; walking uphill from `mid + 1` must
stop somewhere, and wherever it stops (at the latest at the last index, which beats the
imaginary `-infinity`) is a peak, so a peak exists in `mid + 1..=hi`. Otherwise
`nums[mid] > nums[mid + 1]` and, by the mirrored argument, a peak exists in `lo..=mid`. The
range always contains a peak and shrinks every step; when it has one element, that is a peak.

Binary search does not need sorted data, only a test that tells which half is guaranteed to
contain an answer.

**Complexity:** `O(log n)` time, `O(1)` space.

**Rust concepts:**
- The inclusive range `lo..=hi` is used here because `mid + 1` must stay a valid index:
  `mid < hi` holds inside the loop, so `nums[mid + 1]` never goes out of bounds.
- The tests accept any peak by checking the property instead of a fixed index.

```rust
pub struct Solution;

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let (mut lo, mut hi) = (0, nums.len() - 1); // a peak exists in lo..=hi
        while lo < hi {
            let mid = lo + (hi - lo) / 2; // mid < hi, so mid + 1 is valid
            if nums[mid] < nums[mid + 1] {
                lo = mid + 1; // uphill to the right
            } else {
                hi = mid; // downhill to the right: mid or something left of it is a peak
            }
        }
        lo as i32
    }
}
```

Full tested solution: [`src/solutions/p0733_find_peak_element.rs`](../src/solutions/p0733_find_peak_element.rs)

</details>
