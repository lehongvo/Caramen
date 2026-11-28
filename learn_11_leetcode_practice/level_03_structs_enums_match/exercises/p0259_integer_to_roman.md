# Problem 0259: Integer to Roman

- **Difficulty:** Medium
- **Topics:** a `const` table of tuples, destructuring tuples in a `for` loop, greedy subtraction, `String::push_str`
- **Source:** [LeetCode 12. Integer to Roman](https://leetcode.com/problems/integer-to-roman/)

## Problem

Convert an integer `num` between 1 and 3999 into a Roman numeral. Symbols and values are
`M = 1000`, `D = 500`, `C = 100`, `L = 50`, `X = 10`, `V = 5`, `I = 1`, and the rules are:

- write the value from the largest place to the smallest (thousands, hundreds, tens, units);
- a digit `4` or `9` uses the subtractive form: `IV` (4), `IX` (9), `XL` (40), `XC` (90),
  `CD` (400), `CM` (900);
- otherwise a digit is written with the 5-symbol (`V`, `L`, `D`) at most once, followed by up to
  three copies of the 1-symbol (`I`, `X`, `C`, `M`): `8` is `VIII`, `30` is `XXX`.

Implement `Solution::int_to_roman(num: i32) -> String`.

## Examples

```text
Input:  num = 3749
Output: "MMMDCCXLIX"    (3000 = MMM, 700 = DCC, 40 = XL, 9 = IX)

Input:  num = 58
Output: "LVIII"

Input:  num = 1994
Output: "MCMXCIV"
```

## Constraints

- `1 <= num <= 3999`

## Hints

1. Put the 13 "building blocks" in a table, largest first: `(1000, "M"), (900, "CM"), (500, "D"), ..., (1, "I")`.
2. For each block, append its symbol while the remaining number is at least its value.

## Starter code

Write your code in [`src/practice/p0259_integer_to_roman.rs`](../src/practice/p0259_integer_to_roman.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0259
```

```rust
pub struct Solution;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
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
    fn example_3749() {
        assert_eq!(Solution::int_to_roman(3749), "MMMDCCXLIX");
    }

    #[test]
    fn example_58() {
        assert_eq!(Solution::int_to_roman(58), "LVIII");
    }

    #[test]
    fn example_1994() {
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
    }

    #[test]
    fn small_numbers() {
        let expected = ["I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X"];
        for (i, want) in expected.iter().enumerate() {
            assert_eq!(Solution::int_to_roman(i as i32 + 1), *want);
        }
    }

    #[test]
    fn extremes() {
        assert_eq!(Solution::int_to_roman(3999), "MMMCMXCIX");
        assert_eq!(Solution::int_to_roman(3888), "MMMDCCCLXXXVIII");
        assert_eq!(Solution::int_to_roman(400), "CD");
        assert_eq!(Solution::int_to_roman(2024), "MMXXIV");
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** greedy. With the subtractive pairs added to the table, taking the largest block that
still fits, as many times as it fits, produces exactly the standard numeral.

**Complexity:** `O(1)`: at most 15 symbols are produced for numbers up to 3999.

**Rust concepts:**
- `const TABLE: [(i32, &str); 13] = [...]` is a compile-time table of tuples.
- `for &(value, symbol) in TABLE.iter()` destructures each tuple directly in the loop header
  (`&` copies the `i32` and the `&str` out of the reference).
- `String::with_capacity` avoids reallocations when the final size is roughly known.

```rust
pub struct Solution;

const TABLE: [(i32, &str); 13] = [
    (1000, "M"),
    (900, "CM"),
    (500, "D"),
    (400, "CD"),
    (100, "C"),
    (90, "XC"),
    (50, "L"),
    (40, "XL"),
    (10, "X"),
    (9, "IX"),
    (5, "V"),
    (4, "IV"),
    (1, "I"),
];

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut rest = num;
        let mut out = String::with_capacity(15);
        for &(value, symbol) in TABLE.iter() {
            while rest >= value {
                out.push_str(symbol);
                rest -= value;
            }
        }
        out
    }
}
```

Full tested solution: [`src/solutions/p0259_integer_to_roman.rs`](../src/solutions/p0259_integer_to_roman.rs)

</details>
