# Problem 0288: Fraction Addition and Subtraction

- **Difficulty:** Medium
- **Topics:** a `Fraction` struct that keeps itself normalised, a constructor enforcing invariants, scanning signed numbers, `fold`
- **Source:** [LeetCode 592. Fraction Addition and Subtraction](https://leetcode.com/problems/fraction-addition-and-subtraction/)

## Problem

You are given a string `expression` that adds and subtracts fractions, such as `"-1/2+1/3"`.
Compute the result and return it as an **irreducible** fraction `"numerator/denominator"`:

- the denominator is always positive;
- a whole number `n` is written `"n/1"` (so zero is `"0/1"`);
- a negative result has the sign on the numerator (`"-1/6"`).

Every fraction in the input has the form `±a/b` (the first one may have no sign), with
`1 <= a, b <= 10`, and there are at most 10 fractions.

Implement `Solution::fraction_addition(expression: String) -> String`.

For this level, write a `Fraction { num, den }` struct whose constructor always reduces it and
keeps `den > 0`, and a `plus` method.

## Examples

```text
Input:  expression = "-1/2+1/2"
Output: "0/1"

Input:  expression = "-1/2+1/2+1/3"
Output: "1/3"

Input:  expression = "1/3-1/2"
Output: "-1/6"
```

## Constraints

- `expression` contains only digits, `/`, `+` and `-`; it is valid.
- Numerators and denominators stay within `i32` during the computation.

## Hints

1. `a/b + c/d = (a*d + c*b) / (b*d)`; reduce by `gcd(|num|, den)` after every addition.
2. To read the fractions, start a new piece at every `+` or `-` that is not at position 0:
   `"1/3-1/2"` becomes `"1/3"` and `"-1/2"`.

## Starter code

Write your code in [`src/practice/p0288_fraction_addition_and_subtraction.rs`](../src/practice/p0288_fraction_addition_and_subtraction.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0288
```

```rust
pub struct Solution;

impl Solution {
    pub fn fraction_addition(expression: String) -> String {
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

    fn add(e: &str) -> String {
        Solution::fraction_addition(e.to_string())
    }

    #[test]
    fn example_zero() {
        assert_eq!(add("-1/2+1/2"), "0/1");
    }

    #[test]
    fn example_one_third() {
        assert_eq!(add("-1/2+1/2+1/3"), "1/3");
    }

    #[test]
    fn example_negative() {
        assert_eq!(add("1/3-1/2"), "-1/6");
    }

    #[test]
    fn whole_numbers_and_single_fractions() {
        assert_eq!(add("5/3+1/3"), "2/1");
        assert_eq!(add("2/4"), "1/2");
        assert_eq!(add("-10/10"), "-1/1");
        assert_eq!(add("+3/9"), "1/3");
    }

    #[test]
    fn ten_fractions() {
        // 1/1 + 1/2 + ... + 1/10 = 7381/2520
        let e = "1/1+1/2+1/3+1/4+1/5+1/6+1/7+1/8+1/9+1/10";
        assert_eq!(add(e), "7381/2520");
        let neg = "-1/1-1/2-1/3-1/4-1/5-1/6-1/7-1/8-1/9-1/10";
        assert_eq!(add(neg), "-7381/2520");
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** split the expression into signed fractions, parse each one into a `Fraction`, and fold
them with `plus`, starting from `0/1`. `Fraction::new` reduces with the greatest common divisor,
so the final value is already in the required form.

**Complexity:** `O(n)` time for an input of length `n`; `O(n)` space for the pieces.

**Rust concepts:**
- The only way to create a `Fraction` is `Fraction::new`, which normalises sign and common
  factors. Every other method can rely on that invariant, including `text`.
- `str::match_indices` / `char_indices` find the split points; slicing `&expr[start..i]` borrows
  each piece without copying.
- Euclid's algorithm `gcd(a, b) = gcd(b, a % b)` is a three-line recursive function.

```rust
pub struct Solution;

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a } else { gcd(b, a % b) }
}

/// A fraction in lowest terms with a positive denominator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Fraction {
    num: i32,
    den: i32,
}

impl Fraction {
    /// Builds a normalised fraction; `den` must not be zero.
    fn new(num: i32, den: i32) -> Fraction {
        let sign = if den < 0 { -1 } else { 1 };
        let g = gcd(num.abs(), den.abs()).max(1);
        Fraction {
            num: sign * num / g,
            den: sign * den / g,
        }
    }

    fn parse(s: &str) -> Option<Fraction> {
        let (num, den) = s.split_once('/')?;
        let den: i32 = den.parse().ok()?;
        if den == 0 {
            return None;
        }
        Some(Fraction::new(num.parse().ok()?, den))
    }

    fn plus(self, other: Fraction) -> Fraction {
        Fraction::new(
            self.num * other.den + other.num * self.den,
            self.den * other.den,
        )
    }

    fn text(&self) -> String {
        format!("{}/{}", self.num, self.den)
    }
}

/// Splits `"1/3-1/2"` into `["1/3", "-1/2"]`.
fn pieces(expr: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut start = 0;
    for (i, c) in expr.char_indices() {
        if i > 0 && matches!(c, '+' | '-') {
            out.push(&expr[start..i]);
            start = i;
        }
    }
    if start < expr.len() {
        out.push(&expr[start..]);
    }
    out
}

impl Solution {
    pub fn fraction_addition(expression: String) -> String {
        pieces(&expression)
            .into_iter()
            .filter_map(Fraction::parse)
            .fold(Fraction::new(0, 1), Fraction::plus)
            .text()
    }
}
```

Full tested solution: [`src/solutions/p0288_fraction_addition_and_subtraction.rs`](../src/solutions/p0288_fraction_addition_and_subtraction.rs)

</details>
