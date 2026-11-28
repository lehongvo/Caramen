# Problem 0496: Split Array into Fibonacci Sequence

- **Difficulty:** Medium
- **Topics:** trying candidates with `find_map`, `checked_add` + `?` inside an `Option`-returning helper, parsing with overflow detection, `starts_with`
- **Source:** [LeetCode 842. Split Array into Fibonacci Sequence](https://leetcode.com/problems/split-array-into-fibonacci-sequence/)

## Problem

`num` is a string of digits. Split it into a *Fibonacci-like* sequence `f`:

- `f` has at least 3 numbers, each `0 <= f[i] < 2^31` (it fits in an `i32`);
- `f[i] + f[i + 1] == f[i + 2]` for every valid `i`;
- writing the numbers one after another gives exactly `num`, and no number has a leading zero
  (except the number `0` itself).

Return any such sequence, or an empty vector if none exists.

Implement `Solution::split_into_fibonacci(num: String) -> Vec<i32>`.

## Examples

```text
Input:  num = "1101111"
Output: [11, 0, 11, 11]          ([110, 1, 111] would also be accepted)

Input:  num = "112358130"
Output: []

Input:  num = "0123"
Output: []                        ("01" has a leading zero; [0, 1, 2, 3] is not Fibonacci-like)
```

## Constraints

- `1 <= num.len() <= 200`
- `num` contains only digits.

## Hints

1. The first two numbers determine the whole sequence. Try every length for them (each has at
   most 10 digits, since it must fit in an `i32`).
2. Write a helper `fn extend(num, a, b, pos) -> Option<Vec<i32>>` that keeps computing the next
   number with `checked_add(...)?` and checks that `num[pos..]` starts with its decimal text.
3. `find_map` over all `(i, j)` length pairs returns the first sequence that works.

## Starter code

Write your code in [`src/practice/p0496_split_array_into_fibonacci_sequence.rs`](../src/practice/p0496_split_array_into_fibonacci_sequence.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0496
```

```rust
pub struct Solution;

impl Solution {
    pub fn split_into_fibonacci(num: String) -> Vec<i32> {
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

    /// Checks every rule of the statement for a returned sequence.
    fn is_valid(num: &str, seq: &[i32]) -> bool {
        seq.len() >= 3
            && seq
                .windows(3)
                .all(|w| i64::from(w[0]) + i64::from(w[1]) == i64::from(w[2]))
            && seq.iter().map(|x| x.to_string()).collect::<String>() == num
            && seq.iter().all(|&x| x >= 0)
    }

    #[test]
    fn example_with_zero() {
        let seq = Solution::split_into_fibonacci("1101111".to_string());
        assert!(is_valid("1101111", &seq), "{seq:?}");
    }

    #[test]
    fn example_impossible() {
        assert_eq!(
            Solution::split_into_fibonacci("112358130".to_string()),
            Vec::<i32>::new()
        );
    }

    #[test]
    fn example_leading_zero() {
        assert_eq!(
            Solution::split_into_fibonacci("0123".to_string()),
            Vec::<i32>::new()
        );
    }

    #[test]
    fn classic_and_zero_sequences() {
        let seq = Solution::split_into_fibonacci("11235813".to_string());
        assert_eq!(seq, vec![1, 1, 2, 3, 5, 8, 13]);
        assert_eq!(
            Solution::split_into_fibonacci("000".to_string()),
            vec![0, 0, 0]
        );
        assert_eq!(
            Solution::split_into_fibonacci("12".to_string()),
            Vec::<i32>::new()
        );
    }

    #[test]
    fn overflow_is_rejected() {
        // 1, 2147483646, 2147483647 is valid; the next number would overflow, but none is needed.
        let ok = "121474836462147483647";
        let seq = Solution::split_into_fibonacci(ok.to_string());
        assert!(is_valid(ok, &seq), "{seq:?}");
        // Here the third number (2147483648) does not fit in an i32.
        assert_eq!(
            Solution::split_into_fibonacci("214748364712147483648".to_string()),
            Vec::<i32>::new()
        );
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** enumerate the lengths of the first two numbers. Each choice fixes every later number,
so checking it is a linear walk through the string. `Option` and `?` make every failure
(leading zero, overflow, mismatch) a one-liner, and `find_map` stops at the first success.

**Complexity:** `O(10 * 10 * n)` time, `O(n)` space.

**Rust concepts:**
- `parse::<i32>()` rejects values `>= 2^31` (they do not fit), so the range rule comes for free.
- `checked_add` returns `None` on overflow; with `?` the helper gives up immediately.
- `str::starts_with(&text)` compares the expected next number with the remaining input.
- `(1..=10).flat_map(|i| (1..=10).map(move |j| (i, j)))` + `find_map` is a lazy double loop that
  stops at the first answer.

```rust
pub struct Solution;

fn parse_number(s: &str) -> Option<i32> {
    if s.len() > 1 && s.starts_with('0') {
        return None;
    }
    s.parse().ok()
}

fn extend(num: &str, a: i32, b: i32, mut pos: usize) -> Option<Vec<i32>> {
    let mut seq = vec![a, b];
    while pos < num.len() {
        let next = seq[seq.len() - 2].checked_add(seq[seq.len() - 1])?;
        let text = next.to_string();
        if !num[pos..].starts_with(&text) {
            return None;
        }
        pos += text.len();
        seq.push(next);
    }
    Some(seq)
}

impl Solution {
    pub fn split_into_fibonacci(num: String) -> Vec<i32> {
        let n = num.len();
        (1..=10)
            .flat_map(|i| (1..=10).map(move |j| (i, j)))
            .filter(|&(i, j)| i + j < n)
            .find_map(|(i, j)| {
                let a = parse_number(&num[..i])?;
                let b = parse_number(&num[i..i + j])?;
                extend(&num, a, b, i + j)
            })
            .unwrap_or_default()
    }
}
```

Full tested solution: [`src/solutions/p0496_split_array_into_fibonacci_sequence.rs`](../src/solutions/p0496_split_array_into_fibonacci_sequence.rs)

</details>
