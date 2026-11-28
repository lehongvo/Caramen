// ==================================================================
// PRACTICE 0482: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0482_into_iterator_playlist.md
// Run the tests:          cargo test practice::p0482
// ==================================================================

//! # Problem 0482: IntoIterator for Your Own Collection
//!
//! - **Difficulty:** Medium
//! - **Topics:** `IntoIterator` for `T`, `&T` and `&mut T`, associated types `Item` / `IntoIter`, `for` loops over a custom type, `FromIterator` for `collect`
//! - **Source:** Original · [std `IntoIterator`](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html) · [std `FromIterator`](https://doc.rust-lang.org/std/iter/trait.FromIterator.html) · [std `iter`: The three forms of iteration](https://doc.rust-lang.org/std/iter/index.html#the-three-forms-of-iteration)
//!
//! ## Problem
//!
//! A `for` loop works with anything that implements `IntoIterator`. Collections usually implement
//! it three times, one per kind of access:
//!
//! - `for song in playlist` consumes the playlist and yields owned `String`s;
//! - `for song in &playlist` borrows it and yields `&String`;
//! - `for song in &mut playlist` borrows it mutably and yields `&mut String`.
//!
//! The opposite direction, `iterator.collect::<Playlist>()`, needs `FromIterator`.
//!
//! Implement for `Playlist` (a list of song titles):
//!
//! - `Playlist::new()`, `add(&mut self, title: &str)`, `len(&self)`, `is_empty(&self)`.
//! - `IntoIterator for Playlist`, for `&Playlist` and for `&mut Playlist`, delegating to the inner
//!   `Vec<String>` (the associated types are already written for you).
//! - `FromIterator<String> for Playlist`.
//! - `total_chars(p: &Playlist) -> usize`: the total number of characters of all titles, using a
//!   `for` loop over `p`.
//! - `shout(p: &mut Playlist)`: turn every title into upper case **in place**, with a `for` loop.
//! - `into_sorted(p: Playlist) -> Vec<String>`: consume the playlist and return the titles sorted.
//!
//! ## Examples
//!
//! ```text
//! let mut p = Playlist::new(); p.add("Yesterday"); p.add("Help!");
//! p.len()                     -> 2
//! total_chars(&p)             -> 14
//! shout(&mut p);  then  for s in &p  yields "YESTERDAY", "HELP!"
//! into_sorted(p)              -> ["HELP!", "YESTERDAY"]
//! let q: Playlist = vec!["b".to_string(), "a".to_string()].into_iter().collect();
//! q.len()                     -> 2
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. `fn into_iter(self) -> Self::IntoIter { self.songs.into_iter() }`; for `&'a Playlist` use
//!    `self.songs.iter()`, for `&'a mut Playlist` use `self.songs.iter_mut()`.
//! 2. `fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self` can collect into the `Vec`.
//! 3. In `shout`, the loop variable is a `&mut String`: assign with `*song = song.to_uppercase();`.

#[derive(Debug, Default)]
pub struct Playlist {}

impl Playlist {
    pub fn new() -> Playlist {
        todo!("Write your solution here")
    }

    pub fn add(&mut self, title: &str) {
        todo!("Write your solution here")
    }

    pub fn len(&self) -> usize {
        todo!("Write your solution here")
    }

    pub fn is_empty(&self) -> bool {
        todo!("Write your solution here")
    }
}

impl IntoIterator for Playlist {
    type Item = String;
    type IntoIter = std::vec::IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        todo!("Write your solution here")
    }
}

impl<'a> IntoIterator for &'a Playlist {
    type Item = &'a String;
    type IntoIter = std::slice::Iter<'a, String>;

    fn into_iter(self) -> Self::IntoIter {
        todo!("Write your solution here")
    }
}

impl<'a> IntoIterator for &'a mut Playlist {
    type Item = &'a mut String;
    type IntoIter = std::slice::IterMut<'a, String>;

    fn into_iter(self) -> Self::IntoIter {
        todo!("Write your solution here")
    }
}

impl FromIterator<String> for Playlist {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        todo!("Write your solution here")
    }
}

pub fn total_chars(p: &Playlist) -> usize {
    todo!("Write your solution here")
}

pub fn shout(p: &mut Playlist) {
    todo!("Write your solution here")
}

pub fn into_sorted(p: Playlist) -> Vec<String> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Playlist {
        let mut p = Playlist::new();
        p.add("Yesterday");
        p.add("Help!");
        p
    }

    #[test]
    fn build_and_measure() {
        let p = sample();
        assert_eq!(p.len(), 2);
        assert!(!p.is_empty());
        assert!(Playlist::new().is_empty());
        assert_eq!(total_chars(&p), 14);
    }

    #[test]
    fn borrowing_loop_keeps_the_playlist() {
        let p = sample();
        let mut seen = Vec::new();
        for title in &p {
            seen.push(title.clone());
        }
        assert_eq!(seen, vec!["Yesterday", "Help!"]);
        assert_eq!(p.len(), 2);
    }

    #[test]
    fn mutable_loop_changes_in_place() {
        let mut p = sample();
        shout(&mut p);
        let titles: Vec<&String> = (&p).into_iter().collect();
        assert_eq!(titles, vec!["YESTERDAY", "HELP!"]);
    }

    #[test]
    fn consuming_loop_and_sorting() {
        let mut p = sample();
        shout(&mut p);
        assert_eq!(into_sorted(p), vec!["HELP!", "YESTERDAY"]);
        let mut owned = Vec::new();
        for title in sample() {
            owned.push(title);
        }
        assert_eq!(owned, vec!["Yesterday", "Help!"]);
    }

    #[test]
    fn collect_into_a_playlist() {
        let q: Playlist = vec!["b".to_string(), "a".to_string()].into_iter().collect();
        assert_eq!(q.len(), 2);
        assert_eq!(into_sorted(q), vec!["a", "b"]);
        let r: Playlist = (1..=3).map(|i| format!("Track {i}")).collect();
        assert_eq!(total_chars(&r), 21);
    }
}
