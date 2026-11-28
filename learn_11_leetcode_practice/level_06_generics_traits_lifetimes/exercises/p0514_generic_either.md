# Problem 0514: A Generic Either Enum

- **Difficulty:** Easy
- **Topics:** generic enums, `impl<L, R>`, consuming methods, `Option`, mapping with closures
- **Source:** Original · [The Rust Book 10.1](https://doc.rust-lang.org/book/ch10-01-syntax.html#in-enum-definitions)

## Problem

`Option<T>` and `Result<T, E>` are generic enums. Build your own two-variant enum:

```text
pub enum Either<L, R> { Left(L), Right(R) }
```

Derive `Debug`, `Clone` and `PartialEq`, then implement in `impl<L, R> Either<L, R>`:

- `is_left(&self) -> bool` and `is_right(&self) -> bool`
- `left(self) -> Option<L>` and `right(self) -> Option<R>`: the value if it is on that side.
- `flip(self) -> Either<R, L>`: `Left(a)` becomes `Right(a)` and vice versa.
- `map_left<L2, F: FnOnce(L) -> L2>(self, f: F) -> Either<L2, R>`: transforms a `Left` value and
  leaves a `Right` untouched.
- `as_ref(&self) -> Either<&L, &R>`: borrows the inside without moving it.

Also write two free functions:

- `partition<L, R>(items: Vec<Either<L, R>>) -> (Vec<L>, Vec<R>)` splits the items, keeping
  the order on each side.
- `parse_tokens(tokens: &[&str]) -> Vec<Either<i64, String>>`: a token that parses as an `i64`
  becomes `Left(number)`, any other token becomes `Right(token.to_string())`.

## Examples

```text
Either::<i32, char>::Left(5).flip()           => Right(5)
Either::<i32, char>::Right('x').left()        => None
Either::<i32, char>::Left(5).map_left(|n| n > 3) => Left(true)
parse_tokens(&["12", "apples", "-3"])         => [Left(12), Right("apples"), Left(-3)]
partition(parse_tokens(&["12", "apples", "-3"])) => ([12, -3], ["apples"])
```

## Constraints

- `0 <= tokens.len() <= 10^4`

## Hints

1. Every method is a `match self { Either::Left(a) => ..., Either::Right(b) => ... }`.
2. In `as_ref`, match on `self` (a reference): the bindings become references automatically.

## Starter code

Write your code in [`src/practice/p0514_generic_either.rs`](../src/practice/p0514_generic_either.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0514
```

```rust
/// A value that is either a `Left(L)` or a `Right(R)`.
#[derive(Debug, Clone, PartialEq)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    pub fn is_left(&self) -> bool {
        todo!("Write your solution here")
    }

    pub fn is_right(&self) -> bool {
        todo!("Write your solution here")
    }

    pub fn left(self) -> Option<L> {
        todo!("Write your solution here")
    }

    pub fn right(self) -> Option<R> {
        todo!("Write your solution here")
    }

    pub fn flip(self) -> Either<R, L> {
        todo!("Write your solution here")
    }

    pub fn map_left<L2, F: FnOnce(L) -> L2>(self, f: F) -> Either<L2, R> {
        todo!("Write your solution here")
    }

    pub fn as_ref(&self) -> Either<&L, &R> {
        todo!("Write your solution here")
    }
}

/// Splits the items into the left values and the right values, keeping their order.
pub fn partition<L, R>(items: Vec<Either<L, R>>) -> (Vec<L>, Vec<R>) {
    todo!("Write your solution here")
}

