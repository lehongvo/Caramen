# Problem 0972: Median of Two Sorted Arrays

- **Difficulty:** Hard
- **Topics:** binary search on a partition, sentinels with `i32::MIN`/`i32::MAX` via `Option` defaults, `std::mem::swap` of `Vec`s
- **Source:** [LeetCode 4. Median of Two Sorted Arrays](https://leetcode.com/problems/median-of-two-sorted-arrays/)

## Problem

Given two sorted arrays `nums1` and `nums2` (sizes `m` and `n`, not both empty), return the median
of the combined `m + n` numbers. The overall run time must be `O(log(m + n))`, so merging is not
allowed.

Implement `Solution::find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64`.

## Examples

```text
Input:  nums1 = [1, 3], nums2 = [2]       Output: 2.0
Input:  nums1 = [1, 2], nums2 = [3, 4]    Output: 2.5
```

## Constraints

- `0 <= m, n <= 1000`, `1 <= m + n <= 2000`
- `-10^6 <= nums1[i], nums2[i] <= 10^6`

## Hints

1. Cut `nums1` after `i` elements and `nums2` after `j = (m + n + 1) / 2 - i` elements: the two left
   parts together hold the smaller half.
2. The cut is correct when `left1 <= right2` and `left2 <= right1` (missing neighbours count as
   minus or plus infinity). Binary search `i` over the **shorter** array.
3. Odd total: the median is `max(left1, left2)`. Even: the mean of that and `min(right1, right2)`.

## Starter code

Write your code in [`src/practice/p0972_median_of_two_sorted_arrays.rs`](../src/practice/p0972_median_of_two_sorted_arrays.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0972
```

```rust
pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
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

    fn naive(a: &[i32], b: &[i32]) -> f64 {
        let mut all: Vec<i32> = a.iter().chain(b).copied().collect();
        all.sort_unstable();
        let n = all.len();
        if n % 2 == 1 {
            f64::from(all[n / 2])
        } else {
            (f64::from(all[n / 2 - 1]) + f64::from(all[n / 2])) / 2.0
        }
    }

    #[test]
    fn examples() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
    }

    #[test]
    fn one_empty_array() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![1]), 1.0);
        assert_eq!(Solution::find_median_sorted_arrays(vec![2, 3], vec![]), 2.5);
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![], vec![-1, 0, 7]),
            0.0
        );
    }

    #[test]
    fn duplicates_and_extremes() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 1, 1], vec![1, 1]),
            1.0
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![-1_000_000], vec![1_000_000]),
            0.0
        );
    }

    #[test]
    fn matches_naive() {
        for m in 0..12 {
            for n in 0..12 {
                if m + n == 0 {
                    continue;
                }
                let a: Vec<i32> = (0..m).map(|i| i * 3 - 7).collect();
                let b: Vec<i32> = (0..n).map(|i| i * 2 - 4).collect();
                assert_eq!(
                    Solution::find_median_sorted_arrays(a.clone(), b.clone()),
                    naive(&a, &b),
                    "m = {m}, n = {n}"
                );
            }
        }
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** binary search the number of elements `i` taken from the shorter array. If `left1 >
right2`, too many come from `nums1` (move left); if `left2 > right1`, too few (move right).
Otherwise the partition splits the merged order exactly in half, and the median is read from the
four boundary values.

**Complexity:** `O(log min(m, n))` time, `O(1)` space.

**Rust concepts:**
- `checked_sub(1).map_or(i32::MIN, |k| a[k])` reads "the element before the cut, or minus
  infinity" without signed indices.
- `std::mem::swap(&mut a, &mut b)` makes `a` the shorter vector without copying elements.

```rust
pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (mut a, mut b) = (nums1, nums2);
        if a.len() > b.len() {
            std::mem::swap(&mut a, &mut b);
        }
        let (m, n) = (a.len(), b.len());
        let half = (m + n).div_ceil(2);
        let before = |v: &[i32], cut: usize| cut.checked_sub(1).map_or(i32::MIN, |k| v[k]);
        let after = |v: &[i32], cut: usize| v.get(cut).copied().unwrap_or(i32::MAX);
        let (mut lo, mut hi) = (0, m);
        loop {
            let i = (lo + hi) / 2;
            let j = half - i;
            let (left1, right1) = (before(&a, i), after(&a, i));
            let (left2, right2) = (before(&b, j), after(&b, j));
            if left1 > right2 {
                hi = i - 1;
            } else if left2 > right1 {
                lo = i + 1;
            } else {
                let left = f64::from(left1.max(left2));
                return if (m + n) % 2 == 1 {
                    left
                } else {
                    (left + f64::from(right1.min(right2))) / 2.0
                };
            }
        }
    }
}
```

Full tested solution: [`src/solutions/p0972_median_of_two_sorted_arrays.rs`](../src/solutions/p0972_median_of_two_sorted_arrays.rs)

</details>
