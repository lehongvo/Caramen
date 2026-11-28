# Problem 0774: Minimum Flips to Make a OR b Equal to c

- **Difficulty:** Medium
- **Topics:** per-bit case analysis, extracting a bit with `(x >> i) & 1`, `match` on a bit
- **Source:** [LeetCode 1318. Minimum Flips to Make a OR b Equal to c](https://leetcode.com/problems/minimum-flips-to-make-a-or-b-equal-to-c/)

## Problem

You are given three positive integers `a`, `b` and `c`. A **flip** changes one bit of `a` or
of `b` (from `0` to `1` or from `1` to `0`). Return the minimum number of flips needed so that
`a | b == c` (bitwise OR).

Implement `Solution::min_flips(a: i32, b: i32, c: i32) -> i32`.

## Examples

```text
Input:  a = 2, b = 6, c = 5
Output: 3
        a = 010, b = 110, c = 101: flip bit 1 of a, bit 1 of b, bit 0 of a or b
        (bit 2 already works because b has it).

Input:  a = 4, b = 2, c = 7
Output: 1             (set bit 0 of either a or b)

Input:  a = 1, b = 2, c = 3
Output: 0
```

## Constraints

- `1 <= a, b, c <= 10^9`

## Hints

1. The bits are independent: solve the problem for one bit position at a time and add up.
2. If the bit of `c` is `1`, how many flips are needed when `a` and `b` both have `0` there? If it
   is `0`, what about `a` and `b` both having `1`?

## Starter code

Write your code in [`src/practice/p0774_minimum_flips_to_make_a_or_b_equal_to_c.rs`](../src/practice/p0774_minimum_flips_to_make_a_or_b_equal_to_c.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0774
```

```rust
pub struct Solution;

impl Solution {
    pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
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
    fn example_three_flips() {
        assert_eq!(Solution::min_flips(2, 6, 5), 3);
    }

    #[test]
    fn example_one_flip() {
        assert_eq!(Solution::min_flips(4, 2, 7), 1);
    }

    #[test]
    fn example_already_equal() {
        assert_eq!(Solution::min_flips(1, 2, 3), 0);
    }

    #[test]
    fn both_bits_set_where_c_has_zero() {
        // Bit 0: a = 1, b = 1, c = 0 needs two flips; bit 1: c = 1 but a = b = 0 needs one.
        assert_eq!(Solution::min_flips(1, 1, 2), 3);
    }

    #[test]
    fn matches_the_bit_formula() {
        let cases: [(i32, i32, i32); 4] = [
            (8, 3, 5),
            (1_000_000_000, 999_999_999, 1),
            (7, 7, 7),
            (123, 456, 789),
        ];
        for (a, b, c) in cases {
            let formula = ((a | b) ^ c).count_ones() + (a & b & !c).count_ones();
            assert_eq!(Solution::min_flips(a, b, c), formula as i32, "{a} {b} {c}");
        }
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** per-bit case analysis: OR works bit by bit, so each position is an independent
sub-problem with bits `x` (of `a`), `y` (of `b`) and `z` (of `c`):
- `z == 1`: at least one of `x`, `y` must be `1`; one flip is needed only when both are `0`,
  i.e. `1 - (x | y)` flips;
- `z == 0`: both must be `0`, so every `1` among `x` and `y` must be flipped: `x + y` flips.

The values are below `2^30`, so 31 bit positions cover everything. (The same count as one
formula: `((a | b) ^ c).count_ones() + (a & b & !c).count_ones()`.)

**Complexity:** `O(1)` time (31 positions), `O(1)` space.

**Rust concepts:**
- `(a >> i) & 1` extracts bit `i` as `0` or `1`.
- `(0..31).map(...).sum()` adds the per-bit costs; the `match` on the bit of `c` mirrors the
  two cases of the analysis.

```rust
pub struct Solution;

impl Solution {
    pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
        (0..31)
            .map(|i| {
                let (x, y, z) = ((a >> i) & 1, (b >> i) & 1, (c >> i) & 1);
                match z {
                    1 => 1 - (x | y), // need at least one 1
                    _ => x + y,       // need both 0
                }
            })
            .sum()
    }
}
```

Full tested solution: [`src/solutions/p0774_minimum_flips_to_make_a_or_b_equal_to_c.rs`](../src/solutions/p0774_minimum_flips_to_make_a_or_b_equal_to_c.rs)

</details>
