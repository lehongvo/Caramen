# Problem 0329: First Repeated Visitor

- **Difficulty:** Easy
- **Topics:** `HashSet::insert` returns `bool`, `remove` returns `bool`, `len`, sorting a set's contents
- **Source:** Original · [`HashSet`](https://doc.rust-lang.org/std/collections/struct.HashSet.html)

## Problem

A museum logs the names of its visitors. Implement three functions using a
`std::collections::HashSet`:

1. `first_repeat(visitors: &[String]) -> Option<&str>` returns the first name that appears
   for the **second** time while reading the log from the start, or `None` if every name is
   unique.
2. `distinct_count(visitors: &[String]) -> usize` returns how many different names appear.
3. `still_inside(events: &[(&str, bool)]) -> (Vec<String>, usize)` processes badge events in
   order: `(name, true)` means the person enters and `(name, false)` that they leave. Entering
   while already inside, or leaving while not inside, is **invalid** and ignored. Return the
   names of the people inside at the end, **sorted alphabetically**, and the number of invalid
   events.

## Examples

```text
visitors = ["Ann", "Bob", "Cy", "Bob", "Ann"]
first_repeat   => Some("Bob")     (Bob repeats at index 3, before Ann repeats at index 4)
distinct_count => 3

events = [("Zed", true), ("Amy", true), ("Zed", true), ("Bob", false), ("Zed", false), ("Kim", true)]
still_inside => (["Amy", "Kim"], 2)   (the second "Zed enters" and "Bob leaves" are invalid)
```

## Constraints

- `0 <= visitors.len(), events.len() <= 10^5`

## Hints

1. `set.insert(x)` returns `false` when `x` was already in the set.
2. `set.remove(&x)` returns `false` when `x` was not in the set.
3. A `HashSet` iterates in an **unspecified** order; collect into a `Vec` and sort it before
   returning it.

## Starter code

Write your code in [`src/practice/p0329_first_repeated_visitor.rs`](../src/practice/p0329_first_repeated_visitor.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0329
```

```rust
use std::collections::HashSet;

/// Returns the first name seen a second time.
pub fn first_repeat(visitors: &[String]) -> Option<&str> {
    todo!("Write your solution here")
}

/// Returns the number of different names.
pub fn distinct_count(visitors: &[String]) -> usize {
    todo!("Write your solution here")
}

/// Returns the sorted names inside at the end and the number of invalid events.
pub fn still_inside(events: &[(&str, bool)]) -> (Vec<String>, usize) {
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
    fn first_repeat_example() {
        let visitors = strings(&["Ann", "Bob", "Cy", "Bob", "Ann"]);
        assert_eq!(first_repeat(&visitors), Some("Bob"));
        assert_eq!(first_repeat(&strings(&["A", "B", "C"])), None);
        assert_eq!(first_repeat(&[]), None);
    }

    #[test]
    fn distinct_count_example() {
        assert_eq!(
            distinct_count(&strings(&["Ann", "Bob", "Cy", "Bob", "Ann"])),
            3
        );
        assert_eq!(distinct_count(&[]), 0);
        assert_eq!(distinct_count(&strings(&["x", "X"])), 2);
    }

    #[test]
    fn still_inside_example() {
        let events = [
            ("Zed", true),
            ("Amy", true),
            ("Zed", true),
            ("Bob", false),
            ("Zed", false),
            ("Kim", true),
        ];
        assert_eq!(still_inside(&events), (strings(&["Amy", "Kim"]), 2));
    }

    #[test]
    fn still_inside_empty_and_reentry() {
        assert_eq!(still_inside(&[]), (Vec::new(), 0));
        let events = [("A", true), ("A", false), ("A", true)];
        assert_eq!(still_inside(&events), (strings(&["A"]), 0));
    }

    #[test]
    fn large_log() {
        let visitors: Vec<String> = (0..100_000).map(|i| format!("v{}", i % 60_000)).collect();
        assert_eq!(distinct_count(&visitors), 60_000);
        assert_eq!(first_repeat(&visitors), Some("v0"));
        let names: Vec<String> = (0..1000).map(|i| format!("{i:04}")).collect();
        let events: Vec<(&str, bool)> = names.iter().rev().map(|n| (n.as_str(), true)).collect();
        let (inside, invalid) = still_inside(&events);
        assert_eq!(inside, names);
        assert_eq!(invalid, 0);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** a `HashSet` answers "have I seen this before?" in `O(1)` on average, and its `insert`
and `remove` methods already report whether they changed anything, so there is no need for a
separate `contains` call.

- `first_repeat` stops at the first name whose `insert` returns `false`.
- `distinct_count` is the size of the set of all names.
- `still_inside` treats a failed `insert` (entering twice) or a failed `remove` (leaving without
  being inside) as an invalid event.

**Rust concepts:**
- `HashSet<&str>` stores borrowed names, so no `String` is cloned while checking; the returned
  `&str` borrows from `visitors` (lifetime elision ties them together).
- `iter().map(String::as_str).find(|&name| !seen.insert(name))` combines the scan and the
  early exit. `find` passes a reference to each item (`&&str`); the pattern `|&name|` removes
  one level.
- The iteration order of a `HashSet` depends on a random hash seed and may change between runs;
  **never** let output (or tests) depend on it. Sorting makes the result deterministic.

```rust
use std::collections::HashSet;

/// Returns the first name seen a second time.
pub fn first_repeat(visitors: &[String]) -> Option<&str> {
    let mut seen: HashSet<&str> = HashSet::new();
    visitors
        .iter()
        .map(String::as_str)
        .find(|&name| !seen.insert(name))
}

/// Returns the number of different names.
pub fn distinct_count(visitors: &[String]) -> usize {
    visitors.iter().collect::<HashSet<_>>().len()
}

/// Returns the sorted names inside at the end and the number of invalid events.
pub fn still_inside(events: &[(&str, bool)]) -> (Vec<String>, usize) {
    let mut inside: HashSet<&str> = HashSet::new();
    let mut invalid = 0;
    for &(name, entering) in events {
        let ok = if entering {
            inside.insert(name)
        } else {
            inside.remove(name)
        };
        if !ok {
            invalid += 1;
        }
    }
    let mut names: Vec<String> = inside.into_iter().map(String::from).collect();
    names.sort_unstable();
    (names, invalid)
}
```

Full tested solution: [`src/solutions/p0329_first_repeated_visitor.rs`](../src/solutions/p0329_first_repeated_visitor.rs)

</details>
