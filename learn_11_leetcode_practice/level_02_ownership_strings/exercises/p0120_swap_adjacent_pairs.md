# Problem 0120: Swap Adjacent Pairs

- **Difficulty:** Easy
- **Topics:** `&mut [String]`, `slice::swap`, `chunks_exact_mut`, moving non-`Copy` elements
- **Source:** Original · [`slice::swap`](https://doc.rust-lang.org/std/primitive.slice.html#method.swap) · [`slice::chunks_exact_mut`](https://doc.rust-lang.org/std/primitive.slice.html#method.chunks_exact_mut)

## Problem

Dancers stand in a line, stored as a mutable slice of names. When the music changes, partners
trade places: the dancers at positions 0 and 1 swap, then 2 and 3, then 4 and 5, and so on. If
the number of dancers is odd, the last one has no partner and stays where they are.

Implement `swap_pairs(line: &mut [String])`, which performs this swap in place.

With `String` elements the "temporary variable" approach does not compile:
`let tmp = line[0];` would move a `String` out of the slice (error E0507). Use a slice method
that swaps two elements for you. Do not clone.

## Examples

```text
Input:  line = ["Ann", "Ben", "Cid", "Dee"]
Output: line = ["Ben", "Ann", "Dee", "Cid"]

Input:  line = ["Ann", "Ben", "Cid"]
Output: line = ["Ben", "Ann", "Cid"]

Input:  line = ["Ann"]
Output: line = ["Ann"]
```

## Constraints

- `0 <= line.len() <= 10^4`

## Hints

1. `slice.swap(i, j)` exchanges two elements of the same slice; it works for any element type.
2. `line.chunks_exact_mut(2)` yields non-overlapping `&mut [String]` pieces of length exactly 2
   and skips a leftover single element at the end.

## Starter code

Write your code in [`src/practice/p0120_swap_adjacent_pairs.rs`](../src/practice/p0120_swap_adjacent_pairs.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0120
```

```rust
pub fn swap_pairs(line: &mut [String]) {
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

    fn owned(names: &[&str]) -> Vec<String> {
        names.iter().map(|n| n.to_string()).collect()
    }

    #[test]
    fn example_even_length() {
        let mut line = owned(&["Ann", "Ben", "Cid", "Dee"]);
        swap_pairs(&mut line);
        assert_eq!(line, owned(&["Ben", "Ann", "Dee", "Cid"]));
    }

    #[test]
    fn example_odd_length() {
        let mut line = owned(&["Ann", "Ben", "Cid"]);
        swap_pairs(&mut line);
        assert_eq!(line, owned(&["Ben", "Ann", "Cid"]));
    }

    #[test]
    fn example_single() {
        let mut line = owned(&["Ann"]);
        swap_pairs(&mut line);
        assert_eq!(line, owned(&["Ann"]));
    }

    #[test]
    fn empty_line() {
        let mut line: Vec<String> = Vec::new();
        swap_pairs(&mut line);
        assert!(line.is_empty());
    }

    #[test]
    fn swapping_twice_restores_the_order() {
        let original = owned(&["a", "b", "c", "d", "e"]);
        let mut line = original.clone();
        swap_pairs(&mut line);
        swap_pairs(&mut line);
        assert_eq!(line, original);
    }

    #[test]
    fn large_line() {
        let mut line: Vec<String> = (0..10_001).map(|i| i.to_string()).collect();
        swap_pairs(&mut line);
        assert_eq!(line[0], "1");
        assert_eq!(line[1], "0");
        assert_eq!(line[9_998], "9999");
        assert_eq!(line[10_000], "10000");
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** split the slice into mutable chunks of two and swap the two elements of every chunk.
`chunks_exact_mut` leaves out the odd last element, which is exactly the required behaviour.

**Complexity:** `O(n)` time, `O(1)` extra space. No `String` is copied: `swap` exchanges the
small (pointer, length, capacity) headers.

**Rust concepts:**
- `slice::swap(i, j)` is the safe way to exchange two elements without moving out of the slice.
- `chunks_exact_mut(n)` hands out disjoint mutable sub-slices one at a time, so the borrow checker
  is satisfied even though they all come from `line`.
- The index version, `for i in (0..line.len() - 1).step_by(2)`, would underflow when the slice is
  empty; the chunk iterator has no such edge case.

```rust
pub fn swap_pairs(line: &mut [String]) {
    for pair in line.chunks_exact_mut(2) {
        pair.swap(0, 1);
    }
}
```

Full tested solution: [`src/solutions/p0120_swap_adjacent_pairs.rs`](../src/solutions/p0120_swap_adjacent_pairs.rs)

</details>
