# Problem 0729: Find Smallest Letter Greater Than Target

- **Difficulty:** Easy
- **Topics:** binary search for an upper bound, wrapping around, `char` comparison
- **Source:** [LeetCode 744. Find Smallest Letter Greater Than Target](https://leetcode.com/problems/find-smallest-letter-greater-than-target/)

## Problem

You are given a vector `letters` of lowercase letters sorted in non-decreasing order (it
contains at least two **different** letters) and a lowercase letter `target`. Return the
smallest letter in `letters` that is **strictly greater** than `target`. If no letter is greater,
wrap around and return `letters[0]`.

Implement `Solution::next_greatest_letter(letters: Vec<char>, target: char) -> char`.

## Examples

```text
Input:  letters = ['c', 'f', 'j'], target = 'a'
Output: 'c'

Input:  letters = ['c', 'f', 'j'], target = 'c'
Output: 'f'           (strictly greater: 'c' itself does not count)

Input:  letters = ['x', 'x', 'y', 'y'], target = 'z'
Output: 'x'           (nothing is greater than 'z', so wrap around)
```

## Constraints

- `2 <= letters.len() <= 10^4`
- `letters[i]` and `target` are lowercase English letters.
- `letters` is sorted in non-decreasing order and contains at least two different letters.

## Hints

1. You are looking for the first position whose letter is `> target`: an "upper bound".
2. Every letter `<= target` is to the left of that position and every letter `> target` is at
   or to the right of it. Binary search for the boundary.

## Starter code

Write your code in [`src/practice/p0729_find_smallest_letter_greater_than_target.rs`](../src/practice/p0729_find_smallest_letter_greater_than_target.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0729
```

```rust
pub struct Solution;

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
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
    fn example_before_everything() {
        assert_eq!(
            Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'a'),
            'c'
        );
    }

    #[test]
    fn example_equal_letter_is_skipped() {
        assert_eq!(
            Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'c'),
            'f'
        );
    }

    #[test]
    fn example_wrap_around() {
        assert_eq!(
            Solution::next_greatest_letter(vec!['x', 'x', 'y', 'y'], 'z'),
            'x'
        );
    }

    #[test]
    fn target_equals_last_letter() {
        assert_eq!(
            Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'j'),
            'c'
        );
    }

    #[test]
    fn duplicates_of_the_target() {
        assert_eq!(
            Solution::next_greatest_letter(vec!['a', 'b', 'b', 'b', 'c'], 'b'),
            'c'
        );
    }

    #[test]
    fn long_input() {
        let mut letters = vec!['a'; 5_000];
        letters.extend(vec!['m'; 4_999]);
        letters.push('z');
        assert_eq!(Solution::next_greatest_letter(letters.clone(), 'a'), 'm');
        assert_eq!(Solution::next_greatest_letter(letters.clone(), 'm'), 'z');
        assert_eq!(Solution::next_greatest_letter(letters, 'z'), 'a');
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** binary search for a boundary: the predicate "`letters[i] <= target`" is `true` for
a prefix of the sorted vector and `false` afterwards. Search for the first index where it is
false: if `letters[mid] <= target`, the boundary is after `mid` (`lo = mid + 1`); otherwise
`mid` itself may be the boundary (`hi = mid`). When the loop ends, `lo` is the boundary; if it
equals `len`, no letter is greater and the answer wraps to `letters[0]`.

This "first index where a condition becomes true" search is the most reusable form of binary
search: the next problems use it again, and the standard library calls it `partition_point`.

**Complexity:** `O(log n)` time, `O(1)` space.

**Rust concepts:**
- `char` values compare by Unicode scalar value, which for `'a'..='z'` is alphabetical order.
- `letters.get(lo).copied().unwrap_or(letters[0])` handles the wrap-around without an `if`.

```rust
pub struct Solution;

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let (mut lo, mut hi) = (0, letters.len());
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if letters[mid] <= target {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        letters.get(lo).copied().unwrap_or(letters[0])
    }
}
```

Full tested solution: [`src/solutions/p0729_find_smallest_letter_greater_than_target.rs`](../src/solutions/p0729_find_smallest_letter_greater_than_target.rs)

</details>
