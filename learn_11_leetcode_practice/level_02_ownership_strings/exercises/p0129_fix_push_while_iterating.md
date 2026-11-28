# Problem 0129: Fix: Pushing While Iterating

- **Difficulty:** Easy
- **Topics:** fix the borrow checker, error E0502, shared vs mutable borrows, `collect` then `extend`
- **Source:** Original · [E0502](https://doc.rust-lang.org/error_codes/E0502.html) · [The Rust Book 4.2](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

## Problem

This function should append a copy of every even number to the end of the vector, but it does
not compile:

```text
pub fn append_evens(values: &mut Vec<i32>) {
    for value in values.iter() {
        if value % 2 == 0 {
            values.push(*value);
        }
    }
}

error[E0502]: cannot borrow `*values` as mutable because it is also borrowed as immutable
  |     for value in values.iter() {
  |                  -------------
  |                  immutable borrow occurs here
  |                  immutable borrow later used here
  |         if value % 2 == 0 {
  |             values.push(*value);
  |             ^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
```

The rule "many readers **or** one writer" is what protects you here: `push` may reallocate the
vector and leave the iterator pointing at freed memory, and it would also keep finding the new
evens forever.

Implement the corrected `append_evens(values: &mut Vec<i32>)`. After the call, `values` contains
its original elements followed by a copy of each **original** even element, in their original
order. Negative even numbers count as even.

## Examples

```text
Input:  values = [1, 2, 3, 4]
Output: values = [1, 2, 3, 4, 2, 4]

Input:  values = [2, 2]
Output: values = [2, 2, 2, 2]

Input:  values = [1, 3, -5]
Output: values = [1, 3, -5]
```

## Constraints

- `0 <= values.len() <= 10^4`

## Hints

1. Finish reading before you start writing: first collect the evens into a new `Vec`, then add
   them with `values.extend(...)`.
2. Alternatively, loop over the indices `0..len` with `len` computed **before** the loop; indexing
   copies the `i32` out, so no borrow is held across `push`.

## Starter code

Write your code in [`src/practice/p0129_fix_push_while_iterating.rs`](../src/practice/p0129_fix_push_while_iterating.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0129
```

```rust
pub fn append_evens(values: &mut Vec<i32>) {
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

    fn run(mut values: Vec<i32>) -> Vec<i32> {
        append_evens(&mut values);
        values
    }

    #[test]
    fn example_mixed() {
        assert_eq!(run(vec![1, 2, 3, 4]), vec![1, 2, 3, 4, 2, 4]);
    }

    #[test]
    fn example_all_even() {
        assert_eq!(run(vec![2, 2]), vec![2, 2, 2, 2]);
    }

    #[test]
    fn example_no_even() {
        assert_eq!(run(vec![1, 3, -5]), vec![1, 3, -5]);
    }

    #[test]
    fn negative_evens_and_zero() {
        assert_eq!(run(vec![-4, 0, 7]), vec![-4, 0, 7, -4, 0]);
    }

    #[test]
    fn empty_vector() {
        assert_eq!(run(vec![]), Vec::<i32>::new());
    }

    #[test]
    fn large_input() {
        let result = run((0..10_000).collect());
        assert_eq!(result.len(), 15_000);
        assert_eq!(result[10_000], 0);
        assert_eq!(result[14_999], 9_998);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** split the work into a read phase and a write phase. The shared borrow used to collect
the evens ends before the mutable borrow needed by `extend` begins, so the borrow checker is happy.

**Complexity:** `O(n)` time, `O(e)` extra space for the `e` even values.

**Rust concepts:**
- A `&mut Vec` cannot be used for writing while any shared borrow of it (such as an iterator)
  is still alive; non-lexical lifetimes end a borrow at its last use.
- `iter().copied().filter(..).collect::<Vec<i32>>()` produces an independent, owned vector.
- `Vec::extend` consumes any iterator (here the temporary vector) and appends its items.

```rust
pub fn append_evens(values: &mut Vec<i32>) {
    // Read phase: the shared borrow ends when `collect` returns.
    let evens: Vec<i32> = values.iter().copied().filter(|v| v % 2 == 0).collect();
    // Write phase: now a mutable borrow is allowed.
    values.extend(evens);
}
```

Full tested solution: [`src/solutions/p0129_fix_push_while_iterating.rs`](../src/solutions/p0129_fix_push_while_iterating.rs)

</details>
