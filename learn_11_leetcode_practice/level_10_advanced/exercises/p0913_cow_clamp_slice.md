# Problem 0913: Clone-on-Write Slices with `to_mut`

- **Difficulty:** Medium
- **Topics:** `Cow<'a, [T]>`, `Cow::to_mut`, `ToOwned`, `Into<Cow<'static, str>>`, counting clones
- **Source:** Original · [std: `Cow::to_mut`](https://doc.rust-lang.org/std/borrow/enum.Cow.html#method.to_mut)

## Problem

`Cow::to_mut` is the heart of clone-on-write: the first call on a `Borrowed` value clones the data
into an `Owned` one and returns `&mut` to it; later calls just return the same `&mut`. Use it to
write functions that copy their input **at most once, and only if they must change it**.

1. `clamp_slice(values: &[i32], low: i32, high: i32) -> Cow<'_, [i32]>`: every value clamped into
   `[low, high]`. Borrowed if no value changes.
2. `replace_all<'a, T: Clone + PartialEq>(items: &'a [T], from: &T, to: &T) -> Cow<'a, [T]>`:
   every element equal to `from` replaced by a clone of `to`. Borrowed if nothing matches. It must
   clone **exactly** `items.len() + matches` times when something matches (one copy of the slice,
   then one clone of `to` per match) and **zero** times otherwise.
3. `Settings` with a field `name: Cow<'static, str>`:
   - `Settings::default()` has the name `"untitled"`, stored **borrowed** (no allocation), and
     `retries: 3`.
   - `with_name(self, name: impl Into<Cow<'static, str>>) -> Settings` accepts both a
     `&'static str` (stays borrowed) and a `String` (stored owned); other fields are kept.
   - `name(&self) -> &str`.

## Examples

```text
clamp_slice(&[1, 5, 9], 0, 10)    => Borrowed([1, 5, 9])
clamp_slice(&[-3, 5, 12], 0, 10)  => Owned([0, 5, 10])
replace_all(&["a", "b", "a"], &"a", &"z") => Owned(["z", "b", "z"])
Settings::default().name()                 => "untitled"
Settings::default().with_name(format!("v{}", 2)).name() => "v2"
```

## Constraints

- `low <= high`.

## Hints

1. Start with `let mut result = Cow::Borrowed(values);` and loop over the **indices**.
2. Call `result.to_mut()[i] = new_value;` only for elements that change: the clone happens on the
   first call.
3. `impl Into<Cow<'static, str>>` works for `&'static str` and `String` because `Cow` implements
   `From` for both.

## Starter code

Write your code in [`src/practice/p0913_cow_clamp_slice.rs`](../src/practice/p0913_cow_clamp_slice.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0913
```

```rust
use std::borrow::Cow;

pub fn clamp_slice(values: &[i32], low: i32, high: i32) -> Cow<'_, [i32]> {
    todo!("Write your solution here")
}

pub fn replace_all<'a, T: Clone + PartialEq>(items: &'a [T], from: &T, to: &T) -> Cow<'a, [T]> {
    todo!("Write your solution here")
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Settings {
    pub name: Cow<'static, str>,
    pub retries: u8,
}

impl Default for Settings {
    fn default() -> Self {
        todo!("Write your solution here")
    }
}

impl Settings {
    pub fn with_name(self, name: impl Into<Cow<'static, str>>) -> Settings {
        todo!("Write your solution here")
    }

