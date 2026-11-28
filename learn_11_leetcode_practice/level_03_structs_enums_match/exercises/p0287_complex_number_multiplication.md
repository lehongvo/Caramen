# Problem 0287: Complex Number Multiplication

- **Difficulty:** Medium
- **Topics:** a `Complex` struct with `parse`, `times` and `text` methods, `split_once`, `strip_suffix`, `?` on `Option`
- **Source:** [LeetCode 537. Complex Number Multiplication](https://leetcode.com/problems/complex-number-multiplication/)

## Problem

A complex number is written as the string `"real+imaginaryi"`, where `real` and `imaginary` are
integers in `[-100, 100]`. Negative parts keep the `+` separator: `1 - 1i` is written
`"1+-1i"`. The imaginary unit satisfies `i² = -1`.

Given two complex numbers `num1` and `num2` in this format, return their product in the same
format.

Implement `Solution::complex_number_multiply(num1: String, num2: String) -> String`.

For this level, write a `Complex { re, im }` struct with `parse(&str) -> Option<Complex>`,
`times(self, other) -> Complex` and `text(&self) -> String`.

## Examples

```text
Input:  num1 = "1+1i", num2 = "1+1i"
Output: "0+2i"      ((1 + i)² = 1 + 2i + i² = 2i)

Input:  num1 = "1+-1i", num2 = "1+-1i"
Output: "0+-2i"     ((1 - i)² = 1 - 2i + i² = -2i)
```

## Constraints

- Both inputs are valid complex numbers in the format above.

## Hints

1. `(a + bi)(c + di) = (ac - bd) + (ad + bc)i`.
2. `s.strip_suffix('i')?.split_once('+')?` gives the two parts as `&str`s; the imaginary part may
   start with `-`, which `parse::<i32>()` handles.

## Starter code

Write your code in [`src/practice/p0287_complex_number_multiplication.rs`](../src/practice/p0287_complex_number_multiplication.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0287
```

```rust
pub struct Solution;

impl Solution {
    pub fn complex_number_multiply(num1: String, num2: String) -> String {
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

    fn mul(a: &str, b: &str) -> String {
        Solution::complex_number_multiply(a.to_string(), b.to_string())
    }

    #[test]
    fn example_one() {
        assert_eq!(mul("1+1i", "1+1i"), "0+2i");
    }

    #[test]
    fn example_two() {
        assert_eq!(mul("1+-1i", "1+-1i"), "0+-2i");
    }

    #[test]
    fn negative_real_parts() {
        assert_eq!(mul("-1+0i", "-1+0i"), "1+0i");
        assert_eq!(mul("-3+4i", "2+-1i"), "-2+11i");
    }

    #[test]
    fn zero_and_extremes() {
        assert_eq!(mul("0+0i", "100+100i"), "0+0i");
        assert_eq!(mul("100+100i", "100+100i"), "0+20000i");
        assert_eq!(mul("-100+-100i", "100+-100i"), "-20000+0i");
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** parse both numbers into `Complex`, multiply with the formula above, and format the
result as `"{re}+{im}i"`.

**Complexity:** `O(1)` time and space.

**Rust concepts:**
- A small `Copy` struct with methods turns string juggling into ordinary arithmetic.
- `split_once('+')` splits at the **first** `+`. That is the separator because a leading sign of
  the real part can only be `-` in this format.
- `?` chains the three fallible steps of `parse` (`strip_suffix`, `split_once`, `parse`).
- `format!("{}+{}i", re, im)` prints negative numbers with their `-`, which produces `"+-2i"`.

```rust
pub struct Solution;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Complex {
    re: i32,
    im: i32,
}

impl Complex {
    fn parse(s: &str) -> Option<Complex> {
        let (re, im) = s.strip_suffix('i')?.split_once('+')?;
        Some(Complex {
            re: re.parse().ok()?,
            im: im.parse().ok()?,
        })
    }

    fn times(self, other: Complex) -> Complex {
        Complex {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }

    fn text(&self) -> String {
        format!("{}+{}i", self.re, self.im)
    }
}

impl Solution {
    pub fn complex_number_multiply(num1: String, num2: String) -> String {
        match (Complex::parse(&num1), Complex::parse(&num2)) {
            (Some(a), Some(b)) => a.times(b).text(),
            // The inputs are guaranteed valid.
            _ => String::new(),
        }
    }
}
```

Full tested solution: [`src/solutions/p0287_complex_number_multiplication.rs`](../src/solutions/p0287_complex_number_multiplication.rs)

</details>
