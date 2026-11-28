# Problem 0444: Combine Predicates

- **Difficulty:** Medium
- **Topics:** closures that take and return closures, `impl Fn(&T) -> bool`, `Box<dyn Fn(&T) -> bool>`, `Iterator::all`, generic `filter`
- **Source:** Original · [The Rust Book 13.1](https://doc.rust-lang.org/book/ch13-01-closures.html) · [std `Fn`](https://doc.rust-lang.org/std/ops/trait.Fn.html)

## Problem

A *predicate* is a function that answers yes or no about a value: `|n: &i32| *n > 0`. Small
predicates can be combined into bigger ones with *combinators*, functions that take predicates
and return a new predicate.

Implement, for any element type `T`:

- `not<T>(p: impl Fn(&T) -> bool) -> impl Fn(&T) -> bool`: true when `p` is false.
- `and<T>(p: impl Fn(&T) -> bool, q: impl Fn(&T) -> bool) -> impl Fn(&T) -> bool`: both are true.
  `q` must not be evaluated when `p` is already false.
- `or<T>(p, q) -> impl Fn(&T) -> bool`: at least one is true (`q` not evaluated when `p` is true).
- `all_of<T>(ps: Vec<Predicate<T>>) -> impl Fn(&T) -> bool`: every predicate is true (an empty
  list accepts everything). `Predicate<T>` is a type alias (given) for `Box<dyn Fn(&T) -> bool>`.
- `select<T: Clone>(items: &[T], p: impl Fn(&T) -> bool) -> Vec<T>`: the items accepted by `p`,
  in order.

## Examples

```text
let positive = |n: &i32| *n > 0;
let even = |n: &i32| n % 2 == 0;
select(&[-2, -1, 0, 1, 2, 3, 4], and(positive, even))        -> [2, 4]
select(&[-2, -1, 0, 1, 2, 3, 4], or(not(positive), even))    -> [-2, -1, 0, 2, 4]
let short_word = all_of(vec![Box::new(|s: &&str| s.len() <= 4), Box::new(|s: &&str| !s.is_empty())]);
select(&["", "tree", "forest", "ox"], short_word)             -> ["tree", "ox"]
```

## Constraints

- No special constraints.

## Hints

1. `move |x| !p(x)`: the new closure owns `p`.
2. `&&` and `||` already short-circuit: `move |x| p(x) && q(x)`.
3. `move |x| ps.iter().all(|p| p(x))`.

## Starter code

Write your code in [`src/practice/p0444_predicate_combinators.rs`](../src/practice/p0444_predicate_combinators.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0444
```

```rust
/// A boxed predicate, so that predicates of different closure types fit in one `Vec`.
pub type Predicate<T> = Box<dyn Fn(&T) -> bool>;

pub fn not<T>(p: impl Fn(&T) -> bool) -> impl Fn(&T) -> bool {
    move |_| todo!()
}

pub fn and<T>(p: impl Fn(&T) -> bool, q: impl Fn(&T) -> bool) -> impl Fn(&T) -> bool {
    move |_| todo!()
}

pub fn or<T>(p: impl Fn(&T) -> bool, q: impl Fn(&T) -> bool) -> impl Fn(&T) -> bool {
    move |_| todo!()
}

pub fn all_of<T>(ps: Vec<Predicate<T>>) -> impl Fn(&T) -> bool {
    move |_| todo!()
}

pub fn select<T: Clone>(items: &[T], p: impl Fn(&T) -> bool) -> Vec<T> {
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

    const NUMS: [i32; 7] = [-2, -1, 0, 1, 2, 3, 4];

    fn positive(n: &i32) -> bool {
        *n > 0
    }

    fn even(n: &i32) -> bool {
        n % 2 == 0
    }

    #[test]
    fn select_with_simple_predicates() {
        assert_eq!(select(&NUMS, positive), vec![1, 2, 3, 4]);
        assert_eq!(select(&NUMS, |n: &i32| *n == 0), vec![0]);
        assert_eq!(select(&NUMS, not(positive)), vec![-2, -1, 0]);
    }

    #[test]
    fn and_or_combinations() {
        assert_eq!(select(&NUMS, and(positive, even)), vec![2, 4]);
        assert_eq!(
            select(&NUMS, or(not(positive), even)),
            vec![-2, -1, 0, 2, 4]
        );
        assert_eq!(select(&NUMS, not(or(positive, even))), vec![-1]);
    }

    #[test]
    fn and_or_short_circuit() {
        let panicking = |_: &i32| -> bool { panic!("must not be evaluated") };
        assert!(!and(|_: &i32| false, panicking)(&1));
        assert!(or(|_: &i32| true, panicking)(&1));
    }

    #[test]
    fn all_of_a_list() {
        let short_word = all_of(vec![
            Box::new(|s: &&str| s.len() <= 4),
            Box::new(|s: &&str| !s.is_empty()),
        ]);
        assert_eq!(
            select(&["", "tree", "forest", "ox"], short_word),
            vec!["tree", "ox"]
        );
        let anything = all_of::<i32>(vec![]);
        assert_eq!(select(&NUMS, anything), NUMS.to_vec());
    }

    #[test]
    fn predicates_on_strings() {
        let words = vec![
            "Apple".to_string(),
            "kiwi".to_string(),
            "Avocado".to_string(),
        ];
        let starts_with_a = |s: &String| s.starts_with('A');
        let long = |s: &String| s.len() > 5;
        assert_eq!(
            select(&words, and(starts_with_a, long)),
            vec!["Avocado".to_string()]
        );
        assert_eq!(select(&words, not(starts_with_a)), vec!["kiwi".to_string()]);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** each combinator returns a `move` closure that owns its input predicates and calls them.
Because the results are predicates too, they can be fed back into the combinators to build any
boolean expression.

**Rust concepts:**
- `impl Fn(&T) -> bool` is a *higher-ranked* bound: the predicate must work for a reference with
  any lifetime. Closures written in the combinators get that signature from the return type.
- `move` transfers the captured predicates into the new closure, so the result does not borrow
  anything local.
- `Box<dyn Fn(&T) -> bool>` lets a `Vec` hold predicates of different closure types; the alias
  `Predicate<T>` keeps signatures readable (Clippy's `type_complexity` lint asks for it).
- `Iterator::all` short-circuits at the first `false` and returns `true` for an empty iterator.

```rust
/// A boxed predicate, so that predicates of different closure types fit in one `Vec`.
pub type Predicate<T> = Box<dyn Fn(&T) -> bool>;

pub fn not<T>(p: impl Fn(&T) -> bool) -> impl Fn(&T) -> bool {
    move |x| !p(x)
}

pub fn and<T>(p: impl Fn(&T) -> bool, q: impl Fn(&T) -> bool) -> impl Fn(&T) -> bool {
    move |x| p(x) && q(x)
}

pub fn or<T>(p: impl Fn(&T) -> bool, q: impl Fn(&T) -> bool) -> impl Fn(&T) -> bool {
    move |x| p(x) || q(x)
}

pub fn all_of<T>(ps: Vec<Predicate<T>>) -> impl Fn(&T) -> bool {
    move |x| ps.iter().all(|p| p(x))
}

pub fn select<T: Clone>(items: &[T], p: impl Fn(&T) -> bool) -> Vec<T> {
    items.iter().filter(|x| p(x)).cloned().collect()
}
```

Full tested solution: [`src/solutions/p0444_predicate_combinators.rs`](../src/solutions/p0444_predicate_combinators.rs)

</details>