/// Numbers become `Left`, everything else becomes `Right`.
pub fn parse_tokens(tokens: &[&str]) -> Vec<Either<i64, String>> {
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

    #[test]
    fn predicates_and_extractors() {
        let l: Either<i32, char> = Either::Left(5);
        let r: Either<i32, char> = Either::Right('x');
        assert!(l.is_left() && !l.is_right());
        assert!(r.is_right() && !r.is_left());
        assert_eq!(l.clone().left(), Some(5));
        assert_eq!(l.right(), None);
        assert_eq!(r.clone().left(), None);
        assert_eq!(r.right(), Some('x'));
    }

    #[test]
    fn flip_map_and_as_ref() {
        assert_eq!(Either::<i32, char>::Left(5).flip(), Either::Right(5));
        assert_eq!(Either::<i32, char>::Right('q').flip(), Either::Left('q'));
        assert_eq!(
            Either::<i32, char>::Left(5).map_left(|n| n > 3),
            Either::Left(true)
        );
        assert_eq!(
            Either::<i32, char>::Right('c').map_left(|n| n + 1),
            Either::Right('c')
        );
        let owned: Either<String, u8> = Either::Left("text".to_string());
        assert_eq!(owned.as_ref(), Either::Left(&"text".to_string()));
        assert!(owned.is_left()); // still usable: as_ref only borrowed it
    }

    #[test]
    fn parse_and_partition() {
        let parsed = parse_tokens(&["12", "apples", "-3"]);
        assert_eq!(
            parsed,
            vec![
                Either::Left(12),
                Either::Right("apples".to_string()),
                Either::Left(-3)
            ]
        );
        let (nums, words) = partition(parsed);
        assert_eq!(nums, vec![12, -3]);
        assert_eq!(words, vec!["apples"]);
    }

    #[test]
    fn empty_and_large() {
        let (l, r) = partition(Vec::<Either<u8, u8>>::new());
        assert!(l.is_empty() && r.is_empty());
        let tokens: Vec<String> = (0..10_000)
            .map(|i| {
                if i % 2 == 0 {
                    i.to_string()
                } else {
                    format!("w{i}")
                }
            })
            .collect();
        let refs: Vec<&str> = tokens.iter().map(String::as_str).collect();
        let (nums, words) = partition(parse_tokens(&refs));
        assert_eq!(nums.len(), 5_000);
        assert_eq!(words.len(), 5_000);
        assert_eq!(nums[4_999], 9_998);
        assert_eq!(words[0], "w1");
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** every operation is a two-arm `match`. `partition` pushes each value into the vector of
its side; `parse_tokens` uses `str::parse::<i64>()` and picks the variant from the `Result`.

**Rust concepts:**
- A generic enum stores one of several differently typed payloads; `Either<L, R>` is as large as
  its bigger variant plus a tag.
- Methods that take `self` by value consume the enum and can move the payload out; `as_ref` takes
  `&self` and returns references, the same pattern as `Option::as_ref`.
- `flip` returns `Either<R, L>`: the return type swaps the type parameters, and the compiler checks
  that each arm builds the right variant.

```rust
/// A value that is either a `Left(L)` or a `Right(R)`.
#[derive(Debug, Clone, PartialEq)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    pub fn is_left(&self) -> bool {
        matches!(self, Either::Left(_))
    }

    pub fn is_right(&self) -> bool {
        !self.is_left()
    }

    pub fn left(self) -> Option<L> {
        match self {
            Either::Left(l) => Some(l),
            Either::Right(_) => None,
        }
    }

    pub fn right(self) -> Option<R> {
        match self {
            Either::Left(_) => None,
            Either::Right(r) => Some(r),
        }
    }

    pub fn flip(self) -> Either<R, L> {
        match self {
            Either::Left(l) => Either::Right(l),
            Either::Right(r) => Either::Left(r),
        }
    }

    pub fn map_left<L2, F: FnOnce(L) -> L2>(self, f: F) -> Either<L2, R> {
        match self {
            Either::Left(l) => Either::Left(f(l)),
            Either::Right(r) => Either::Right(r),
        }
    }

    pub fn as_ref(&self) -> Either<&L, &R> {
        match self {
            Either::Left(l) => Either::Left(l),
            Either::Right(r) => Either::Right(r),
        }
    }
}

/// Splits the items into the left values and the right values, keeping their order.
pub fn partition<L, R>(items: Vec<Either<L, R>>) -> (Vec<L>, Vec<R>) {
    let mut lefts = Vec::new();
    let mut rights = Vec::new();
    for item in items {
        match item {
            Either::Left(l) => lefts.push(l),
            Either::Right(r) => rights.push(r),
        }
    }
    (lefts, rights)
}

/// Numbers become `Left`, everything else becomes `Right`.
pub fn parse_tokens(tokens: &[&str]) -> Vec<Either<i64, String>> {
    tokens
        .iter()
        .map(|t| match t.parse::<i64>() {
            Ok(n) => Either::Left(n),
            Err(_) => Either::Right(t.to_string()),
        })
        .collect()
}
```

Full tested solution: [`src/solutions/p0514_generic_either.rs`](../src/solutions/p0514_generic_either.rs)

</details>
