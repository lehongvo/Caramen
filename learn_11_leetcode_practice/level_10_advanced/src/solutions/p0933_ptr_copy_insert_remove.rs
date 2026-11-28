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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** `insert_at` makes room with `reserve(1)`, moves `[index, len)` one slot right with
//! `ptr::copy`, writes the new value into the gap and increments the length. `remove_at` reads the
//! value out, moves `[index + 1, len)` one slot left and decrements the length. `swap_remove_at`
//! reads the value out and copies the last element over the hole. `move_to_front` reads the element,
//! shifts `[0, index)` right and writes the element at the front.
//!
//! **Complexity:** `insert_at`, `remove_at` and `move_to_front` are `O(n)` (one `memmove`),
//! `swap_remove_at` is `O(1)`.
//!
//! **Rust concepts:**
//! - Between the bitwise moves there are moments when a value exists twice or a slot holds garbage;
//!   no code that could panic or observe the vector may run then.
//! - `set_len` is `unsafe` because it changes which slots the vector will drop and expose.
//! - `ptr::read` on a `String` duplicates its pointer; correctness relies on making exactly one of
//!   the two copies unreachable afterwards.
// @solution-end

use std::ptr;

pub fn insert_at<T>(v: &mut Vec<T>, index: usize, value: T) {
    // @solution-start
    let len = v.len();
    assert!(index <= len, "insertion index out of bounds");
    v.reserve(1);
    // SAFETY: after `reserve(1)` the buffer has room for `len + 1` elements. We move the
    // initialised range `[index, len)` to `[index + 1, len + 1)` (overlapping, so `copy`), then fill
    // the gap at `index` with `write`, which does not drop the stale bits there.
    unsafe {
        let gap = v.as_mut_ptr().add(index);
        ptr::copy(gap, gap.add(1), len - index);
        ptr::write(gap, value);
        v.set_len(len + 1);
    }
    // @solution-end
}

pub fn remove_at<T>(v: &mut Vec<T>, index: usize) -> T {
    // @solution-start
    let len = v.len();
    assert!(index < len, "removal index out of bounds");
    // SAFETY: `index < len`, so the slot is initialised. After reading it out we close the gap by
    // moving `[index + 1, len)` one slot left, leaving `len - 1` initialised elements.
    unsafe {
        let hole = v.as_mut_ptr().add(index);
        let value = ptr::read(hole);
        ptr::copy(hole.add(1), hole, len - index - 1);
        v.set_len(len - 1);
        value
    }
    // @solution-end
}

pub fn swap_remove_at<T>(v: &mut Vec<T>, index: usize) -> T {
    // @solution-start
    let len = v.len();
    assert!(index < len, "removal index out of bounds");
    // SAFETY: both `index` and `len - 1` are initialised slots. We read the removed value, then move
    // the last element into the hole (a no-op copy of zero distance is avoided by the `if`).
    unsafe {
        let base = v.as_mut_ptr();
        let value = ptr::read(base.add(index));
        let last = len - 1;
        if index != last {
            ptr::copy_nonoverlapping(base.add(last), base.add(index), 1);
        }
        v.set_len(last);
        value
    }
    // @solution-end
}

pub fn move_to_front<T>(items: &mut [T], index: usize) {
    // @solution-start
    assert!(index < items.len(), "index out of bounds");
    // SAFETY: `index < len`. We take the element out, shift `[0, index)` to `[1, index + 1)` (all in
    // bounds) and write the element into slot 0; every slot is initialised again at the end.
    unsafe {
        let base = items.as_mut_ptr();
        let value = ptr::read(base.add(index));
        ptr::copy(base, base.add(1), index);
        ptr::write(base, value);
    }
    // @solution-end
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
