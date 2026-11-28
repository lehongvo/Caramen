# Problem 0463: Slowest Key

- **Difficulty:** Easy
- **Topics:** `iter::once` + `chain` + `windows`, `zip` with `chars`, `max` on tuples for tie-breaking
- **Source:** [LeetCode 1629. Slowest Key](https://leetcode.com/problems/slowest-key/)

## Problem

A tester pressed `n` keys one after another. `keys_pressed[i]` is the `i`-th key and
`release_times[i]` is the moment it was released. The first key was pressed at time `0`, and
every later key was pressed exactly when the previous one was released. So key `0` was held for
`release_times[0]` and key `i > 0` for `release_times[i] - release_times[i - 1]`.

Return the key with the **longest** hold duration. If several presses tie, return the
lexicographically **largest** key among them. (The same key may be pressed several times.)

Implement `Solution::slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char`.

## Examples

```text
Input:  release_times = [9, 29, 49, 50], keys_pressed = "cbcd"
Output: 'c'
Explanation: durations are c: 9, b: 20, c: 20, d: 1. 'b' and 'c' tie with 20; 'c' is larger.

Input:  release_times = [12, 23, 36, 46, 62], keys_pressed = "spuda"
Output: 'a'
Explanation: durations are 12, 11, 13, 10, 16; 'a' was held the longest.
```

## Constraints

- `release_times.len() == keys_pressed.len() == n`, `2 <= n <= 1000`
- `1 <= release_times[i] <= 10^9`, strictly increasing
- `keys_pressed` contains lowercase English letters.

## Hints

1. The durations are `release_times[0]` followed by the differences of neighbours:
   `once(release_times[0]).chain(release_times.windows(2).map(|w| w[1] - w[0]))`.
2. Tuples compare lexicographically, so the maximum of `(duration, key)` pairs is the longest
   duration, with ties broken by the largest key.

## Starter code

Write your code in [`src/practice/p0463_slowest_key.rs`](../src/practice/p0463_slowest_key.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0463
```

```rust
pub struct Solution;

impl Solution {
    pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
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
    fn example_tie_goes_to_larger_key() {
        assert_eq!(
            Solution::slowest_key(vec![9, 29, 49, 50], "cbcd".to_string()),
            'c'
        );
    }

    #[test]
    fn example_last_key_is_slowest() {
        assert_eq!(
            Solution::slowest_key(vec![12, 23, 36, 46, 62], "spuda".to_string()),
            'a'
        );
    }

    #[test]
    fn first_key_can_win() {
        assert_eq!(Solution::slowest_key(vec![100, 101], "za".to_string()), 'z');
        assert_eq!(Solution::slowest_key(vec![5, 10], "ab".to_string()), 'b');
        assert_eq!(Solution::slowest_key(vec![5, 10], "ba".to_string()), 'b');
    }

    #[test]
    fn large_times() {
        assert_eq!(
            Solution::slowest_key(vec![1, 999_999_999, 1_000_000_000], "xyz".to_string()),
            'y'
        );
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** build the stream of durations, pair each with its key, and take the maximum pair. The
tuple ordering encodes the whole rule ("longest, then largest key") without a custom comparator.

**Complexity:** `O(n)` time, `O(1)` extra space.

**Rust concepts:**
- `iter::once(x).chain(...)` prepends a single value to an iterator.
- `zip` pairs two iterators element by element (here `i32`s and `char`s).
- `(i32, char)` implements `Ord` because both parts do, so `Iterator::max` works directly.
- The input is never empty, so `max()` returns `Some`; `map_or` keeps the code panic-free.

```rust
pub struct Solution;

impl Solution {
    pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
        let first = release_times.first().copied().unwrap_or(0);
        let durations = std::iter::once(first).chain(release_times.windows(2).map(|w| w[1] - w[0]));
        durations
            .zip(keys_pressed.chars())
            .max()
            .map_or(' ', |(_, key)| key)
    }
}
```

Full tested solution: [`src/solutions/p0463_slowest_key.rs`](../src/solutions/p0463_slowest_key.rs)

</details>
