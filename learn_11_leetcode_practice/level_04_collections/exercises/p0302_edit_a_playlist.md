# Problem 0302: Edit a Playlist

- **Difficulty:** Easy
- **Topics:** `Vec::push`, `insert`, `remove`, `swap`, `pop`, avoiding index panics, `match` on an enum
- **Source:** Original · [The Rust Book 8.1](https://doc.rust-lang.org/book/ch08-01-vectors.html) · [`Vec::insert`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.insert)

## Problem

A music app keeps a playlist as a `Vec<String>`. The user sends a list of edits, described by
this enum (positions are 0-based):

```text
pub enum Edit {
    Append(String),          // add the song at the end
    InsertAt(usize, String), // put the song at that position, shifting later songs right
    RemoveAt(usize),         // delete the song at that position, shifting later songs left
    Swap(usize, usize),      // exchange the songs at the two positions
    RemoveLast,              // delete the last song
}
```

Implement `apply_edits(playlist: &mut Vec<String>, edits: Vec<Edit>) -> usize`. Apply the edits
in order. An edit that is **impossible** for the playlist as it is at that moment is skipped:

- `InsertAt(i, _)` is possible when `i <= len` (inserting at `len` appends);
- `RemoveAt(i)` is possible when `i < len`;
- `Swap(i, j)` is possible when both `i < len` and `j < len`;
- `RemoveLast` is possible when the playlist is not empty;
- `Append` is always possible.

Return the number of skipped edits. The function must never panic.

## Examples

```text
playlist = ["Intro", "Song"]
edits    = [Append("Outro"), InsertAt(1, "Bridge"), Swap(0, 3), RemoveAt(9)]
result   = 1   (RemoveAt(9) is skipped)
playlist = ["Outro", "Bridge", "Song", "Intro"]

playlist = []
edits    = [RemoveLast, InsertAt(0, "A"), InsertAt(2, "B"), RemoveLast]
result   = 2   (the first RemoveLast and InsertAt(2, ..) are skipped)
playlist = []
```

## Constraints

- `0 <= playlist.len(), edits.len() <= 1000`

## Hints

1. `Vec::insert(i, x)` panics if `i > len`, `Vec::remove(i)` panics if `i >= len`, and
   `swap(i, j)` panics if either index is out of bounds. Check first.
2. `pop()` returns an `Option`, so `RemoveLast` can be decided by matching on its result.

## Starter code

Write your code in [`src/practice/p0302_edit_a_playlist.rs`](../src/practice/p0302_edit_a_playlist.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0302
```

```rust
/// One change to a playlist. Positions are 0-based.
#[derive(Debug, Clone, PartialEq)]
pub enum Edit {
    /// Add the song at the end.
    Append(String),
    /// Insert the song at the position, shifting later songs to the right.
    InsertAt(usize, String),
    /// Remove the song at the position.
    RemoveAt(usize),
    /// Exchange the songs at the two positions.
    Swap(usize, usize),
    /// Remove the last song.
    RemoveLast,
}

/// Applies the edits in order and returns how many of them were impossible (and skipped).
pub fn apply_edits(playlist: &mut Vec<String>, edits: Vec<Edit>) -> usize {
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

    fn songs(names: &[&str]) -> Vec<String> {
        names.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn example_mixed_edits() {
        let mut playlist = songs(&["Intro", "Song"]);
        let edits = vec![
            Edit::Append("Outro".to_string()),
            Edit::InsertAt(1, "Bridge".to_string()),
            Edit::Swap(0, 3),
            Edit::RemoveAt(9),
        ];
        assert_eq!(apply_edits(&mut playlist, edits), 1);
        assert_eq!(playlist, songs(&["Outro", "Bridge", "Song", "Intro"]));
    }

    #[test]
    fn example_empty_playlist() {
        let mut playlist = Vec::new();
        let edits = vec![
            Edit::RemoveLast,
            Edit::InsertAt(0, "A".to_string()),
            Edit::InsertAt(2, "B".to_string()),
            Edit::RemoveLast,
        ];
        assert_eq!(apply_edits(&mut playlist, edits), 2);
        assert!(playlist.is_empty());
    }

    #[test]
    fn insert_at_len_appends_and_remove_shifts() {
        let mut playlist = songs(&["A", "B", "C"]);
        let edits = vec![
            Edit::InsertAt(3, "D".to_string()),
            Edit::RemoveAt(0),
            Edit::Swap(1, 1),
        ];
        assert_eq!(apply_edits(&mut playlist, edits), 0);
        assert_eq!(playlist, songs(&["B", "C", "D"]));
    }

    #[test]
    fn swap_out_of_bounds_is_skipped() {
        let mut playlist = songs(&["A", "B"]);
        let edits = vec![Edit::Swap(0, 2), Edit::Swap(5, 0), Edit::Swap(0, 1)];
        assert_eq!(apply_edits(&mut playlist, edits), 2);
        assert_eq!(playlist, songs(&["B", "A"]));
    }

    #[test]
    fn many_edits() {
        let mut playlist = Vec::new();
        let mut edits: Vec<Edit> = (0..500).map(|i| Edit::InsertAt(0, i.to_string())).collect();
        edits.extend((0..600).map(|_| Edit::RemoveLast));
        assert_eq!(apply_edits(&mut playlist, edits), 100);
        assert!(playlist.is_empty());
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** `match` on each edit, test the precondition for that kind of edit, and either call
the corresponding `Vec` method or count the edit as skipped.

**Rust concepts:**
- `insert` and `remove` shift every later element, so they are `O(n)`; `push` and `pop` work
  at the end and are `O(1)` (amortized for `push`).
- `remove(i)` returns the removed element; here it is simply dropped.
- `pop()` returns `Option<T>`: `is_some()` tells whether something was removed, so no separate
  emptiness check is needed.
- Matching on `Edit` by value moves the `String` out of `Append`/`InsertAt` straight into the
  playlist, without cloning.

```rust
/// One change to a playlist. Positions are 0-based.
#[derive(Debug, Clone, PartialEq)]
pub enum Edit {
    /// Add the song at the end.
    Append(String),
    /// Insert the song at the position, shifting later songs to the right.
    InsertAt(usize, String),
    /// Remove the song at the position.
    RemoveAt(usize),
    /// Exchange the songs at the two positions.
    Swap(usize, usize),
    /// Remove the last song.
    RemoveLast,
}

/// Applies the edits in order and returns how many of them were impossible (and skipped).
pub fn apply_edits(playlist: &mut Vec<String>, edits: Vec<Edit>) -> usize {
    let mut skipped = 0;
    for edit in edits {
        let len = playlist.len();
        let applied = match edit {
            Edit::Append(song) => {
                playlist.push(song);
                true
            }
            Edit::InsertAt(i, song) if i <= len => {
                playlist.insert(i, song);
                true
            }
            Edit::RemoveAt(i) if i < len => {
                playlist.remove(i);
                true
            }
            Edit::Swap(i, j) if i < len && j < len => {
                playlist.swap(i, j);
                true
            }
            Edit::RemoveLast => playlist.pop().is_some(),
            // An InsertAt, RemoveAt or Swap whose guard failed.
            _ => false,
        };
        if !applied {
            skipped += 1;
        }
    }
    skipped
}
```

Full tested solution: [`src/solutions/p0302_edit_a_playlist.rs`](../src/solutions/p0302_edit_a_playlist.rs)

</details>
