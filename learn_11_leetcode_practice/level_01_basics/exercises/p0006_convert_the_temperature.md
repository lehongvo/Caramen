# Problem 0006: Convert the Temperature

- **Difficulty:** Easy
- **Topics:** `f64` arithmetic, float literals, `vec!`
- **Source:** [LeetCode 2469. Convert the Temperature](https://leetcode.com/problems/convert-the-temperature/)

## Problem

You are given a temperature in degrees Celsius as a floating-point number `celsius` (it has at
most two digits after the decimal point). Convert it to the two other common scales and return
them as a vector `[kelvin, fahrenheit]`, in that order:

- `kelvin = celsius + 273.15`
- `fahrenheit = celsius * 1.80 + 32.00`

An answer is accepted if each value is within `10^-5` of the exact result.

Implement `Solution::convert_temperature(celsius: f64) -> Vec<f64>`.

## Examples

```text
Input:  celsius = 36.50
Output: [309.65000, 97.70000]

Input:  celsius = 122.11
Output: [395.26000, 251.79800]
```

## Constraints

- `0 <= celsius <= 1000`

## Hints

1. Float literals need a decimal point: `32.0`, not `32` (an integer literal cannot be added to an `f64`).
2. `vec![a, b]` builds a `Vec` with two elements.

## Starter code

Write your code in [`src/practice/p0006_convert_the_temperature.rs`](../src/practice/p0006_convert_the_temperature.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0006
```

```rust
pub struct Solution;

impl Solution {
    pub fn convert_temperature(celsius: f64) -> Vec<f64> {
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

    fn assert_close(actual: Vec<f64>, expected: [f64; 2]) {
        assert_eq!(actual.len(), 2);
        for (a, e) in actual.iter().zip(expected.iter()) {
            assert!(
                (a - e).abs() < 1e-5,
                "got {actual:?}, expected {expected:?}"
            );
        }
    }

    #[test]
    fn example_body_temperature() {
        assert_close(Solution::convert_temperature(36.50), [309.65, 97.70]);
    }

    #[test]
    fn example_122_11() {
        assert_close(Solution::convert_temperature(122.11), [395.26, 251.798]);
    }

    #[test]
    fn freezing_point() {
        assert_close(Solution::convert_temperature(0.0), [273.15, 32.0]);
    }

    #[test]
    fn upper_bound() {
        assert_close(Solution::convert_temperature(1000.0), [1273.15, 1832.0]);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** apply the two formulas and put the results in a vector.

**Complexity:** `O(1)` time, `O(1)` space.

**Rust concepts:**
- `f64` is the default floating-point type (64-bit IEEE 754). See
  [The Rust Book 3.2](https://doc.rust-lang.org/book/ch03-02-data-types.html#floating-point-types).
- Rust never converts numbers implicitly: `celsius * 1.80 + 32` is a compile error because `32`
  is an integer. Use `32.0`.
- Floating-point results are approximate (`273.15` cannot be stored exactly in binary), which is
  why the problem accepts a small error and why the tests compare with a tolerance.

```rust
pub struct Solution;

impl Solution {
    pub fn convert_temperature(celsius: f64) -> Vec<f64> {
        let kelvin = celsius + 273.15;
        let fahrenheit = celsius * 1.80 + 32.00;
        vec![kelvin, fahrenheit]
    }
}
```

Full tested solution: [`src/solutions/p0006_convert_the_temperature.rs`](../src/solutions/p0006_convert_the_temperature.rs)

</details>
