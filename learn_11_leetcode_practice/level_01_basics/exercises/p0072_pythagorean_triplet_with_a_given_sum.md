# Problem 0072: Pythagorean Triplet with a Given Sum

- **Difficulty:** Easy
- **Topics:** labeled blocks with `break 'label value`, nested `for` loops, choosing loop bounds, tuples
- **Source:** Original · [Rust Reference: labeled block expressions](https://doc.rust-lang.org/reference/expressions/loop-expr.html#labeled-block-expressions)

## Problem

A **Pythagorean triplet** is a set of three positive integers `a < b < c` with
`a^2 + b^2 == c^2`. Write `pythagorean_triplet(sum: u32) -> Option<(u32, u32, u32)>` that finds a
triplet with `a + b + c == sum`. If several exist, return the one with the **smallest** `a`. If
none exists, return `None`.

## Examples

```text
pythagorean_triplet(12)   -> Some((3, 4, 5))
pythagorean_triplet(30)   -> Some((5, 12, 13))
pythagorean_triplet(120)  -> Some((20, 48, 52))   (24, 45, 51) and (30, 40, 50) also work
pythagorean_triplet(1000) -> Some((200, 375, 425))
pythagorean_triplet(11)   -> None
```

## Constraints

- `0 <= sum <= 10_000` (so `c^2` fits in a `u32`)

## Hints

1. Choose `a` and `b`; then `c = sum - a - b` is forced.
2. Because `a < b < c`, `a` is below `sum / 3`, and `b` is below `(sum - a) / 2`.
   Tight bounds also guarantee that `sum - a - b` never underflows.
3. To leave two loops **with a value**, wrap them in a labeled block: `'search: { ... }`.

## Starter code

Write your code in [`src/practice/p0072_pythagorean_triplet_with_a_given_sum.rs`](../src/practice/p0072_pythagorean_triplet_with_a_given_sum.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0072
```

```rust
pub fn pythagorean_triplet(sum: u32) -> Option<(u32, u32, u32)> {
    todo!("Write your solution here")
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
    fn small_triplets() {
        assert_eq!(pythagorean_triplet(12), Some((3, 4, 5)));
        assert_eq!(pythagorean_triplet(24), Some((6, 8, 10)));
        assert_eq!(pythagorean_triplet(30), Some((5, 12, 13)));
    }

    #[test]
    fn smallest_a_wins() {
        assert_eq!(pythagorean_triplet(120), Some((20, 48, 52)));
    }

    #[test]
    fn project_euler_classic() {
        assert_eq!(pythagorean_triplet(1000), Some((200, 375, 425)));
    }

    #[test]
    fn no_triplet() {
        for sum in [0, 1, 2, 3, 11, 13, 1001] {
            assert_eq!(pythagorean_triplet(sum), None, "sum = {sum}");
        }
    }

    #[test]
    fn results_are_valid_up_to_the_limit() {
        for sum in (9_000..=10_000).step_by(7) {
            if let Some((a, b, c)) = pythagorean_triplet(sum) {
                assert!(a < b && b < c);
                assert_eq!(a + b + c, sum);
                assert_eq!(a * a + b * b, c * c);
            }
        }
        assert_eq!(pythagorean_triplet(10_000), Some((2000, 3750, 4250)));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** for each `a` in increasing order and each valid `b`, compute `c` and test the
Pythagorean equation. The first hit has the smallest `a`. The bounds `a <= sum / 3` and
`2 * b <= sum - a - 1` guarantee `a < b < c` and keep every subtraction non-negative.

**Rust concepts:**
- A **labeled block** `'search: { ... }` is an expression. `break 'search value;` leaves the
  block (from any depth of nested loops) and makes `value` the block's result. If the block
  finishes normally, its final expression (`None`) is the result.
- With unsigned integers, write bounds so that no subtraction can go below zero: `sum - a - 1`
  is safe because `a <= sum / 3 < sum` whenever the loop body runs.

```rust
pub fn pythagorean_triplet(sum: u32) -> Option<(u32, u32, u32)> {
    'search: {
        for a in 1..=sum / 3 {
            for b in a + 1..=(sum - a - 1) / 2 {
                let c = sum - a - b;
                if a * a + b * b == c * c {
                    break 'search Some((a, b, c));
                }
            }
        }
        None
    }
}
```

Full tested solution: [`src/solutions/p0072_pythagorean_triplet_with_a_given_sum.rs`](../src/solutions/p0072_pythagorean_triplet_with_a_given_sum.rs)

</details>
