# Problem 0916: A Lending Iterator with GATs

- **Difficulty:** Hard
- **Topics:** generic associated types (`type Item<'a> where Self: 'a`), lending vs. ordinary iterators, `&mut [T]` windows, reusable buffers
- **Source:** Original · [Reference: Associated types (generic)](https://doc.rust-lang.org/reference/items/associated-items.html#associated-types) · [std: `slice::windows`](https://doc.rust-lang.org/std/primitive.slice.html#method.windows)

## Problem

`slice::windows` yields overlapping **shared** windows, but there is no `windows_mut`: an ordinary
`Iterator` could hand out two overlapping `&mut` windows at the same time, which Rust forbids. A
**lending iterator** fixes this: each item borrows the iterator itself, so the previous item must
be dropped before `next` can be called again. Generic associated types make the trait expressible:

```text
pub trait LendingIterator {
    type Item<'a> where Self: 'a;
    fn next(&mut self) -> Option<Self::Item<'_>>;
    fn count(mut self) -> usize where Self: Sized { ... }   // provided method
}
```

Implement:

1. The provided method `count`: the number of items left (consumes the lender).
2. `WindowsMut<'s, T>` from `windows_mut(slice, size)`: lends `&mut [T]` windows of length `size`
   starting at index 0, 1, 2, ... (`slice.len() - size + 1` windows, none if `size > len`).
   `windows_mut` panics with `"window size must be positive"` when `size == 0`.
3. `prefix_sums_in_place(values: &mut [i64])` using `windows_mut(values, 2)`: afterwards
   `values[i]` is the sum of the original `values[0..=i]`.
4. `UpperLines<'s>` from `upper_lines(text)`: lends each line of `text` (split like `str::lines`)
   converted to uppercase, as a `&str` borrowed from **one** internal `String` buffer that is
   cleared and refilled on every call (no allocation per line once the buffer is large enough).

## Examples

```text
let mut data = [1, 2, 3, 4];
let mut windows = windows_mut(&mut data, 2);
while let Some(w) = windows.next() { w[1] += w[0]; }
data                                   => [1, 3, 6, 10]
windows_mut(&mut [1, 2, 3], 2).count() => 2
upper_lines("ab\ncd") lends "AB", then "CD", then None
```

## Constraints

- No special constraints.

## Hints

1. In `impl<T> LendingIterator for WindowsMut<'_, T>`, write
   `type Item<'a> = &'a mut [T] where Self: 'a;`.
2. `self.slice.get_mut(self.start..self.start + self.size)` reborrows the slice for the
   lifetime of `&mut self` only, which is exactly what a lending iterator wants.
3. For `UpperLines`, keep a `std::str::Lines<'s>` and a `String` buffer; return
   `Some(self.buffer.as_str())`.

## Starter code

Write your code in [`src/practice/p0916_lending_iterator_gat.rs`](../src/practice/p0916_lending_iterator_gat.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0916
```

```rust
pub trait LendingIterator {
    type Item<'a>
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>>;

    /// Consumes the lender and returns how many items it produced.
    fn count(mut self) -> usize
    where
        Self: Sized,
    {
        todo!("Write your solution here")
    }
}

/// Lends overlapping mutable windows of a slice.
pub struct WindowsMut<'s, T> {
    slice: &'s mut [T],
    size: usize,
    start: usize,
}

pub fn windows_mut<T>(slice: &mut [T], size: usize) -> WindowsMut<'_, T> {
    todo!("Write your solution here")
}

impl<T> LendingIterator for WindowsMut<'_, T> {
    type Item<'a>
        = &'a mut [T]
    where
        Self: 'a;

    fn next(&mut self) -> Option<&mut [T]> {
        todo!("Write your solution here")
    }
}

pub fn prefix_sums_in_place(values: &mut [i64]) {
    todo!("Write your solution here")
}

/// Lends each line of a text in uppercase, from one reused buffer.
pub struct UpperLines<'s> {
    lines: std::str::Lines<'s>,
    buffer: String,
}

pub fn upper_lines(text: &str) -> UpperLines<'_> {
    todo!("Write your solution here")
}

impl UpperLines<'_> {
    /// The capacity of the internal buffer (used by the tests to check it is reused).
    pub fn buffer_capacity(&self) -> usize {
        self.buffer.capacity()
    }
}

