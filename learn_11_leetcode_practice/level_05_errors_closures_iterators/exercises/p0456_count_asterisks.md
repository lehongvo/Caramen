# Problem 0456: Count Asterisks

- **Difficulty:** Easy
- **Topics:** `str::split`, `Iterator::step_by`, `str::matches`, `map` + `sum`
- **Source:** [LeetCode 2315. Count Asterisks](https://leetcode.com/problems/count-asterisks/)

## Problem

The string `s` contains bars `'|'` that come in pairs: the 1st and 2nd bar form a pair, the 3rd
and 4th form the next pair, and so on. Count the asterisks `'*'` in `s` that are **not** between
the two bars of a pair.

Implement `Solution::count_asterisks(s: String) -> i32`.

## Examples

```text
Input:  s = "l|*e*et|c**o|*de|"
Output: 2
Explanation: "*e*et" and "*de" are inside pairs; only the two asterisks of "c**o" count.

Input:  s = "iamprogrammer"
Output: 0

Input:  s = "yo|uar|e**|b|e***au|tifu|l"
Output: 5
Explanation: the parts outside pairs are "yo", "e**", "e***au" and "l".
```

## Constraints

- `1 <= s.len() <= 1000`
- `s` contains lowercase English letters, `'|'` and `'*'`.
- `s` contains an **even** number of `'|'`.

## Hints

1. Splitting on `'|'` gives the parts between bars. Which parts are outside the pairs: the ones
   with an even index or an odd index?
2. `step_by(2)` keeps every second item of an iterator, starting with the first.

## Starter code

Write your code in [`src/practice/p0456_count_asterisks.rs`](../src/practice/p0456_count_asterisks.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0456
```

```rust
pub struct Solution;

impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
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

    fn run(s: &str) -> i32 {
        Solution::count_asterisks(s.to_string())
    }

    #[test]
    fn example_mixed() {
        assert_eq!(run("l|*e*et|c**o|*de|"), 2);
    }

    #[test]
    fn example_no_bars() {
        assert_eq!(run("iamprogrammer"), 0);
    }

    #[test]
    fn example_many_pairs() {
        assert_eq!(run("yo|uar|e**|b|e***au|tifu|l"), 5);
    }

    #[test]
    fn edge_cases() {
        assert_eq!(run("*"), 1);
        assert_eq!(run("|*|"), 0);
        assert_eq!(run("||**||"), 2);
        assert_eq!(run(&"*".repeat(1000)), 1000);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** `s.split('|')` alternates between "outside a pair" (index 0, 2, 4, ...) and "inside a
pair" (index 1, 3, ...). Keep the even-indexed parts with `step_by(2)` and count their asterisks.

**Complexity:** `O(n)` time, `O(1)` extra space.

**Rust concepts:**
- `step_by(k)` yields the first item and then every `k`-th item; it panics if `k == 0`.
- `str::matches('*')` iterates over the occurrences of a pattern; `count()` counts them.
- `sum::<usize>()` adds the counts; the final `as i32` is safe because `s.len() <= 1000`.

```rust
pub struct Solution;

impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        s.split('|')
            .step_by(2)
            .map(|part| part.matches('*').count())
            .sum::<usize>() as i32
    }
}
```

Full tested solution: [`src/solutions/p0456_count_asterisks.rs`](../src/solutions/p0456_count_asterisks.rs)

</details>
