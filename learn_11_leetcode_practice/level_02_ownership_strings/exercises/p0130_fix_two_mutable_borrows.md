# Problem 0130: Fix: Two Mutable Borrows of One Vec

- **Difficulty:** Easy
- **Topics:** fix the borrow checker, error E0499, `slice::swap`, `split_first_mut`, `split_last_mut`
- **Source:** Original · [E0499](https://doc.rust-lang.org/error_codes/E0499.html) · [The Rust Book 4.2](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

## Problem

This function tries to exchange the first and the last word of a list, but it does not compile:

```text
pub fn swap_ends(words: &mut Vec<String>) {
    let first = &mut words[0];
    let last = &mut words[words.len() - 1];
    std::mem::swap(first, last);
}

error[E0499]: cannot borrow `*words` as mutable more than once at a time
  |     let first = &mut words[0];
  |                      ----- first mutable borrow occurs here
  |     let last = &mut words[words.len() - 1];
  |                     ^^^^^ second mutable borrow occurs here
  |     std::mem::swap(first, last);
  |                    ----- first borrow later used here
  = help: use `.split_at_mut(position)` to obtain two mutable non-overlapping sub-slices
```

The compiler cannot see that index `0` and index `len - 1` are different elements, so it refuses
two simultaneous `&mut` borrows of the same vector. (It would also panic on an empty vector.)

Implement the corrected `swap_ends(words: &mut [String]) -> bool`. If the slice has at least two
elements, exchange the first and the last one and return `true`. Otherwise leave it unchanged and
return `false`. Do not clone any `String`.

## Examples

```text
Input:  words = ["alpha", "beta", "gamma"]
Output: true, words = ["gamma", "beta", "alpha"]

Input:  words = ["x", "y"]
Output: true, words = ["y", "x"]

Input:  words = ["lonely"]
Output: false, words = ["lonely"]
```

## Constraints

- `0 <= words.len() <= 10^4`

## Hints

1. `slice.swap(i, j)` takes **indices**, so only one mutable borrow (of the slice) is needed.
2. If you really need two `&mut` at once, `split_first_mut()` returns the first element and the
   rest as separate, non-overlapping mutable borrows.

## Starter code

Write your code in [`src/practice/p0130_fix_two_mutable_borrows.rs`](../src/practice/p0130_fix_two_mutable_borrows.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0130
```

```rust
pub fn swap_ends(words: &mut [String]) -> bool {
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

    fn owned(words: &[&str]) -> Vec<String> {
        words.iter().map(|w| w.to_string()).collect()
    }

    #[test]
    fn example_three_words() {
        let mut words = owned(&["alpha", "beta", "gamma"]);
        assert!(swap_ends(&mut words));
        assert_eq!(words, owned(&["gamma", "beta", "alpha"]));
    }

    #[test]
    fn example_two_words() {
        let mut words = owned(&["x", "y"]);
        assert!(swap_ends(&mut words));
        assert_eq!(words, owned(&["y", "x"]));
    }

    #[test]
    fn example_single_word() {
        let mut words = owned(&["lonely"]);
        assert!(!swap_ends(&mut words));
        assert_eq!(words, owned(&["lonely"]));
    }

    #[test]
    fn empty_slice() {
        let mut words: Vec<String> = Vec::new();
        assert!(!swap_ends(&mut words));
    }

    #[test]
    fn large_list() {
        let mut words: Vec<String> = (0..10_000).map(|i| i.to_string()).collect();
        assert!(swap_ends(&mut words));
        assert_eq!(words[0], "9999");
        assert_eq!(words[9_999], "0");
        assert_eq!(words[5_000], "5000");
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** let the slice do the swap. `words.swap(0, len - 1)` borrows `words` mutably once and
exchanges the two elements internally. The length check both avoids the panic and answers the
`bool` question.

**Complexity:** `O(1)` time and extra space.

**Rust concepts:**
- At most **one** `&mut` to a value may be alive at a time; indexing `words[i]` borrows the whole
  vector, whatever the index.
- APIs such as `swap`, `split_at_mut`, `split_first_mut` and `chunks_mut` hand out disjoint
  mutable access in a way the compiler can verify (they use `unsafe` internally, correctly).
- The two-borrow version written safely:
  `if let Some((first, rest)) = words.split_first_mut() { if let Some(last) = rest.last_mut() {
  std::mem::swap(first, last); } }`.

```rust
pub fn swap_ends(words: &mut [String]) -> bool {
    let len = words.len();
    if len < 2 {
        return false;
    }
    words.swap(0, len - 1);
    true
}
```

Full tested solution: [`src/solutions/p0130_fix_two_mutable_borrows.rs`](../src/solutions/p0130_fix_two_mutable_borrows.rs)

</details>
