# Problem 0763: Count Inversions

- **Difficulty:** Medium
- **Topics:** merge sort with counting (divide and conquer), `u64` results, `to_vec`
- **Source:** Original · [`slice::split_at`](https://doc.rust-lang.org/std/primitive.slice.html#method.split_at)

## Problem

An **inversion** in a sequence is a pair of positions `i < j` with `nums[i] > nums[j]`: two
elements in the wrong order. The number of inversions measures how far a sequence is from being
sorted (it is also the number of swaps bubble sort would make).

Write `count_inversions(nums: &[i32]) -> u64` that returns the number of inversions of `nums`
in `O(n log n)` time. Do not modify the input (it is borrowed immutably). Equal elements do not
form an inversion.

## Examples

```text
count_inversions(&[2, 4, 1, 3, 5]) = 3     (pairs (2, 1), (4, 1), (4, 3))
count_inversions(&[1, 2, 3])       = 0
count_inversions(&[3, 2, 1])       = 3
count_inversions(&[5, 5, 1])       = 2     (each 5 with the 1; the two 5s are not inverted)
```

## Constraints

- `0 <= nums.len() <= 10^5`
- Any `i32` values.
- A reversed array of `10^5` elements has `4_999_950_000` inversions, more than `u32::MAX`.

## Hints

1. Checking every pair is `O(n^2)`: about `5 * 10^9` comparisons for the largest input.
2. Split the sequence in two halves. Inversions are inside the left half, inside the right half,
   or across the halves. The first two kinds are counted recursively.
3. While merging two **sorted** halves, when you take an element from the right half, how many
   elements of the left half are greater than it?

## Starter code

Write your code in [`src/practice/p0763_count_inversions.rs`](../src/practice/p0763_count_inversions.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0763
```

```rust
pub fn count_inversions(nums: &[i32]) -> u64 {
    todo!("Write your solution here")
}
```

## Tests

<details>
<summary>Show the tests used to check your solution</summary>

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn brute_force(nums: &[i32]) -> u64 {
        nums.iter()
            .enumerate()
            .map(|(i, &a)| nums[i + 1..].iter().filter(|&&b| a > b).count() as u64)
            .sum()
    }

    #[test]
    fn example_three_inversions() {
        assert_eq!(count_inversions(&[2, 4, 1, 3, 5]), 3);
    }

    #[test]
    fn example_sorted_and_reversed() {
        assert_eq!(count_inversions(&[1, 2, 3]), 0);
        assert_eq!(count_inversions(&[3, 2, 1]), 3);
    }

    #[test]
    fn example_equal_values() {
        assert_eq!(count_inversions(&[5, 5, 1]), 2);
        assert_eq!(count_inversions(&[2, 2, 2]), 0);
    }

    #[test]
    fn empty_and_single() {
        assert_eq!(count_inversions(&[]), 0);
        assert_eq!(count_inversions(&[i32::MIN]), 0);
    }

    #[test]
    fn input_is_not_modified() {
        let nums = vec![3, 1, 2];
        assert_eq!(count_inversions(&nums), 2);
        assert_eq!(nums, vec![3, 1, 2]);
    }

    #[test]
    fn matches_brute_force() {
        let nums: Vec<i32> = (0..400).map(|i: i32| (i * 7_919) % 101 - 50).collect();
        assert_eq!(count_inversions(&nums), brute_force(&nums));
    }

    #[test]
    fn large_reversed_input_exceeds_u32() {
        let nums: Vec<i32> = (0..100_000).rev().collect();
        assert_eq!(count_inversions(&nums), 4_999_950_000);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** merge sort with counting: sort a copy of the input with merge sort, and let each
merge count the inversions that cross the two halves. When the merge takes `right[j]` while
`left[i..]` is still waiting, every element of `left[i..]` is greater than `right[j]` (the
halves are sorted and `left[i] > right[j]`), and each of them sat before `right[j]` in the
original order: that is `left.len() - i` inversions at once. Ties take from the left first, so
equal elements are never counted. The total is the sum of the two recursive counts and the
cross count.

**Complexity:** `O(n log n)` time, `O(n)` extra space (the copy and one buffer).

**Rust concepts:**
- The function borrows `&[i32]`, so it sorts `nums.to_vec()`, an owned copy, and the caller's
  data stays untouched.
- The count is a `u64` and each partial count is converted with `as u64`; `usize` would also
  work on 64-bit targets, but `u64` states the size explicitly.

```rust
/// Sorts `v` (with `buf` of the same length as scratch space) and returns its inversion count.
fn sort_and_count(v: &mut [i32], buf: &mut [i32]) -> u64 {
    let n = v.len();
    if n <= 1 {
        return 0;
    }
    let mid = n / 2;
    let mut count = sort_and_count(&mut v[..mid], &mut buf[..mid]);
    count += sort_and_count(&mut v[mid..], &mut buf[mid..]);
    let (left, right) = v.split_at(mid);
    let (mut i, mut j) = (0, 0);
    for slot in buf.iter_mut() {
        if j == right.len() || (i < left.len() && left[i] <= right[j]) {
            *slot = left[i];
            i += 1;
        } else {
            // right[j] is smaller than every remaining left element: all of them are inversions.
            count += (left.len() - i) as u64;
            *slot = right[j];
            j += 1;
        }
    }
    v.copy_from_slice(buf);
    count
}

pub fn count_inversions(nums: &[i32]) -> u64 {
    let mut v = nums.to_vec();
    let mut buf = vec![0; v.len()];
    sort_and_count(&mut v, &mut buf)
}
```

Full tested solution: [`src/solutions/p0763_count_inversions.rs`](../src/solutions/p0763_count_inversions.rs)

</details>
