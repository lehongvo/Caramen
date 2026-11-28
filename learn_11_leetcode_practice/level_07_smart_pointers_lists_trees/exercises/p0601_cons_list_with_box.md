# Problem 0601: Cons List with Box

- **Difficulty:** Easy
- **Topics:** `Box<T>`, recursive `enum`, `while let`, `Iterator::rev` + `fold`
- **Source:** Original · [The Rust Book 15.1](https://doc.rust-lang.org/book/ch15-01-box.html)

## Problem

A *cons list* is the classic recursive list from Lisp: a list is either empty (`Nil`) or a
value followed by another list (`Cons(value, rest)`). Writing it in Rust as
`enum List { Cons(i32, List), Nil }` does not compile: the compiler cannot compute the size of
a type that contains itself (error `E0072`, "recursive type has infinite size"). Putting the
rest of the list behind a `Box` fixes this, because a `Box<List>` is just a pointer with a known
size, while the boxed value lives on the heap.

The type is given:

```text
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

Implement these methods:

- `List::from_slice(values: &[i32]) -> List`: build the list holding `values` in the same order
  (`&[]` gives `Nil`);
- `len(&self) -> usize`: the number of `Cons` cells;
- `is_empty(&self) -> bool`: `true` for `Nil`;
- `sum(&self) -> i64`: the sum of all values (as `i64`, so it cannot overflow);
- `to_vec(&self) -> Vec<i32>`: the values from front to back.

## Examples

```text
List::from_slice(&[1, 2, 3])
  == Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))))

let list = List::from_slice(&[4, -1, 7]);
list.len()    == 3
list.sum()    == 10
list.to_vec() == vec![4, -1, 7]

List::from_slice(&[]) == Nil, its len() is 0 and is_empty() is true
```

## Constraints

- `0 <= values.len() <= 10^4`
- `-10^9 <= values[i] <= 10^9`

## Hints

1. Build the list from the **back**: start with `Nil` and wrap it in a new `Cons` cell for each
   value of `values.iter().rev()`.
2. To walk the list without recursion, keep a `current: &List` and loop with
   `while let List::Cons(value, rest) = current { ...; current = rest; }`.
3. `rest` is a `&Box<List>`; the compiler turns it into a `&List` for you (deref coercion), or
   you can write `rest.as_ref()` explicitly.

## Starter code

Write your code in [`src/practice/p0601_cons_list_with_box.rs`](../src/practice/p0601_cons_list_with_box.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0601
```

```rust
#[derive(Debug, PartialEq)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    pub fn from_slice(values: &[i32]) -> List {
        todo!("Write your solution here")
    }

    pub fn len(&self) -> usize {
        todo!("Write your solution here")
    }

    pub fn is_empty(&self) -> bool {
        todo!("Write your solution here")
    }

    pub fn sum(&self) -> i64 {
        todo!("Write your solution here")
    }

    pub fn to_vec(&self) -> Vec<i32> {
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
    fn from_slice_builds_nested_cells() {
        let expected = List::Cons(
            1,
            Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
        );
        assert_eq!(List::from_slice(&[1, 2, 3]), expected);
    }

    #[test]
    fn len_sum_and_to_vec() {
        let list = List::from_slice(&[4, -1, 7]);
        assert_eq!(list.len(), 3);
        assert!(!list.is_empty());
        assert_eq!(list.sum(), 10);
        assert_eq!(list.to_vec(), vec![4, -1, 7]);
    }

    #[test]
    fn empty_list_is_nil() {
        let list = List::from_slice(&[]);
        assert_eq!(list, List::Nil);
        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
        assert_eq!(list.sum(), 0);
        assert!(list.to_vec().is_empty());
    }

    #[test]
    fn large_values_do_not_overflow_the_sum() {
        let values = vec![1_000_000_000; 10_000];
        let list = List::from_slice(&values);
        assert_eq!(list.len(), 10_000);
        assert_eq!(list.sum(), 10_000_000_000_000);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** `from_slice` folds from the right: `Nil` is the empty list and each step puts one more
value in front, `Cons(value, Box::new(tail))`. All other methods walk the cells with a shared
reference until they reach `Nil`.

**Complexity:** every method is `O(n)` time; `from_slice` uses `O(n)` heap space for the cells,
the others `O(1)` extra space (`to_vec` aside, which returns `n` values).

**Rust concepts:**
- `Box<T>` owns a value on the heap. The enum only stores the pointer, so its size is known at
  compile time even though the type is recursive.
- Walking with a loop instead of recursion keeps the stack flat, so a 10 000-cell list is fine.
- `use List::{Cons, Nil};` brings the variants into scope so patterns stay short.
- Clippy asks for `is_empty` whenever a type has a public `len` method (`len_without_is_empty`).

```rust
#[derive(Debug, PartialEq)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

impl List {
    pub fn from_slice(values: &[i32]) -> List {
        values
            .iter()
            .rev()
            .fold(Nil, |tail, &value| Cons(value, Box::new(tail)))
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut current = self;
        while let Cons(_, rest) = current {
            count += 1;
            current = rest.as_ref();
        }
        count
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, Nil)
    }

    pub fn sum(&self) -> i64 {
        let mut total = 0_i64;
        let mut current = self;
        while let Cons(value, rest) = current {
            total += i64::from(*value);
            current = rest.as_ref();
        }
        total
    }

    pub fn to_vec(&self) -> Vec<i32> {
        let mut values = Vec::new();
        let mut current = self;
        while let Cons(value, rest) = current {
            values.push(*value);
            current = rest.as_ref();
        }
        values
    }
}
```

Full tested solution: [`src/solutions/p0601_cons_list_with_box.rs`](../src/solutions/p0601_cons_list_with_box.rs)

</details>
