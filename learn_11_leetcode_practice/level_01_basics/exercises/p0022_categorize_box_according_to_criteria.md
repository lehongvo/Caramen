# Problem 0022: Categorize Box According to Criteria

- **Difficulty:** Easy
- **Topics:** integer overflow, widening `i32` to `i64`, `bool` variables, `match` on a tuple of `bool`s
- **Source:** [LeetCode 2525. Categorize Box According to Criteria](https://leetcode.com/problems/categorize-box-according-to-criteria/)

## Problem

A warehouse labels each box from its three dimensions (`length`, `width`, `height`) and its
`mass`:

- The box is **bulky** if at least one dimension is `>= 10^4`, **or** if its volume
  (`length * width * height`) is `>= 10^9`.
- The box is **heavy** if its mass is `>= 100`.

Return the label as a `String`:

- `"Both"` if the box is bulky and heavy;
- `"Bulky"` if it is bulky but not heavy;
- `"Heavy"` if it is heavy but not bulky;
- `"Neither"` otherwise.

Implement `Solution::categorize_box(length: i32, width: i32, height: i32, mass: i32) -> String`.

## Examples

```text
Input:  length = 1000, width = 35, height = 700, mass = 300
Output: "Heavy"
Explanation: no side reaches 10^4 and the volume is 24_500_000 < 10^9; the mass is >= 100.

Input:  length = 200, width = 50, height = 800, mass = 50
Output: "Neither"

Input:  length = 2000, width = 1000, height = 1000, mass = 10
Output: "Bulky"
Explanation: the volume is 2 * 10^9.
```

## Constraints

- `1 <= length, width, height <= 10^5`
- `1 <= mass <= 10^3`

## Hints

1. The volume can be as large as `10^15`, but an `i32` stops at about `2.1 * 10^9`. Which type is big enough?
2. Compute two `bool` variables, `bulky` and `heavy`, then decide the label from the pair.

## Starter code

Write your code in [`src/practice/p0022_categorize_box_according_to_criteria.rs`](../src/practice/p0022_categorize_box_according_to_criteria.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0022
```

```rust
pub struct Solution;

impl Solution {
    pub fn categorize_box(length: i32, width: i32, height: i32, mass: i32) -> String {
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
    fn example_heavy() {
        assert_eq!(Solution::categorize_box(1000, 35, 700, 300), "Heavy");
    }

    #[test]
    fn example_neither() {
        assert_eq!(Solution::categorize_box(200, 50, 800, 50), "Neither");
    }

    #[test]
    fn example_bulky_by_volume() {
        assert_eq!(Solution::categorize_box(2000, 1000, 1000, 10), "Bulky");
    }

    #[test]
    fn bulky_by_one_dimension() {
        assert_eq!(Solution::categorize_box(10_000, 1, 1, 99), "Bulky");
        assert_eq!(Solution::categorize_box(9_999, 1, 1, 99), "Neither");
    }

    #[test]
    fn both_with_maximum_values_does_not_overflow() {
        assert_eq!(
            Solution::categorize_box(100_000, 100_000, 100_000, 1000),
            "Both"
        );
    }

    #[test]
    fn volume_exactly_at_the_limit() {
        assert_eq!(Solution::categorize_box(1000, 1000, 1000, 100), "Both");
        assert_eq!(Solution::categorize_box(1000, 1000, 999, 100), "Heavy");
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** convert the dimensions to `i64` before multiplying, compute `bulky` and `heavy`, and
`match` on the tuple `(bulky, heavy)`, which has exactly four possible values.

**Complexity:** `O(1)` time, `O(1)` space.

**Rust concepts:**
- Multiplying three `i32` values of `10^5` overflows. In a debug build (and in `cargo test`)
  that **panics** with "attempt to multiply with overflow"; in a release build it silently
  wraps around. Widening first (`i64::from(length)`) avoids the problem. See
  [The Rust Book 3.2](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow).
- `i64::from(x)` is a lossless conversion; `x as i64` would also be correct here.
- `match (bulky, heavy)` with the four `bool` combinations is exhaustive, so no `_` arm is needed.
- Integer literals may contain `_` separators (`1_000_000_000`) to stay readable.

```rust
pub struct Solution;

impl Solution {
    pub fn categorize_box(length: i32, width: i32, height: i32, mass: i32) -> String {
        let volume = i64::from(length) * i64::from(width) * i64::from(height);
        let bulky =
            length >= 10_000 || width >= 10_000 || height >= 10_000 || volume >= 1_000_000_000;
        let heavy = mass >= 100;
        let label = match (bulky, heavy) {
            (true, true) => "Both",
            (true, false) => "Bulky",
            (false, true) => "Heavy",
            (false, false) => "Neither",
        };
        label.to_string()
    }
}
```

Full tested solution: [`src/solutions/p0022_categorize_box_according_to_criteria.rs`](../src/solutions/p0022_categorize_box_according_to_criteria.rs)

</details>
