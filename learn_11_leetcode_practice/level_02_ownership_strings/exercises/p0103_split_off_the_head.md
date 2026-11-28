# Problem 0103: Split Off the Head

- **Difficulty:** Easy
- **Topics:** moving values out of a `Vec`, `into_iter`, `collect`, `Option`
- **Source:** Original · [The Rust Book 4.1](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

## Problem

You receive a queue of names as an owned `Vec<String>`. Split it into its first element (the
*head*) and everything after it (the *rest*), without cloning any `String`.

Implement `split_off_head(items: Vec<String>) -> Option<(String, Vec<String>)>`:

- if `items` is empty, return `None`;
- otherwise return `Some((head, rest))` where `head` is the first element and `rest` holds the
  remaining elements in their original order (possibly empty).

You cannot simply write `let head = items[0];`: indexing gives you a place *inside* the vector,
and moving a `String` out of it would leave a hole (error E0507, "cannot move out of index").
Because the function owns the whole `Vec`, it can instead take the vector apart.

## Examples

```text
Input:  items = ["ann", "bob", "cy"]
Output: Some(("ann", ["bob", "cy"]))

Input:  items = ["solo"]
Output: Some(("solo", []))

Input:  items = []
Output: None
```

## Constraints

- `0 <= items.len() <= 10^4`
- Do not call `.clone()`.

## Hints

1. `items.into_iter()` consumes the vector and yields each `String` **by value**.
2. Call `.next()` once to get the head, then `.collect()` the rest of the same iterator.
3. Another option: `Vec::remove(0)` moves the first element out and shifts the others left.

## Starter code

Write your code in [`src/practice/p0103_split_off_the_head.rs`](../src/practice/p0103_split_off_the_head.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0103
```

```rust
pub fn split_off_head(items: Vec<String>) -> Option<(String, Vec<String>)> {
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
    fn example_three_items() {
        let result = split_off_head(owned(&["ann", "bob", "cy"]));
        assert_eq!(result, Some((String::from("ann"), owned(&["bob", "cy"]))));
    }

    #[test]
    fn example_single_item() {
        let result = split_off_head(owned(&["solo"]));
        assert_eq!(result, Some((String::from("solo"), Vec::new())));
    }

    #[test]
    fn example_empty() {
        assert_eq!(split_off_head(Vec::new()), None);
    }

    #[test]
    fn repeated_splitting_walks_the_queue() {
        let mut queue = owned(&["a", "b", "c"]);
        let mut seen = Vec::new();
        while let Some((head, rest)) = split_off_head(queue) {
            seen.push(head);
            queue = rest;
        }
        assert_eq!(seen, owned(&["a", "b", "c"]));
    }

    #[test]
    fn large_queue_keeps_order() {
        let items: Vec<String> = (0..10_000).map(|i| i.to_string()).collect();
        let (head, rest) = split_off_head(items).expect("non-empty input");
        assert_eq!(head, "0");
        assert_eq!(rest.len(), 9_999);
        assert_eq!(rest[0], "1");
        assert_eq!(rest[9_998], "9999");
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** `into_iter()` turns the owned vector into an iterator of owned `String`s. The first
`next()` is the head (or `None` for an empty vector); collecting what is left of the iterator
gives the rest, still in order. Every `String` is moved, never copied.

**Rust concepts:**
- `Vec::into_iter(self)` takes ownership of the vector, so its elements can be moved out one by
  one. `iter()` would only lend `&String`s.
- Indexing (`items[0]`) produces a *place*, and you cannot move out of a place you do not fully
  own; that is why `into_iter`, `remove`, `pop` or `swap_remove` exist.
- `Option::map` transforms `Some(head)` into `Some((head, rest))` and leaves `None` alone. The
  closure moves the iterator in, which is fine because `next()` has already returned.

```rust
pub fn split_off_head(items: Vec<String>) -> Option<(String, Vec<String>)> {
    let mut rest = items.into_iter();
    rest.next().map(|head| (head, rest.collect()))
}
```

Full tested solution: [`src/solutions/p0103_split_off_the_head.rs`](../src/solutions/p0103_split_off_the_head.rs)

</details>