    pub fn name(&self) -> &str {
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
    use std::cell::Cell;
    use std::rc::Rc;

    /// A value that counts how many times it (or any value sharing its counter) was cloned.
    #[derive(Debug)]
    struct Counted {
        id: u32,
        clones: Rc<Cell<usize>>,
    }

    impl Clone for Counted {
        fn clone(&self) -> Self {
            self.clones.set(self.clones.get() + 1);
            Counted {
                id: self.id,
                clones: Rc::clone(&self.clones),
            }
        }
    }

    impl PartialEq for Counted {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }

    #[test]
    fn clamp_slice_borrows_or_fixes() {
        let ok = clamp_slice(&[1, 5, 9], 0, 10);
        assert!(matches!(ok, Cow::Borrowed(_)));
        let fixed = clamp_slice(&[-3, 5, 12], 0, 10);
        assert!(matches!(fixed, Cow::Owned(_)));
        assert_eq!(&*fixed, &[0, 5, 10]);
        assert!(matches!(clamp_slice(&[], 0, 1), Cow::Borrowed(_)));
    }

    #[test]
    fn replace_all_values() {
        let out = replace_all(&["a", "b", "a"], &"a", &"z");
        assert_eq!(&*out, &["z", "b", "z"]);
        let same = replace_all(&[1, 2, 3], &9, &0);
        assert!(matches!(same, Cow::Borrowed(_)));
    }

    #[test]
    fn replace_all_clones_exactly_as_needed() {
        let clones = Rc::new(Cell::new(0));
        let make = |id| Counted {
            id,
            clones: Rc::clone(&clones),
        };
        let items = vec![make(1), make(2), make(1), make(3), make(1)];
        let none = replace_all(&items, &make(7), &make(0));
        assert!(matches!(none, Cow::Borrowed(_)));
        assert_eq!(clones.get(), 0);
        let out = replace_all(&items, &make(1), &make(0));
        let ids: Vec<u32> = out.iter().map(|c| c.id).collect();
        assert_eq!(ids, vec![0, 2, 0, 3, 0]);
        // 5 clones to copy the slice once + 3 clones of `to`.
        assert_eq!(clones.get(), 8);
    }

    #[test]
    fn settings_names() {
        let default = Settings::default();
        assert_eq!(default.name(), "untitled");
        assert!(matches!(default.name, Cow::Borrowed(_)));
        let literal = Settings::default().with_name("report");
        assert!(matches!(literal.name, Cow::Borrowed("report")));
        let computed = Settings {
            retries: 9,
            ..Settings::default()
        }
        .with_name(format!("v{}", 2));
        assert_eq!(computed.retries, 9);
        assert_eq!(computed.name(), "v2");
        assert!(matches!(computed.name, Cow::Owned(_)));
    }

    #[test]
    fn large_slice_single_change() {
        let mut values: Vec<i32> = (0..100_000).collect();
        values[50_000] = -1;
        let out = clamp_slice(&values, 0, 100_000);
        assert_eq!(out[50_000], 0);
        assert_eq!(out.len(), 100_000);
        assert_eq!(
            out.iter().map(|&v| i64::from(v)).sum::<i64>(),
            4_999_950_000 - 50_000
        );
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** both slice functions start from `Cow::Borrowed(input)` and walk the indices, reading the
original element and writing through `to_mut()` only when it must change. The first `to_mut`
performs `to_owned()` (one clone per element), every later one reuses the owned vector, so the
number of clones is `len + matches` in `replace_all`, or zero if nothing matched.

**Complexity:** `O(n)` time; `O(n)` extra space only if something changes.

**Rust concepts:**
- `Cow<'a, [T]>` needs `[T]: ToOwned`, which holds for `T: Clone`; the owned form is `Vec<T>`.
- `to_mut(&mut self) -> &mut <B as ToOwned>::Owned` turns a borrowed `Cow` into an owned one in
  place; `into_owned()` would always produce an owned value.
- `Cow<'static, str>` is a common field type for "usually a literal, sometimes computed" strings.

```rust
use std::borrow::Cow;

pub fn clamp_slice(values: &[i32], low: i32, high: i32) -> Cow<'_, [i32]> {
    let mut result = Cow::Borrowed(values);
    for (i, &value) in values.iter().enumerate() {
        let clamped = value.clamp(low, high);
        if clamped != value {
            result.to_mut()[i] = clamped;
        }
    }
    result
}

pub fn replace_all<'a, T: Clone + PartialEq>(items: &'a [T], from: &T, to: &T) -> Cow<'a, [T]> {
    let mut result = Cow::Borrowed(items);
    for (i, item) in items.iter().enumerate() {
        if item == from {
            result.to_mut()[i] = to.clone();
        }
    }
    result
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Settings {
    pub name: Cow<'static, str>,
    pub retries: u8,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            name: Cow::Borrowed("untitled"),
            retries: 3,
        }
    }
}

impl Settings {
    pub fn with_name(self, name: impl Into<Cow<'static, str>>) -> Settings {
        Settings {
            name: name.into(),
            ..self
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
```

Full tested solution: [`src/solutions/p0913_cow_clamp_slice.rs`](../src/solutions/p0913_cow_clamp_slice.rs)

</details>
