# Problem 0070: Count Square Sum Triples

- **Difficulty:** Easy
- **Topics:** nested `for` loops, float square root with `as` casts, verifying a float result with integers
- **Source:** [LeetCode 1925. Count Square Sum Triples](https://leetcode.com/problems/count-square-sum-triples/)

## Problem

A **square triple** is a triple of integers `(a, b, c)` with `a^2 + b^2 == c^2`. Given `n`,
count the square triples with `1 <= a, b, c <= n`. The order matters: `(3, 4, 5)` and `(4, 3, 5)`
are two different triples.

Implement `Solution::count_triples(n: i32) -> i32`.

## Examples

```text
Input:  n = 5
Output: 2
Explanation: (3, 4, 5) and (4, 3, 5).

Input:  n = 10
Output: 4
Explanation: (3, 4, 5), (4, 3, 5), (6, 8, 10) and (8, 6, 10).
```

## Constraints

- `1 <= n <= 250`

## Hints

1. Loop over `a` and `b`; `c` is then determined by `c^2 = a^2 + b^2`.
2. `(x as f64).sqrt() as i32` gives a candidate `c`. Floats can be slightly off, so confirm with
   the integer test `c * c == a * a + b * b`.

## Starter code

Write your code in [`src/practice/p0070_count_square_sum_triples.rs`](../src/practice/p0070_count_square_sum_triples.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0070
```

```rust
pub struct Solution;

impl Solution {
    pub fn count_triples(n: i32) -> i32 {
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
    fn example_five() {
        assert_eq!(Solution::count_triples(5), 2);
    }

    #[test]
    fn example_ten() {
        assert_eq!(Solution::count_triples(10), 4);
    }

    #[test]
    fn too_small_for_any_triple() {
        assert_eq!(Solution::count_triples(1), 0);
        assert_eq!(Solution::count_triples(4), 0);
    }

    #[test]
    fn matches_triple_loop() {
        for n in [13, 17, 25, 50] {
            let mut expected = 0;
            for a in 1..=n {
                for b in 1..=n {
                    for c in 1..=n {
                        if a * a + b * b == c * c {
                            expected += 1;
                        }
                    }
                }
            }
            assert_eq!(Solution::count_triples(n), expected, "n = {n}");
        }
    }

    #[test]
    fn largest_input() {
        assert_eq!(Solution::count_triples(250), 330);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** for each pair `(a, b)`, compute `s = a^2 + b^2`, take the rounded square root `c`, and
count the pair if `c * c == s` and `c <= n`.

**Complexity:** `O(n^2)` time, `O(1)` space.

**Rust concepts:**
- `as f64` converts an integer to a float exactly (for values below `2^53`), and `f64::sqrt`
  returns an `f64`.
- `.round() as i32` converts back; rounding first protects against a result like `4.9999999`
  being truncated to 4.
- Using the float only as a **guess** and checking with exact integer arithmetic is a robust
  pattern. (`i32::isqrt` also exists, but only since Rust 1.84.)

```rust
pub struct Solution;

impl Solution {
    pub fn count_triples(n: i32) -> i32 {
        let mut count = 0;
        for a in 1..=n {
            for b in 1..=n {
                let s = a * a + b * b;
                let c = (s as f64).sqrt().round() as i32;
                if c <= n && c * c == s {
                    count += 1;
                }
            }
        }
        count
    }
}
```

Full tested solution: [`src/solutions/p0070_count_square_sum_triples.rs`](../src/solutions/p0070_count_square_sum_triples.rs)

</details>
