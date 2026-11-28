# Problem 0209: Derive Debug, Clone and PartialEq

- **Difficulty:** Easy
- **Topics:** `#[derive(Debug, Clone, PartialEq)]`, `{:?}` and `{:#?}`, `clone`, `==` on structs
- **Source:** Original · [The Rust Book, Appendix C](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html)

## Problem

`#[derive(...)]` asks the compiler to write standard trait implementations for a struct:

- `Debug` enables `{:?}` (one line) and `{:#?}` (pretty, multi-line) formatting;
- `Clone` adds `.clone()`, a deep copy (every field is cloned);
- `PartialEq` adds `==` and `!=`, comparing all fields in order.

The types below already derive all three:

```text
pub struct Point3   { pub x: i32, pub y: i32, pub z: i32 }
pub struct Waypoint { pub name: String, pub position: Point3, pub tags: Vec<String> }
```

Implement, using the derived traits:

- `debug_line(w: &Waypoint) -> String`: the `{:?}` representation of `w`.
- `pretty(p: &Point3) -> String`: the `{:#?}` representation of `p`.
- `renamed_copy(w: &Waypoint, new_name: &str) -> Waypoint`: a clone of `w` with a new name and the
  extra tag `"copy"` appended to `tags`. The original must not change.
- `count_equal(points: &[Point3], target: &Point3) -> usize`: how many points equal `target`.

## Examples

```text
w = Waypoint { name: "Camp", position: Point3 { x: 1, y: 2, z: 3 }, tags: ["base"] }

debug_line(&w)
  => "Waypoint { name: \"Camp\", position: Point3 { x: 1, y: 2, z: 3 }, tags: [\"base\"] }"

pretty(&Point3 { x: 1, y: 2, z: 3 })
  => "Point3 {\n    x: 1,\n    y: 2,\n    z: 3,\n}"

renamed_copy(&w, "Camp 2").tags  => ["base", "copy"]
count_equal(&[p, q, p], &p)      => 2
```

## Constraints

- No special constraints.

## Hints

1. `format!("{:?}", w)` and `format!("{:#?}", p)` produce exactly the derived output.
2. Clone first (`let mut copy = w.clone();`), then change the copy's fields.
3. `points.iter().filter(|p| *p == target).count()` compares two `&Point3` values.

## Starter code

Write your code in [`src/practice/p0209_derive_debug_clone_partialeq.rs`](../src/practice/p0209_derive_debug_clone_partialeq.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0209
```

```rust
/// A point in 3D space.
#[derive(Debug, Clone, PartialEq)]
pub struct Point3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

/// A named point on a map.
#[derive(Debug, Clone, PartialEq)]
pub struct Waypoint {
    pub name: String,
    pub position: Point3,
    pub tags: Vec<String>,
}

/// The one-line debug representation.
pub fn debug_line(w: &Waypoint) -> String {
    todo!("Write your solution here")
}

/// The pretty, multi-line debug representation.
pub fn pretty(p: &Point3) -> String {
    todo!("Write your solution here")
}

/// A renamed copy of `w`, tagged with `"copy"`.
pub fn renamed_copy(w: &Waypoint, new_name: &str) -> Waypoint {
    todo!("Write your solution here")
}

/// Counts the points equal to `target`.
pub fn count_equal(points: &[Point3], target: &Point3) -> usize {
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

    fn camp() -> Waypoint {
        Waypoint {
            name: String::from("Camp"),
            position: Point3 { x: 1, y: 2, z: 3 },
            tags: vec![String::from("base")],
        }
    }

    #[test]
    fn example_debug_line() {
        assert_eq!(
            debug_line(&camp()),
            "Waypoint { name: \"Camp\", position: Point3 { x: 1, y: 2, z: 3 }, tags: [\"base\"] }"
        );
    }

    #[test]
    fn example_pretty() {
        assert_eq!(
            pretty(&Point3 { x: 1, y: 2, z: 3 }),
            "Point3 {\n    x: 1,\n    y: 2,\n    z: 3,\n}"
        );
    }

    #[test]
    fn example_renamed_copy() {
        let original = camp();
        let copy = renamed_copy(&original, "Camp 2");
        assert_eq!(copy.name, "Camp 2");
        assert_eq!(copy.tags, vec!["base", "copy"]);
        assert_eq!(copy.position, original.position);
        // The original is unchanged.
        assert_eq!(original, camp());
        assert_ne!(copy, original);
    }

    #[test]
    fn example_count_equal() {
        let p = Point3 { x: 0, y: 0, z: 1 };
        let q = Point3 { x: 0, y: 1, z: 0 };
        assert_eq!(count_equal(&[p.clone(), q, p.clone()], &p), 2);
    }

    #[test]
    fn count_in_empty_slice() {
        assert_eq!(count_equal(&[], &Point3 { x: 1, y: 1, z: 1 }), 0);
    }

    #[test]
    fn debug_escapes_quotes_and_shows_empty_vec() {
        let w = Waypoint {
            name: String::from("say \"hi\""),
            position: Point3 { x: -1, y: 0, z: 0 },
            tags: Vec::new(),
        };
        assert_eq!(
            debug_line(&w),
            "Waypoint { name: \"say \\\"hi\\\"\", position: Point3 { x: -1, y: 0, z: 0 }, tags: [] }"
        );
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** every function is a direct use of one derived trait: `Debug` for the two strings,
`Clone` for the copy, `PartialEq` for the count.

**Rust concepts:**
- The derived `Debug` prints the struct name, then `field: value` pairs; strings are quoted and
  escaped, vectors look like `[...]`. `{:#?}` puts one field per line with 4-space indentation and
  a trailing comma.
- Derives require every field type to implement the trait too: `Waypoint` can derive `Clone`
  only because `String`, `Point3` and `Vec<String>` are `Clone`.
- `clone()` of a struct containing a `Vec<String>` allocates new buffers, so changing the copy's
  `tags` never affects the original.
- `PartialEq` on references compares the values: `&a == &b` is `a == b`.

```rust
/// A point in 3D space.
#[derive(Debug, Clone, PartialEq)]
pub struct Point3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

/// A named point on a map.
#[derive(Debug, Clone, PartialEq)]
pub struct Waypoint {
    pub name: String,
    pub position: Point3,
    pub tags: Vec<String>,
}

/// The one-line debug representation.
pub fn debug_line(w: &Waypoint) -> String {
    format!("{w:?}")
}

/// The pretty, multi-line debug representation.
pub fn pretty(p: &Point3) -> String {
    format!("{p:#?}")
}

/// A renamed copy of `w`, tagged with `"copy"`.
pub fn renamed_copy(w: &Waypoint, new_name: &str) -> Waypoint {
    let mut copy = w.clone();
    copy.name = new_name.to_string();
    copy.tags.push(String::from("copy"));
    copy
}

/// Counts the points equal to `target`.
pub fn count_equal(points: &[Point3], target: &Point3) -> usize {
    points.iter().filter(|p| *p == target).count()
}
```

Full tested solution: [`src/solutions/p0209_derive_debug_clone_partialeq.rs`](../src/solutions/p0209_derive_debug_clone_partialeq.rs)

</details>
