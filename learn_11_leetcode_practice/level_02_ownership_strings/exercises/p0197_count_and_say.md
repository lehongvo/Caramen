# Problem 0197: Count and Say

- **Difficulty:** Medium
- **Topics:** building a new `String` from the previous one, `slice::chunk_by`, replacing an owned value in a loop
- **Source:** [LeetCode 38. Count and Say](https://leetcode.com/problems/count-and-say/)

## Problem

The *count-and-say* sequence starts with `"1"`. Each next term describes the previous term by
reading it aloud group by group: every run of equal digits is written as the length of the run
followed by the digit.

- term 1: `"1"`
- term 2: `"11"` (one 1)
- term 3: `"21"` (two 1s)
- term 4: `"1211"` (one 2, one 1)
- term 5: `"111221"` (one 1, one 2, two 1s)

Implement `Solution::count_and_say(n: i32) -> String`, which returns the `n`-th term.

## Examples

```text
Input:  n = 1
Output: "1"

Input:  n = 4
Output: "1211"

Input:  n = 6
Output: "312211"         (three 1s, two 2s, one 1)
```

## Constraints

- `1 <= n <= 30`

## Hints

1. Keep the current term in a `String`; build the next term in a new `String`, then replace the
   current one with it (`current = next;` drops the old term).
2. `current.as_bytes().chunk_by(|a, b| a == b)` yields each run of equal digits as a `&[u8]`.
3. For each run, push its length (`run.len().to_string()`) and then its digit (`run[0] as char`).

## Starter code

Write your code in [`src/practice/p0197_count_and_say.rs`](../src/practice/p0197_count_and_say.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0197
```

```rust
pub struct Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
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
    fn example_first_term() {
        assert_eq!(Solution::count_and_say(1), "1");
    }

    #[test]
    fn example_fourth_term() {
        assert_eq!(Solution::count_and_say(4), "1211");
    }

    #[test]
    fn example_sixth_term() {
        assert_eq!(Solution::count_and_say(6), "312211");
    }

    #[test]
    fn first_ten_terms() {
        let expected = [
            "1",
            "11",
            "21",
            "1211",
            "111221",
            "312211",
            "13112221",
            "1113213211",
            "31131211131221",
            "13211311123113112211",
        ];
        for (i, term) in expected.iter().enumerate() {
            assert_eq!(Solution::count_and_say(i as i32 + 1), *term);
        }
    }

    #[test]
    fn largest_term() {
        let term = Solution::count_and_say(30);
        assert_eq!(term.len(), 4_462);
        // Every term after the first has an even length (pairs of count and digit), and the
        // sequence never contains a digit other than 1, 2 or 3.
        assert!(term.bytes().all(|b| matches!(b, b'1' | b'2' | b'3')));
        assert_eq!(Solution::count_and_say(29).len(), 3_410);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** simulate the sequence `n - 1` times. Each step groups the current term into runs with
`chunk_by` and writes "length, digit" for every run into a fresh `String`, which then replaces the
current term.

**Complexity:** `O(L)` time and space per step, where `L` is the length of the term (the terms
grow roughly by 30% per step; term 30 has 4462 digits).

**Rust concepts:**
- The `for` loop borrows `current` through `as_bytes()`; that borrow ends with the loop, so the
  following `current = next;` (a move) is allowed.
- `slice::chunk_by` (stable since Rust 1.77) splits a slice into runs where the predicate holds
  between neighbours.
- Assigning to `current` drops the previous `String` and moves `next` in; no copy is made.

```rust
pub struct Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut current = String::from("1");
        for _ in 1..n {
            let mut next = String::with_capacity(current.len() * 2);
            for run in current.as_bytes().chunk_by(|a, b| a == b) {
                next.push_str(&run.len().to_string());
                next.push(run[0] as char);
            }
            current = next;
        }
        current
    }
}
```

Full tested solution: [`src/solutions/p0197_count_and_say.rs`](../src/solutions/p0197_count_and_say.rs)

</details>
