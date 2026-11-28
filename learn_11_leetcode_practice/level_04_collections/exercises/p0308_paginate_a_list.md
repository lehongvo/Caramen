# Problem 0308: Paginate a List

- **Difficulty:** Easy
- **Topics:** `slice::chunks`, `chunks_exact`, `remainder`, `Iterator::nth`, returning sub-slices
- **Source:** Original · [`slice::chunks`](https://doc.rust-lang.org/std/primitive.slice.html#method.chunks)

## Problem

A web page shows search results `page_size` at a time. Implement:

1. `paginate(items: &[String], page_size: usize) -> Vec<Vec<String>>` returns all pages in
   order. Every page has `page_size` items except possibly the last one, which holds what is
   left. If `page_size` is `0`, return an empty vector.
2. `page(items: &[String], page_size: usize, page_number: usize) -> Option<&[String]>` returns
   page number `page_number`, counting from **1**, as a slice that borrows from `items`. Return
   `None` if that page does not exist or if `page_size` or `page_number` is `0`.
3. `full_pages_and_rest(items: &[i32], page_size: usize) -> (usize, &[i32])` returns the number
   of **completely full** pages and the leftover items that do not fill a page. `page_size` is
   at least `1` here.

## Examples

```text
items = ["a", "b", "c", "d", "e"]
paginate(items, 2)   => [["a", "b"], ["c", "d"], ["e"]]
paginate(items, 0)   => []
page(items, 2, 2)    => Some(["c", "d"])
page(items, 2, 4)    => None

full_pages_and_rest([1, 2, 3, 4, 5, 6, 7], 3) => (2, [7])
full_pages_and_rest([1, 2, 3, 4, 5, 6], 3)    => (2, [])
```

## Constraints

- `0 <= items.len() <= 10^4`
- `page_size` and `page_number` can be any `usize` (except `page_size >= 1` in function 3).

## Hints

1. `chunks(n)` yields consecutive sub-slices of length `n` (the last one may be shorter). It
   **panics** when `n == 0`.
2. `nth(k)` on an iterator skips `k` items and returns the next one as an `Option`.
3. `chunks_exact(n)` yields only full chunks; its `remainder()` method gives the rest.

## Starter code

Write your code in [`src/practice/p0308_paginate_a_list.rs`](../src/practice/p0308_paginate_a_list.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0308
```

```rust
/// Splits `items` into pages of `page_size` items (the last page may be shorter).
pub fn paginate(items: &[String], page_size: usize) -> Vec<Vec<String>> {
    todo!("Write your solution here")
}

/// Returns page `page_number` (1-based) without copying the items.
pub fn page(items: &[String], page_size: usize, page_number: usize) -> Option<&[String]> {
    todo!("Write your solution here")
}

/// Returns the number of full pages and the items left over.
pub fn full_pages_and_rest(items: &[i32], page_size: usize) -> (usize, &[i32]) {
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

    fn strings(words: &[&str]) -> Vec<String> {
        words.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn paginate_example() {
        let items = strings(&["a", "b", "c", "d", "e"]);
        assert_eq!(
            paginate(&items, 2),
            vec![strings(&["a", "b"]), strings(&["c", "d"]), strings(&["e"])]
        );
        assert_eq!(paginate(&items, 0), Vec::<Vec<String>>::new());
        assert_eq!(paginate(&items, 10), vec![items.clone()]);
        assert_eq!(paginate(&[], 3), Vec::<Vec<String>>::new());
    }

    #[test]
    fn page_example() {
        let items = strings(&["a", "b", "c", "d", "e"]);
        assert_eq!(page(&items, 2, 2), Some(&items[2..4]));
        assert_eq!(page(&items, 2, 3), Some(&items[4..]));
        assert_eq!(page(&items, 2, 4), None);
        assert_eq!(page(&items, 2, 0), None);
        assert_eq!(page(&items, 0, 1), None);
    }

    #[test]
    fn full_pages_and_rest_examples() {
        assert_eq!(
            full_pages_and_rest(&[1, 2, 3, 4, 5, 6, 7], 3),
            (2, &[7][..])
        );
        assert_eq!(full_pages_and_rest(&[1, 2, 3, 4, 5, 6], 3), (2, &[][..]));
        assert_eq!(full_pages_and_rest(&[1, 2], 5), (0, &[1, 2][..]));
    }

    #[test]
    fn large_list() {
        let items: Vec<String> = (0..10_000).map(|i| i.to_string()).collect();
        let pages = paginate(&items, 30);
        assert_eq!(pages.len(), 334);
        assert_eq!(
            pages[333],
            strings(&[
                "9990", "9991", "9992", "9993", "9994", "9995", "9996", "9997", "9998", "9999"
            ])
        );
        assert_eq!(page(&items, 30, 334).map(|p| p.len()), Some(10));
        assert_eq!(page(&items, 30, usize::MAX), None);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** slices already know how to cut themselves into pieces.

- `paginate` maps every chunk to an owned `Vec<String>` with `to_vec()` (which clones the
  strings, because the result must own them).
- `page` turns the 1-based number into a 0-based position with `checked_sub(1)` (which is
  `None` for `0`) and asks the chunk iterator for that element; no strings are cloned.
- `full_pages_and_rest` keeps the `ChunksExact` iterator in a variable so it can both count the
  full chunks and return `remainder()`.

**Rust concepts:**
- `page` returns `Option<&[String]>`. With a single reference parameter, lifetime elision ties
  the returned slice to `items`: it borrows the caller's data instead of copying it.
- `?` works on `Option` too: `page_number.checked_sub(1)?` returns `None` early.
- `remainder()` returns a slice that borrows from `items`, not from the iterator. `count()`
  consumes the iterator (it takes `self` by value), so read the remainder **before** counting;
  the remainder slice stays valid afterwards.

```rust
/// Splits `items` into pages of `page_size` items (the last page may be shorter).
pub fn paginate(items: &[String], page_size: usize) -> Vec<Vec<String>> {
    if page_size == 0 {
        return Vec::new();
    }
    items
        .chunks(page_size)
        .map(|chunk| chunk.to_vec())
        .collect()
}

/// Returns page `page_number` (1-based) without copying the items.
pub fn page(items: &[String], page_size: usize, page_number: usize) -> Option<&[String]> {
    if page_size == 0 {
        return None;
    }
    let index = page_number.checked_sub(1)?;
    items.chunks(page_size).nth(index)
}

/// Returns the number of full pages and the items left over.
pub fn full_pages_and_rest(items: &[i32], page_size: usize) -> (usize, &[i32]) {
    let pages = items.chunks_exact(page_size);
    let rest = pages.remainder();
    (pages.count(), rest)
}
```

Full tested solution: [`src/solutions/p0308_paginate_a_list.rs`](../src/solutions/p0308_paginate_a_list.rs)

</details>
