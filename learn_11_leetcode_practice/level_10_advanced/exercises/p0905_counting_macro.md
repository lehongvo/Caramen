# Problem 0905: Counting Tokens with `count!`

- **Difficulty:** Medium
- **Topics:** `macro_rules!`, `tt` fragments, helper macros, recursion limit, constant expressions
- **Source:** Original · [Reference: Macros By Example](https://doc.rust-lang.org/reference/macros-by-example.html) · [Rust by Example: Repeat](https://doc.rust-lang.org/rust-by-example/macros/repeat.html)

## Problem

Macros often need to know **how many** things they were given (to size an array, to pre-allocate
a collection, ...). Write two counting macros:

- `count_tts!(...)` expands to a `usize` equal to the number of **token trees** in its input.
  A token tree is a single token (`a`, `42`, `+`, `"s"`) or a whole bracketed group (`(..)`,
  `[..]`, `{..}`), which counts as **one**.
- `count_exprs!(e1, e2, ...)` expands to the number of comma-separated **expressions** (a trailing
  comma is allowed; `count_exprs!()` is `0`). The expressions must **not be evaluated**: only
  counted.

Both expansions must be constant expressions (usable as `const N: usize = count_tts!(a b c);`)
and must work for **hundreds** of tokens. The naive recursive version
`($head:tt $($tail:tt)*) => { 1 + count_tts!($($tail)*) }` hits the default macro recursion limit
(128) long before that.

Finally write `fn fixed_tags() -> [&'static str; 4]` that builds `["red", "green", "blue", "cyan"]`
with an array whose length is written as `count_tts!(red green blue cyan)`.

## Examples

```text
count_tts!()                    => 0
count_tts!(a b c)               => 3
count_tts!(a (b c) [d e f] 1+2) => 6     (a, (b c), [d e f], 1, +, 2)
count_exprs!(1 + 2, foo(), "x") => 3     and foo() is never called
count_exprs!(x,)                => 1
```

## Constraints

- Inputs of up to a few hundred token trees must compile.

## Hints

1. Replace every token tree by `()` and take the length of the resulting array of units:
   `<[()]>::len(&[(), (), ()])` is `3`, and `<[()]>::len` is a `const fn`.
2. A tiny helper macro `unit!($t:tt) => { () }` does the replacement.
3. A captured `$e:expr` can be passed on to another macro as a single token tree.

## Starter code

Write your code in [`src/practice/p0905_counting_macro.rs`](../src/practice/p0905_counting_macro.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0905
```

```rust
/// Counts the token trees in its input.
macro_rules! count_tts {
    ($($t:tt)*) => {
        todo!()
    };
}

/// Counts comma-separated expressions without evaluating them.
// Only invoked by the tests below (in a real crate it would be `#[macro_export]`ed).
#[allow(unused_macros)]
macro_rules! count_exprs {
    ($($t:tt)*) => {
        todo!()
    };
}

/// `["red", "green", "blue", "cyan"]`, with the array length computed by `count_tts!`.
pub fn fixed_tags() -> [&'static str; 4] {
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
    use std::cell::Cell;

    #[test]
    fn counts_simple_token_trees() {
        let zero: usize = count_tts!();
        let three: usize = count_tts!(a b c);
        let groups: usize = count_tts!(a (b c) [d e f] 1+2);
        assert_eq!((zero, three, groups), (0, 3, 6));
    }

    #[test]
    fn counts_expressions_without_evaluating_them() {
        let calls = Cell::new(0);
        let foo = || {
            calls.set(calls.get() + 1);
            7
        };
        let n: usize = count_exprs!(1 + 2, foo(), "x");
        let trailing: usize = count_exprs!(foo(),);
        let none: usize = count_exprs!();
        assert_eq!((n, trailing, none), (3, 1, 0));
        assert_eq!(
            calls.get(),
            0,
            "count_exprs! must not evaluate its arguments"
        );
        assert_eq!(foo(), 7);
        assert_eq!(calls.get(), 1);
    }

    #[test]
    fn fixed_tags_array() {
        assert_eq!(fixed_tags(), ["red", "green", "blue", "cyan"]);
    }

    #[test]
    fn hundreds_of_tokens_do_not_hit_the_recursion_limit() {
        let n: usize = count_tts!(
            x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x
            x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x
            x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x
            x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x
            x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x x
        );
        assert_eq!(n, 200);
        let exprs: usize = count_exprs!(
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
            47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68,
            69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90,
            91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109,
            110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126,
            127, 128, 129, 130,
        );
        assert_eq!(exprs, 130);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** map each counted item to the unit value `()` with a helper macro, collect the units in a
slice literal and ask for its length. The expansion is one flat array, so there is no recursion
at all, and the expressions inside `count_exprs!` disappear: they are replaced by `()` before the
compiler ever type-checks them.

**Rust concepts:**
- `$($t:tt)*` matches any sequence of token trees; `$( unit!($t) ),*` re-emits one `()` per item.
- `<[()]>::len(&[...])` calls the slice method `len` on an array coerced to a slice; it is a
  `const fn`, so the result can initialise a `const` or an array length.
- The default `#![recursion_limit]` is 128: deeply recursive macros need either a larger limit or
  a non-recursive formulation like this one.
- A fragment captured as `$e:expr` becomes an **opaque** token tree: another macro can still
  match it with `$t:tt`, but no longer look inside it.

```rust
/// Turns any single token tree into `()`.
macro_rules! unit {
    ($t:tt) => {
        ()
    };
}

/// Counts the token trees in its input.
macro_rules! count_tts {
    ($($t:tt)*) => {
        <[()]>::len(&[$(unit!($t)),*])
    };
}

/// Counts comma-separated expressions without evaluating them.
// Only invoked by the tests below (in a real crate it would be `#[macro_export]`ed).
#[allow(unused_macros)]
macro_rules! count_exprs {
    ($($e:expr),* $(,)?) => {
        <[()]>::len(&[$(unit!($e)),*])
    };
}

/// `["red", "green", "blue", "cyan"]`, with the array length computed by `count_tts!`.
pub fn fixed_tags() -> [&'static str; 4] {
    const N: usize = count_tts!(red green blue cyan);
    let tags: [&'static str; N] = ["red", "green", "blue", "cyan"];
    tags
}
```

Full tested solution: [`src/solutions/p0905_counting_macro.rs`](../src/solutions/p0905_counting_macro.rs)

</details>
