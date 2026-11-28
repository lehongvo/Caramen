// ==================================================================
// PRACTICE 0933: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0933_ptr_copy_insert_remove.md
// Run the tests:          cargo test practice::p0933
// ==================================================================

//! # Problem 0933: Vec Insert and Remove with `std::ptr`
//!
//! - **Difficulty:** Hard
//! - **Topics:** `ptr::copy` (memmove), `ptr::copy_nonoverlapping`, `ptr::read` / `ptr::write`, `Vec::set_len`, `reserve`, moving non-`Copy` values without double drops
//! - **Source:** Original · [The Rustonomicon: Insert and Remove](https://doc.rust-lang.org/nomicon/vec/vec-insert-remove.html) · [std: `ptr::copy`](https://doc.rust-lang.org/std/ptr/fn.copy.html)
//!
//! ## Problem
//!
//! `Vec::insert` and `Vec::remove` shift the tail of the vector by one slot with a single memory
//! move. Re-implement them (and two relatives) with raw pointers, for any `T`, including types that
//! own heap memory such as `String`: every value must end up owned by **exactly one** slot, never
//! duplicated (double free) and never forgotten (leak).
//!
//! Do not call `Vec::insert`, `Vec::remove`, `Vec::swap_remove`, `slice::rotate_right` or similar
//! helpers; use `as_mut_ptr`, `reserve`, `set_len` and the functions of `std::ptr`.
//!
//! - `insert_at<T>(v: &mut Vec<T>, index: usize, value: T)`: `index <= len`, otherwise panic with
//!   `"insertion index out of bounds"`. Elements from `index` on move one slot to the right.
//! - `remove_at<T>(v: &mut Vec<T>, index: usize) -> T`: `index < len`, otherwise panic with
//!   `"removal index out of bounds"`. Elements after `index` move one slot to the left.
//! - `swap_remove_at<T>(v: &mut Vec<T>, index: usize) -> T`: removes element `index` and moves the
//!   **last** element into its place (`O(1)`); same panic message as `remove_at`.
//! - `move_to_front<T>(items: &mut [T], index: usize)`: moves element `index` to position 0, shifting
//!   the elements before it one slot right; panic with `"index out of bounds"` if `index >= len`.
//!
//! ## Examples
//!
//! ```text
//! let mut v = vec!["a", "b", "c"];
//! insert_at(&mut v, 1, "x");        // ["a", "x", "b", "c"]
//! remove_at(&mut v, 0)              => "a"   v == ["x", "b", "c"]
//! swap_remove_at(&mut v, 0)         => "x"   v == ["c", "b"]
//! let mut w = [1, 2, 3, 4];
//! move_to_front(&mut w, 2);         // [3, 1, 2, 4]
//! ```
//!
//! ## Constraints
//!
//! - `T` is not a zero-sized type.
//!
//! ## Hints
//!
//! 1. `ptr::copy(src, dst, count)` handles overlapping ranges; `copy_nonoverlapping` does not.
//! 2. `ptr::read` moves a value **out** of a slot (bitwise copy) and `ptr::write` moves one **in**
//!    without dropping the old bits. After `read`, treat the slot as uninitialised.
//! 3. Change the length with `set_len` so that the vector never considers an uninitialised slot to be
//!    part of it when you are done.

use std::ptr;

pub fn insert_at<T>(v: &mut Vec<T>, index: usize, value: T) {
    todo!("Write your solution here")
}

pub fn remove_at<T>(v: &mut Vec<T>, index: usize) -> T {
    todo!("Write your solution here")
}

pub fn swap_remove_at<T>(v: &mut Vec<T>, index: usize) -> T {
    todo!("Write your solution here")
}

pub fn move_to_front<T>(items: &mut [T], index: usize) {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    #[test]
    fn insert_and_remove_strs() {
        let mut v = vec!["a", "b", "c"];
        insert_at(&mut v, 1, "x");
        assert_eq!(v, ["a", "x", "b", "c"]);
        insert_at(&mut v, 4, "end");
        insert_at(&mut v, 0, "start");
        assert_eq!(v, ["start", "a", "x", "b", "c", "end"]);
        assert_eq!(remove_at(&mut v, 0), "start");
        assert_eq!(remove_at(&mut v, 4), "end");
        assert_eq!(remove_at(&mut v, 0), "a");
        assert_eq!(v, ["x", "b", "c"]);
        assert_eq!(swap_remove_at(&mut v, 0), "x");
        assert_eq!(v, ["c", "b"]);
        assert_eq!(swap_remove_at(&mut v, 1), "b");
        assert_eq!(v, ["c"]);
    }

    #[test]
    fn owned_strings_are_moved_not_copied() {
        let mut v: Vec<String> = Vec::new();
        for word in ["one", "two", "three"] {
            insert_at(&mut v, 0, word.to_string());
        }
        assert_eq!(v, ["three", "two", "one"]);
        let removed = remove_at(&mut v, 1);
        assert_eq!(removed, "two");
        drop(removed);
        assert_eq!(v, ["three", "one"]);
        v.push("four".to_string());
        assert_eq!(swap_remove_at(&mut v, 0), "three");
        assert_eq!(v, ["four", "one"]);
    }

    #[test]
    fn no_value_is_leaked_or_duplicated() {
        let token = Rc::new(());
        let mut v: Vec<Rc<()>> = (0..5).map(|_| Rc::clone(&token)).collect();
        assert_eq!(Rc::strong_count(&token), 6);
        insert_at(&mut v, 2, Rc::clone(&token));
        assert_eq!(Rc::strong_count(&token), 7);
        drop(remove_at(&mut v, 0));
        drop(swap_remove_at(&mut v, 1));
        assert_eq!(Rc::strong_count(&token), 5);
        move_to_front(&mut v, 3);
        assert_eq!(v.len(), 4);
        drop(v);
        assert_eq!(Rc::strong_count(&token), 1);
    }

    #[test]
    fn move_to_front_rotates_a_prefix() {
        let mut w = [1, 2, 3, 4];
        move_to_front(&mut w, 2);
        assert_eq!(w, [3, 1, 2, 4]);
        move_to_front(&mut w, 0);
        assert_eq!(w, [3, 1, 2, 4]);
        move_to_front(&mut w, 3);
        assert_eq!(w, [4, 3, 1, 2]);
    }

    #[test]
    #[should_panic(expected = "insertion index out of bounds")]
    fn insert_past_the_end_panics() {
        let mut v = vec![1];
        insert_at(&mut v, 2, 0);
    }

    #[test]
    #[should_panic(expected = "removal index out of bounds")]
    fn remove_from_empty_panics() {
        let mut v: Vec<u8> = Vec::new();
        remove_at(&mut v, 0);
    }

    #[test]
    fn many_operations_match_std() {
        let mut mine: Vec<u32> = Vec::new();
        let mut expected: Vec<u32> = Vec::new();
        for i in 0..2_000_u32 {
            let index = (i as usize * 7) % (mine.len() + 1);
            insert_at(&mut mine, index, i);
            expected.insert(index, i);
        }
        for i in 0..1_000 {
            let index = (i * 13) % mine.len();
            assert_eq!(remove_at(&mut mine, index), expected.remove(index));
        }
        assert_eq!(mine, expected);
    }
}
