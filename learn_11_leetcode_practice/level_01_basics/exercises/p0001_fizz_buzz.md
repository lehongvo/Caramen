# Problem 0001: Fizz Buzz

- **Difficulty:** Easy
- **Topics:** `for` loops over ranges, `match` on tuples, `String`, `Vec`
- **Source:** [LeetCode 412. Fizz Buzz](https://leetcode.com/problems/fizz-buzz/)

## Problem

You are given a positive integer `n`. Build and return a `Vec<String>` with exactly `n`
elements. The element for the number `i` (where `i` goes from `1` to `n`, inclusive) is:

- `"FizzBuzz"` if `i` is divisible by **both** 3 and 5;
- `"Fizz"` if `i` is divisible by 3 but not by 5;
- `"Buzz"` if `i` is divisible by 5 but not by 3;
- the decimal representation of `i` (for example `"7"`) otherwise.

Implement `Solution::fizz_buzz(n: i32) -> Vec<String>`.

## Examples

```text
Input:  n = 3
Output: ["1", "2", "Fizz"]

Input:  n = 5
Output: ["1", "2", "Fizz", "4", "Buzz"]

Input:  n = 15
Output: ["1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz",
         "11", "Fizz", "13", "14", "FizzBuzz"]
```

## Constraints

- `1 <= n <= 10^4`

## Hints

1. Iterate `i` over the range `1..=n` (the `=` makes the range include `n`).
2. Handle "divisible by both 3 and 5" **first**; otherwise 15 would fall into the `"Fizz"` branch.
3. Turn a number into a string with `i.to_string()`, and a string literal into a `String` with `String::from("Fizz")`.

## Starter code

Write your code in [`src/practice/p0001_fizz_buzz.rs`](../src/practice/p0001_fizz_buzz.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0001
```

```rust
pub struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
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
    fn example_n_3() {
        assert_eq!(Solution::fizz_buzz(3), vec!["1", "2", "Fizz"]);
    }

    #[test]
    fn example_n_5() {
        assert_eq!(Solution::fizz_buzz(5), vec!["1", "2", "Fizz", "4", "Buzz"]);
    }

    #[test]
    fn example_n_15() {
        let result = Solution::fizz_buzz(15);
        assert_eq!(result.len(), 15);
        assert_eq!(result[2], "Fizz");
        assert_eq!(result[9], "Buzz");
        assert_eq!(result[13], "14");
        assert_eq!(result[14], "FizzBuzz");
    }

    #[test]
    fn smallest_input() {
        assert_eq!(Solution::fizz_buzz(1), vec!["1"]);
    }

    #[test]
    fn largest_input_counts() {
        let result = Solution::fizz_buzz(10_000);
        assert_eq!(result.len(), 10_000);
        // 1..=10000 contains 10000 / 15 = 666 multiples of 15.
        assert_eq!(result.iter().filter(|s| *s == "FizzBuzz").count(), 666);
        assert_eq!(result[9_999], "Buzz");
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** for each `i`, look at the pair of remainders `(i % 3, i % 5)`. A `match` on that
tuple lists the four cases directly: `(0, 0)` is divisible by both, `(0, _)` only by 3,
`(_, 0)` only by 5, and `_` covers everything else. Match arms are tried from top to bottom,
so `(0, 0)` must come first.

**Complexity:** `O(n)` time; `O(n)` space for the output vector.

**Rust concepts:**
- `1..=n` is an inclusive range; `1..n` would stop before `n`.
- `.map(...).collect()` turns an iterator into a `Vec<String>`; the compiler infers the target
  collection from the function's return type.
- The `_` pattern inside a tuple means "any value", which makes the `match` exhaustive.

```rust
pub struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        (1..=n)
            .map(|i| match (i % 3, i % 5) {
                (0, 0) => String::from("FizzBuzz"),
                (0, _) => String::from("Fizz"),
                (_, 0) => String::from("Buzz"),
                _ => i.to_string(),
            })
            .collect()
    }
}
```

Full tested solution: [`src/solutions/p0001_fizz_buzz.rs`](../src/solutions/p0001_fizz_buzz.rs)

</details>
