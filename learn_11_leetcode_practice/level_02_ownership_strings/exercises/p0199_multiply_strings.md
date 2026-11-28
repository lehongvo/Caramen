# Problem 0199: Multiply Strings

- **Difficulty:** Medium
- **Topics:** digit vectors, schoolbook multiplication, `enumerate().rev()`, `char::from_digit`
- **Source:** [LeetCode 43. Multiply Strings](https://leetcode.com/problems/multiply-strings/)

## Problem

Given two non-negative integers `num1` and `num2` as decimal strings, return their product, also
as a decimal string. The numbers can have up to 200 digits, so you cannot convert them to integer
types, and you may not use a big-integer library.

Implement `Solution::multiply(num1: String, num2: String) -> String`.

## Examples

```text
Input:  num1 = "2", num2 = "3"
Output: "6"

Input:  num1 = "123", num2 = "456"
Output: "56088"

Input:  num1 = "0", num2 = "52"
Output: "0"
```

## Constraints

- `1 <= num1.len(), num2.len() <= 200`
- Both contain only digits and have no leading zeros except the number `"0"`.

## Hints

1. The product of an `n`-digit and an `m`-digit number has at most `n + m` digits: allocate
   `vec![0u32; n + m]`.
2. Digit `i` of `num1` times digit `j` of `num2` (both counted from the left) contributes to
   position `i + j + 1` of the result, with its carry going to position `i + j`.
3. Skip the leading zeros at the end, but keep at least one digit.

## Starter code

Write your code in [`src/practice/p0199_multiply_strings.rs`](../src/practice/p0199_multiply_strings.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0199
```

```rust
pub struct Solution;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
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
        Solution::multiply(a.to_string(), b.to_string())
    }

    #[test]
    fn example_single_digits() {
        assert_eq!(mul("2", "3"), "6");
    }

    #[test]
    fn example_three_digits() {
        assert_eq!(mul("123", "456"), "56088");
    }

    #[test]
    fn example_zero() {
        assert_eq!(mul("0", "52"), "0");
        assert_eq!(mul("52", "0"), "0");
    }

    #[test]
    fn carries_everywhere() {
        assert_eq!(mul("99", "99"), "9801");
        assert_eq!(mul("999", "1"), "999");
    }

    #[test]
    fn matches_u128_multiplication() {
        let pairs = [
            (123_456_789u128, 987_654_321u128),
            (u64::MAX as u128, u64::MAX as u128),
        ];
        for (x, y) in pairs {
            assert_eq!(mul(&x.to_string(), &y.to_string()), (x * y).to_string());
        }
    }

    #[test]
    fn longest_inputs() {
        // (10^200 - 1)^2 = 10^400 - 2 * 10^200 + 1 = 99...9800...01
        let nines = "9".repeat(200);
        let expected = format!("{}8{}1", "9".repeat(199), "0".repeat(199));
        assert_eq!(mul(&nines, &nines), expected);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** long multiplication. Process digit pairs from the least significant ones. For the pair
`(i, j)`, add the product to slot `i + j + 1`, keep the last digit there and move the carry to
slot `i + j`. Because the pairs are visited from right to left, every slot is normalized again
after its last carry, so all slots end up being single digits. Finally drop the leading zeros
and convert the digits to characters.

**Complexity:** `O(n * m)` time, `O(n + m)` space.

**Rust concepts:**
- `as_bytes()` + `b - b'0'` reads digits without allocating; `u32::from(b - b'0')` widens them.
- `iter().enumerate().rev()` walks a slice from the back while keeping the original indices.
- `char::from_digit(d, 10)` turns a digit value into a `char` and returns `None` for `d >= 10`,
  which cannot happen here.
- The early return for `"0"` avoids producing results such as `"000"`.

```rust
pub struct Solution;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == "0" || num2 == "0" {
            return String::from("0");
        }
        let (a, b) = (num1.as_bytes(), num2.as_bytes());
        let mut digits = vec![0u32; a.len() + b.len()];
        for (i, &x) in a.iter().enumerate().rev() {
            for (j, &y) in b.iter().enumerate().rev() {
                let sum = digits[i + j + 1] + u32::from(x - b'0') * u32::from(y - b'0');
                digits[i + j + 1] = sum % 10;
                digits[i + j] += sum / 10;
            }
        }
        // Both inputs are non-zero, so a non-zero digit exists.
        let first = digits.iter().position(|&d| d != 0).unwrap_or(0);
        digits[first..]
            .iter()
            .map(|&d| char::from_digit(d, 10).expect("every slot holds a single digit"))
            .collect()
    }
}
```

Full tested solution: [`src/solutions/p0199_multiply_strings.rs`](../src/solutions/p0199_multiply_strings.rs)

</details>
