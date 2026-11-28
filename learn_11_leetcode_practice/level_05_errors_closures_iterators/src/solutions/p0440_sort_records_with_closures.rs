//! # Problem 0440: Sort Records with Closures
//!
//! - **Difficulty:** Easy
//! - **Topics:** `sort_by_key`, `sort_by`, `Ordering::then_with`, `cmp::Reverse`, `f64::total_cmp`, stable sorting
//! - **Source:** Original · [std `slice::sort_by`](https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by) · [std `Ordering::then_with`](https://doc.rust-lang.org/std/cmp/enum.Ordering.html#method.then_with)
//!
//! ## Problem
//!
//! Sorting is one of the most common places to pass a closure: the closure tells the sort **what to
//! compare**. `sort_by_key(|x| key)` sorts by a key extracted from each element; `sort_by(|a, b| ...)`
//! receives two elements and returns an `Ordering`. Both are **stable**: elements that compare equal
//! keep their original relative order.
//!
//! Implement, for a slice of `Book`s:
//!
//! - `by_year(books: &mut [Book])`: oldest first.
//! - `by_rating_desc(books: &mut [Book])`: highest rating first (ratings are `f64`, never NaN in the
//!   tests); equal ratings keep their order.
//! - `by_author_then_title(books: &mut [Book])`: by author (alphabetical, byte order), and for the
//!   same author by title.
//! - `newest_titles(books: &[Book], n: usize) -> Vec<String>`: the titles of the `n` newest books
//!   (largest `year` first; equal years keep their input order). The input is **not** modified.
//!
//! ## Examples
//!
//! ```text
//! books = [("Dune", "Herbert", 1965, 4.3), ("Emma", "Austen", 1815, 4.0),
//!          ("Persuasion", "Austen", 1817, 4.1), ("Neuromancer", "Gibson", 1984, 3.9)]
//! by_year              -> Emma, Persuasion, Dune, Neuromancer
//! by_rating_desc       -> Dune, Persuasion, Emma, Neuromancer
//! by_author_then_title -> Emma, Persuasion, Neuromancer, Dune
//! newest_titles(2)     -> ["Neuromancer", "Dune"]
//! ```
//!
//! ## Constraints
//!
//! - At most `10^4` books.
//!
//! ## Hints
//!
//! 1. `books.sort_by_key(|b| b.year)` sorts by an integer key.
//! 2. `f64` is not `Ord` (because of NaN), so use `sort_by(|a, b| b.rating.total_cmp(&a.rating))`;
//!    swapping `a` and `b` reverses the order.
//! 3. `a.author.cmp(&b.author).then_with(|| a.title.cmp(&b.title))`; for "largest first" with a key,
//!    wrap the key in `std::cmp::Reverse`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** every function is a single call to a sort method with the right closure. The closures
//! extract keys or combine comparisons; stability guarantees the tie-breaking rules.
//!
//! **Rust concepts:**
//! - `sort_by_key` calls the key closure on demand; returning a borrowed key such as `&b.title` is
//!   not allowed with it (the key must not borrow from the element), so string comparisons use
//!   `sort_by`.
//! - `Ordering::then_with(|| ...)` evaluates the second comparison only when the first is `Equal`.
//! - `Reverse(key)` flips the order of any `Ord` key without writing a custom comparator.
//! - `total_cmp` gives a total order on `f64`, which `sort_by` needs.
//! - `newest_titles` sorts references (`Vec<&Book>`), so it never clones a whole `Book`.
// @solution-end

use std::cmp::Reverse;

#[derive(Debug, Clone, PartialEq)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub year: u16,
    pub rating: f64,
}

pub fn by_year(books: &mut [Book]) {
    // @solution-start
    books.sort_by_key(|b| b.year);
    // @solution-end
}

pub fn by_rating_desc(books: &mut [Book]) {
    // @solution-start
    books.sort_by(|a, b| b.rating.total_cmp(&a.rating));
    // @solution-end
}

pub fn by_author_then_title(books: &mut [Book]) {
    // @solution-start
    books.sort_by(|a, b| a.author.cmp(&b.author).then_with(|| a.title.cmp(&b.title)));
    // @solution-end
}

pub fn newest_titles(books: &[Book], n: usize) -> Vec<String> {
    // @solution-start
    let mut refs: Vec<&Book> = books.iter().collect();
    refs.sort_by_key(|b| Reverse(b.year));
    refs.into_iter().take(n).map(|b| b.title.clone()).collect()
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    fn book(title: &str, author: &str, year: u16, rating: f64) -> Book {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            year,
            rating,
        }
    }

    fn sample() -> Vec<Book> {
        vec![
            book("Dune", "Herbert", 1965, 4.3),
            book("Emma", "Austen", 1815, 4.0),
            book("Persuasion", "Austen", 1817, 4.1),
            book("Neuromancer", "Gibson", 1984, 3.9),
        ]
    }

    fn titles(books: &[Book]) -> Vec<&str> {
        books.iter().map(|b| b.title.as_str()).collect()
    }

    #[test]
    fn sort_by_year() {
        let mut books = sample();
        by_year(&mut books);
        assert_eq!(
            titles(&books),
            vec!["Emma", "Persuasion", "Dune", "Neuromancer"]
        );
    }

    #[test]
    fn sort_by_rating_descending_is_stable() {
        let mut books = sample();
        by_rating_desc(&mut books);
        assert_eq!(
            titles(&books),
            vec!["Dune", "Persuasion", "Emma", "Neuromancer"]
        );
        let mut ties = vec![
            book("A", "x", 1, 2.0),
            book("B", "x", 1, 5.0),
            book("C", "x", 1, 2.0),
        ];
        by_rating_desc(&mut ties);
        assert_eq!(titles(&ties), vec!["B", "A", "C"]);
    }

    #[test]
    fn sort_by_author_then_title() {
        let mut books = sample();
        books.push(book("Emma 2", "Austen", 2000, 1.0));
        books.push(book("Children of Dune", "Herbert", 1976, 4.0));
        by_author_then_title(&mut books);
        assert_eq!(
            titles(&books),
            vec![
                "Emma",
                "Emma 2",
                "Persuasion",
                "Neuromancer",
                "Children of Dune",
                "Dune"
            ]
        );
    }

    #[test]
    fn newest_titles_do_not_modify_the_input() {
        let books = sample();
        assert_eq!(newest_titles(&books, 2), vec!["Neuromancer", "Dune"]);
        assert_eq!(newest_titles(&books, 0), Vec::<String>::new());
        assert_eq!(newest_titles(&books, 10).len(), 4);
        assert_eq!(books, sample());
    }

    #[test]
    fn newest_titles_ties_keep_input_order() {
        let books = vec![
            book("old", "a", 1, 0.0),
            book("x", "a", 9, 0.0),
            book("y", "a", 9, 0.0),
        ];
        assert_eq!(newest_titles(&books, 3), vec!["x", "y", "old"]);
    }
}
