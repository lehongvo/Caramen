# Problem 0932: A Safe `split_at_mut`

- **Difficulty:** Hard
- **Topics:** `std::slice::from_raw_parts_mut`, building safe abstractions over `unsafe`, disjoint `&mut` borrows, lifetimes of returned borrows
- **Source:** Original · [The Rust Book 20.1: Creating a Safe Abstraction over Unsafe Code](https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html#creating-a-safe-abstraction-over-unsafe-code) · [std: `slice::from_raw_parts_mut`](https://doc.rust-lang.org/std/slice/fn.from_raw_parts_mut.html)

## Problem

The borrow checker cannot see that `&mut v[..mid]` and `&mut v[mid..]` do not overlap, so it
rejects taking both at once. The standard library's `split_at_mut` solves this with a little
`unsafe` code behind a safe signature. Write your own versions (do **not** call the std methods
`split_at_mut`, `split_first_mut`, `split_last_mut`, `chunks_mut` or `get_disjoint_mut`):

1. `split_at_mut<T>(slice: &mut [T], mid: usize) -> (&mut [T], &mut [T])`: the elements before
   `mid` and from `mid` on. Panics with `"mid out of bounds"` if `mid > len`.
2. `get_two_mut<T>(slice: &mut [T], i: usize, j: usize) -> Option<(&mut T, &mut T)>`: mutable
   references to two **different** elements; `None` if `i == j` or an index is out of bounds.
3. `first_rest_last_mut<T>(slice: &mut [T]) -> Option<(&mut T, &mut [T], &mut T)>`: the first
   element, the middle part and the last element; `None` if the slice has fewer than 2 elements.
4. `chunks_exact_mut<T>(slice: &mut [T], size: usize) -> Vec<&mut [T]>`: the consecutive,
   non-overlapping chunks of exactly `size` elements (a shorter remainder is left out). Panics
   with `"chunk size must be positive"` if `size == 0`.

## Examples

```text
let mut v = [1, 2, 3, 4, 5];
let (left, right) = split_at_mut(&mut v, 2);   // left = [1, 2], right = [3, 4, 5]
left[0] = 10; right[0] = 30;                   // v == [10, 2, 30, 4, 5]
get_two_mut(&mut v, 4, 0)                      => Some((&mut 5, &mut 10))
get_two_mut(&mut v, 1, 1)                      => None
chunks_exact_mut(&mut [1, 2, 3, 4, 5], 2)      => [[1, 2], [3, 4]]
```

## Constraints

- No special constraints.

## Hints

1. Take `let ptr = slice.as_mut_ptr();` once, then build each part with
   `std::slice::from_raw_parts_mut(ptr.add(offset), length)`.
2. The returned slices must not overlap: that is the whole safety argument.
3. `split_at_mut` is a good building block for the other three.

## Starter code

Write your code in [`src/practice/p0932_safe_split_at_mut.rs`](../src/practice/p0932_safe_split_at_mut.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0932
```

```rust
pub fn split_at_mut<T>(slice: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    todo!("Write your solution here")
}

pub fn get_two_mut<T>(slice: &mut [T], i: usize, j: usize) -> Option<(&mut T, &mut T)> {
    todo!("Write your solution here")
}

pub fn first_rest_last_mut<T>(slice: &mut [T]) -> Option<(&mut T, &mut [T], &mut T)> {
    todo!("Write your solution here")
}

pub fn chunks_exact_mut<T>(slice: &mut [T], size: usize) -> Vec<&mut [T]> {
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
    fn both_halves_are_mutable() {
        let mut v = [1, 2, 3, 4, 5];
        let (left, right) = split_at_mut(&mut v, 2);
        assert_eq!((left.len(), right.len()), (2, 3));
        left[0] = 10;
        right[0] = 30;
        assert_eq!(v, [10, 2, 30, 4, 5]);
        let (all, none) = split_at_mut(&mut v, 5);
        assert_eq!((all.len(), none.len()), (5, 0));
        let (none, all) = split_at_mut(&mut v, 0);
        assert_eq!((none.len(), all.len()), (0, 5));
    }

    #[test]
    #[should_panic(expected = "mid out of bounds")]
    fn mid_past_the_end_panics() {
        let mut v = [1, 2];
        let _ = split_at_mut(&mut v, 3);
    }

    #[test]
    fn two_distinct_elements() {
        let mut v = [10, 2, 30, 4, 5];
        let (a, b) = get_two_mut(&mut v, 4, 0).unwrap();
        assert_eq!((*a, *b), (5, 10));
        std::mem::swap(a, b);
        assert_eq!(v, [5, 2, 30, 4, 10]);
        assert!(get_two_mut(&mut v, 1, 1).is_none());
        assert!(get_two_mut(&mut v, 1, 5).is_none());
        let mut words = vec![String::from("a"), String::from("b")];
        if let Some((x, y)) = get_two_mut(&mut words, 0, 1) {
            x.push_str(y);
        }
        assert_eq!(words, ["ab", "b"]);
    }

    #[test]
    fn first_middle_last() {
        let mut v = [1, 2, 3, 4];
        let (first, middle, last) = first_rest_last_mut(&mut v).unwrap();
        *first += 100;
        middle.reverse();
        *last *= 10;
        assert_eq!(v, [101, 3, 2, 40]);
        let mut pair = [1, 2];
        let (a, middle, b) = first_rest_last_mut(&mut pair).unwrap();
        assert!(middle.is_empty());
        std::mem::swap(a, b);
        assert_eq!(pair, [2, 1]);
        assert!(first_rest_last_mut(&mut [1]).is_none());
    }

    #[test]
    fn exact_chunks() {
        let mut v = [1, 2, 3, 4, 5];
        let chunks = chunks_exact_mut(&mut v, 2);
        assert_eq!(chunks.len(), 2);
        for chunk in chunks {
            chunk.swap(0, 1);
        }
        assert_eq!(v, [2, 1, 4, 3, 5]);
        assert!(chunks_exact_mut(&mut v, 6).is_empty());
        let mut big: Vec<u32> = (0..10_000).collect();
        for chunk in chunks_exact_mut(&mut big, 100) {
            chunk[0] = 0;
        }
        assert_eq!(big.iter().filter(|&&x| x == 0).count(), 100);
    }

    #[test]
    #[should_panic(expected = "chunk size must be positive")]
    fn zero_chunk_size_panics() {
        let _ = chunks_exact_mut(&mut [1, 2, 3], 0);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** `split_at_mut` checks `mid <= len` and creates two slices from the same base pointer
that cover `[0, mid)` and `[mid, len)`. Every other function reduces to it: `get_two_mut` splits
at the larger index so the two elements land in different halves, `first_rest_last_mut` splits
twice, and `chunks_exact_mut` repeatedly splits the remaining tail.

**Complexity:** `O(1)` per split; `chunks_exact_mut` is `O(len / size)`.

**Rust concepts:**
- `from_raw_parts_mut(ptr, len)` requires a valid, aligned pointer to `len` initialised elements
  that no other live reference may access for the lifetime of the result.
- The signature `fn(&'a mut [T]) -> (&'a mut [T], &'a mut [T])` (elided here) makes both halves
  borrow the original slice, so the caller cannot use it while the halves are alive.
- Only one function contains `unsafe`; the others are safe code on top of it, which keeps the
  amount of code that needs manual review small.

```rust
pub fn split_at_mut<T>(slice: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    let len = slice.len();
    assert!(mid <= len, "mid out of bounds");
    let ptr = slice.as_mut_ptr();
    // SAFETY: `mid <= len`, so `[0, mid)` and `[mid, len)` are in bounds and disjoint, and both
    // come from a unique `&mut` borrow that the returned slices now take over.
    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

pub fn get_two_mut<T>(slice: &mut [T], i: usize, j: usize) -> Option<(&mut T, &mut T)> {
    if i == j || i >= slice.len() || j >= slice.len() {
        return None;
    }
    let (low, high) = (i.min(j), i.max(j));
    let (left, right) = split_at_mut(slice, high);
    let (low_ref, high_ref) = (&mut left[low], &mut right[0]);
    if i < j {
        Some((low_ref, high_ref))
    } else {
        Some((high_ref, low_ref))
    }
}

pub fn first_rest_last_mut<T>(slice: &mut [T]) -> Option<(&mut T, &mut [T], &mut T)> {
    let len = slice.len();
    if len < 2 {
        return None;
    }
    let (first, rest) = split_at_mut(slice, 1);
    let (middle, last) = split_at_mut(rest, len - 2);
    Some((&mut first[0], middle, &mut last[0]))
}

pub fn chunks_exact_mut<T>(slice: &mut [T], size: usize) -> Vec<&mut [T]> {
    assert!(size > 0, "chunk size must be positive");
    let mut chunks = Vec::with_capacity(slice.len() / size);
    let mut rest = slice;
    while rest.len() >= size {
        let (chunk, tail) = split_at_mut(rest, size);
        chunks.push(chunk);
        rest = tail;
    }
    chunks
}
```

Full tested solution: [`src/solutions/p0932_safe_split_at_mut.rs`](../src/solutions/p0932_safe_split_at_mut.rs)

</details>