impl LendingIterator for UpperLines<'_> {
    type Item<'a>
        = &'a str
    where
        Self: 'a;

    fn next(&mut self) -> Option<&str> {
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
    fn mutable_windows_update_the_slice() {
        let mut data = [1, 2, 3, 4];
        let mut windows = windows_mut(&mut data, 2);
        while let Some(w) = windows.next() {
            w[1] += w[0];
        }
        assert_eq!(data, [1, 3, 6, 10]);
    }

    #[test]
    fn window_counts() {
        assert_eq!(windows_mut(&mut [1, 2, 3], 2).count(), 2);
        assert_eq!(windows_mut(&mut [1, 2, 3], 3).count(), 1);
        assert_eq!(windows_mut(&mut [1, 2, 3], 4).count(), 0);
        assert_eq!(windows_mut::<u8>(&mut [], 1).count(), 0);
    }

    #[test]
    fn windows_of_three_see_earlier_writes() {
        let mut data = [0, 0, 1, 0, 0, 0];
        let mut windows = windows_mut(&mut data, 3);
        // Each window sets its last cell to the sum of its first two.
        while let Some(w) = windows.next() {
            w[2] += w[0] + w[1];
        }
        assert_eq!(data, [0, 0, 1, 1, 2, 3]);
    }

    #[test]
    #[should_panic(expected = "window size must be positive")]
    fn zero_size_panics() {
        let _ = windows_mut(&mut [1], 0);
    }

    #[test]
    fn prefix_sums() {
        let mut values = [5, -2, 7, 0, 1];
        prefix_sums_in_place(&mut values);
        assert_eq!(values, [5, 3, 10, 10, 11]);
        let mut big: Vec<i64> = vec![1; 100_000];
        prefix_sums_in_place(&mut big);
        assert_eq!(big[99_999], 100_000);
        let mut empty: [i64; 0] = [];
        prefix_sums_in_place(&mut empty);
    }

    #[test]
    fn upper_lines_reuse_one_buffer() {
        let mut lines = upper_lines("ab\ncd\n\nstra\u{df}e");
        assert_eq!(lines.next(), Some("AB"));
        assert_eq!(lines.next(), Some("CD"));
        assert_eq!(lines.next(), Some(""));
        assert_eq!(lines.next(), Some("STRASSE"));
        assert_eq!(lines.next(), None);
        let text = "a long first line\nshort\nx";
        let mut lines = upper_lines(text);
        let first_len = lines.next().map(str::len);
        let capacity = lines.buffer_capacity();
        assert_eq!(first_len, Some(17));
        assert_eq!(lines.next(), Some("SHORT"));
        assert_eq!(lines.next(), Some("X"));
        assert_eq!(lines.buffer_capacity(), capacity);
        assert_eq!(upper_lines("1\n2\n3").count(), 3);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** `WindowsMut` remembers the start of the next window and reborrows that sub-slice
mutably from `&mut self`; because the returned window borrows the lender, two windows can never
be alive at once, so the overlap is harmless. `UpperLines` refills its buffer and lends a `&str`
into it. `count` loops with `while let` because a lending iterator cannot implement `Iterator`.

**Complexity:** `prefix_sums_in_place` is `O(n)`; `UpperLines` is `O(total length)`.

**Rust concepts:**
- A GAT lets the item type depend on the lifetime of the `&mut self` borrow in `next`; with a plain
  `type Item` the item could not mention that lifetime at all.
- The `where Self: 'a` bound says the lender outlives the loan; it is required on the trait's
  declaration so implementations may borrow from `self`.
- Lending iterators do not work with `for` loops or adapters such as `map`: use `while let`.

```rust
pub trait LendingIterator {
    type Item<'a>
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>>;

    /// Consumes the lender and returns how many items it produced.
    fn count(mut self) -> usize
    where
        Self: Sized,
    {
        let mut n = 0;
        while self.next().is_some() {
            n += 1;
        }
        n
    }
}

/// Lends overlapping mutable windows of a slice.
pub struct WindowsMut<'s, T> {
    slice: &'s mut [T],
    size: usize,
    start: usize,
}

pub fn windows_mut<T>(slice: &mut [T], size: usize) -> WindowsMut<'_, T> {
    assert!(size > 0, "window size must be positive");
    WindowsMut {
        slice,
        size,
        start: 0,
    }
}

impl<T> LendingIterator for WindowsMut<'_, T> {
    type Item<'a>
        = &'a mut [T]
    where
        Self: 'a;

    fn next(&mut self) -> Option<&mut [T]> {
        let window = self.slice.get_mut(self.start..self.start + self.size)?;
        self.start += 1;
        Some(window)
    }
}

pub fn prefix_sums_in_place(values: &mut [i64]) {
    let mut windows = windows_mut(values, 2);
    while let Some(window) = windows.next() {
        window[1] += window[0];
    }
}

/// Lends each line of a text in uppercase, from one reused buffer.
pub struct UpperLines<'s> {
    lines: std::str::Lines<'s>,
    buffer: String,
}

pub fn upper_lines(text: &str) -> UpperLines<'_> {
    UpperLines {
        lines: text.lines(),
        buffer: String::new(),
    }
}

impl UpperLines<'_> {
    /// The capacity of the internal buffer (used by the tests to check it is reused).
    pub fn buffer_capacity(&self) -> usize {
        self.buffer.capacity()
    }
}

impl LendingIterator for UpperLines<'_> {
    type Item<'a>
        = &'a str
    where
        Self: 'a;

    fn next(&mut self) -> Option<&str> {
        let line = self.lines.next()?;
        self.buffer.clear();
        self.buffer
            .extend(line.chars().flat_map(char::to_uppercase));
        Some(self.buffer.as_str())
    }
}
```

Full tested solution: [`src/solutions/p0916_lending_iterator_gat.rs`](../src/solutions/p0916_lending_iterator_gat.rs)

</details>
