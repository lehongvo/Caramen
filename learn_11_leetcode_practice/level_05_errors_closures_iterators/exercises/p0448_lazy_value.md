# Problem 0448: Lazy Initialization with FnOnce

- **Difficulty:** Medium
- **Topics:** struct generic over `F: FnOnce() -> T`, `Option::take`, `Option::get_or_insert_with`, disjoint field capture, consuming `self`
- **Source:** Original · [std `Option::get_or_insert_with`](https://doc.rust-lang.org/std/option/enum.Option.html#method.get_or_insert_with) · [std `FnOnce`](https://doc.rust-lang.org/std/ops/trait.FnOnce.html)

## Problem

Some values are expensive to build and not always needed: a parsed configuration, a big lookup
table... *Lazy initialization* stores **how** to build the value (a closure) and runs it only the
first time the value is requested. Because the builder runs at most once, it only needs to be
`FnOnce`.

`Lazy<T, F>` holds two fields: `init: Option<F>` (the builder, until it is used) and
`value: Option<T>` (the value, once built). Implement:

- `Lazy::new(init: F) -> Lazy<T, F>`: nothing is computed yet.
- `is_initialized(&self) -> bool`.
- `get(&mut self) -> &T`: build the value on the first call, then return a reference to it.
- `into_value(self) -> T`: build the value if needed and return it by value.

## Examples

```text
let mut config = Lazy::new(|| "port=8080".to_string());
config.is_initialized()  -> false
config.get()             -> "port=8080"   (the closure runs now)
config.get()             -> "port=8080"   (no second run)
config.is_initialized()  -> true
Lazy::new(|| vec![1, 2, 3]).into_value() -> [1, 2, 3]
```

## Constraints

- No special constraints.

## Hints

1. Invariant: exactly one of `init` and `value` is `Some`.
2. `self.value.get_or_insert_with(|| ...)` runs its closure only when `value` is `None`. Inside,
   `self.init.take()` gives you the builder; borrowing `self.init` and `self.value` at the same
   time is fine because closures capture individual fields.
3. `into_value` can call `get` first, then move the value out of `self.value`.

## Starter code

Write your code in [`src/practice/p0448_lazy_value.rs`](../src/practice/p0448_lazy_value.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0448
```

```rust
pub struct Lazy<T, F: FnOnce() -> T> {
    init: Option<F>,
    value: Option<T>,
}

impl<T, F: FnOnce() -> T> Lazy<T, F> {
    pub fn new(init: F) -> Lazy<T, F> {
        todo!("Write your solution here")
    }

    pub fn is_initialized(&self) -> bool {
        todo!("Write your solution here")
    }

    pub fn get(&mut self) -> &T {
        todo!("Write your solution here")
    }

    pub fn into_value(mut self) -> T {
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
    fn value_is_built_on_first_get() {
        let mut config = Lazy::new(|| "port=8080".to_string());
        assert!(!config.is_initialized());
        assert_eq!(config.get(), "port=8080");
        assert!(config.is_initialized());
        assert_eq!(config.get(), "port=8080");
    }

    #[test]
    fn builder_runs_at_most_once() {
        let mut runs = 0;
        {
            let mut lazy = Lazy::new(|| {
                runs += 1;
                42
            });
            assert_eq!(*lazy.get(), 42);
            assert_eq!(*lazy.get(), 42);
            assert_eq!(*lazy.get(), 42);
        }
        assert_eq!(runs, 1);
    }

    #[test]
    fn builder_never_runs_if_unused() {
        let mut runs = 0;
        {
            let lazy = Lazy::new(|| {
                runs += 1;
                0
            });
            assert!(!lazy.is_initialized());
        }
        assert_eq!(runs, 0);
    }

    #[test]
    fn into_value_builds_or_reuses() {
        assert_eq!(Lazy::new(|| vec![1, 2, 3]).into_value(), vec![1, 2, 3]);
        let mut lazy = Lazy::new(|| String::from("once"));
        lazy.get();
        assert_eq!(lazy.into_value(), "once");
    }

    #[test]
    fn builder_can_consume_captured_data() {
        let words = vec!["a".to_string(), "b".to_string()];
        let mut joined = Lazy::new(move || words.into_iter().collect::<String>());
        assert_eq!(joined.get(), "ab");
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** keep the builder in an `Option` until it is needed. On the first `get`, `take()` it
(leaving `None`), call it, and store the result in `value`. From then on `get` only returns a
reference. The invariant "one of the two options is `Some`" makes the two `expect` calls
unreachable.

**Rust concepts:**
- Generic parameters on a struct (`Lazy<T, F>`) with a closure bound (`F: FnOnce() -> T`) store
  the closure without boxing.
- Calling an `FnOnce` consumes it, so it must be moved out of the struct first: `Option::take`
  is the tool for moving out of a field behind `&mut self`.
- `get_or_insert_with` returns `&mut T`, which coerces to the `&T` we return.
- `into_value(mut self)` takes ownership, so it may move `value` out of the struct.
- The standard library has ready-made versions of this idea: `std::cell::LazyCell` and
  `std::sync::LazyLock` (stable since Rust 1.80).

```rust
pub struct Lazy<T, F: FnOnce() -> T> {
    init: Option<F>,
    value: Option<T>,
}

impl<T, F: FnOnce() -> T> Lazy<T, F> {
    pub fn new(init: F) -> Lazy<T, F> {
        Lazy {
            init: Some(init),
            value: None,
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.value.is_some()
    }

    pub fn get(&mut self) -> &T {
        let init = &mut self.init;
        self.value.get_or_insert_with(|| {
            // `init` is `Some` exactly as long as `value` is `None` (see `new`).
            let build = init
                .take()
                .expect("the builder is present until the value exists");
            build()
        })
    }

    pub fn into_value(mut self) -> T {
        self.get();
        // `get` has just stored the value.
        self.value.expect("get always stores the value")
    }
}
```

Full tested solution: [`src/solutions/p0448_lazy_value.rs`](../src/solutions/p0448_lazy_value.rs)

</details>
