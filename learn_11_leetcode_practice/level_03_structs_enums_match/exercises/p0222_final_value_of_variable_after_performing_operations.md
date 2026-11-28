# Problem 0222: Final Value of Variable After Performing Operations

- **Difficulty:** Easy
- **Topics:** parsing strings into an `enum`, `match` on `&str` with `|` alternatives, `fold`
- **Source:** [LeetCode 2011. Final Value of Variable After Performing Operations](https://leetcode.com/problems/final-value-of-variable-after-performing-operations/)

## Problem

A toy language has one variable `X`, which starts at `0`, and four statements:
`"++X"` and `"X++"` add one to `X`; `"--X"` and `"X--"` subtract one from `X`.

You are given the program as a list of statements `operations`. Return the value of `X` after
running all of them in order.

Implement `Solution::final_value_after_operations(operations: Vec<String>) -> i32`.

For this level, first convert each statement into an `enum Op { Increment, Decrement }`, then
evaluate the list of `Op`s.

## Examples

```text
Input:  operations = ["--X", "X++", "X++"]
Output: 1          (0 - 1 + 1 + 1)

Input:  operations = ["++X", "++X", "X++"]
Output: 3

Input:  operations = ["X++", "++X", "--X", "X--"]
Output: 0
```

## Constraints

- `1 <= operations.len() <= 100`
- Each statement is exactly one of `"++X"`, `"X++"`, `"--X"`, `"X--"`.

## Hints

1. A `match` on a `&str` can list several literals in one arm: `"++X" | "X++" => ...`.
2. `operations` holds `String`s; `s.as_str()` gives the `&str` to match on.

## Starter code

Write your code in [`src/practice/p0222_final_value_of_variable_after_performing_operations.rs`](../src/practice/p0222_final_value_of_variable_after_performing_operations.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0222
```

```rust
pub struct Solution;

impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
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

    fn ops(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::final_value_after_operations(ops(&["--X", "X++", "X++"])),
            1
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::final_value_after_operations(ops(&["++X", "++X", "X++"])),
            3
        );
    }

    #[test]
    fn example_three_cancels_out() {
        assert_eq!(
            Solution::final_value_after_operations(ops(&["X++", "++X", "--X", "X--"])),
            0
        );
    }

    #[test]
    fn single_decrement() {
        assert_eq!(Solution::final_value_after_operations(ops(&["X--"])), -1);
    }

    #[test]
    fn hundred_decrements() {
        let program = vec!["--X".to_string(); 100];
        assert_eq!(Solution::final_value_after_operations(program), -100);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** parse each statement into `Op` (returning `None` for anything unknown), then fold
the ops into the final value: `Increment` adds 1 and `Decrement` subtracts 1.

**Complexity:** `O(n)` time, `O(1)` space.

**Rust concepts:**
- String literals are valid patterns, and `|` joins alternatives inside one arm.
- Matching on `s.as_str()` is necessary because a `String` cannot be compared with a literal
  pattern directly.
- An `Option`-returning parser (`Op::parse`) plus `filter_map` is a common way to turn text into
  typed values.

```rust
pub struct Solution;

/// One statement of the toy language.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Op {
    Increment,
    Decrement,
}

impl Op {
    fn parse(s: &str) -> Option<Op> {
        match s {
            "++X" | "X++" => Some(Op::Increment),
            "--X" | "X--" => Some(Op::Decrement),
            _ => None,
        }
    }
}

impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        operations
            .iter()
            .filter_map(|s| Op::parse(s.as_str()))
            .fold(0, |x, op| match op {
                Op::Increment => x + 1,
                Op::Decrement => x - 1,
            })
    }
}
```

Full tested solution: [`src/solutions/p0222_final_value_of_variable_after_performing_operations.rs`](../src/solutions/p0222_final_value_of_variable_after_performing_operations.rs)

</details>
