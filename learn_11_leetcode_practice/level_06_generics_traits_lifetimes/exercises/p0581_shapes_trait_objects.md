# Problem 0581: Shapes as Trait Objects

- **Difficulty:** Easy
- **Topics:** `dyn Trait`, `Vec<Box<dyn Shape>>` for heterogeneous collections, `&dyn Shape`, static vs dynamic dispatch
- **Source:** Original · [The Rust Book 18.2](https://doc.rust-lang.org/book/ch18-02-trait-objects.html)

## Problem

A generic function `fn total<T: Shape>(shapes: &[T])` only accepts slices where **every** element
has the same type. To mix circles and rectangles in one collection, store **trait objects**:
`Box<dyn Shape>` (owned) or `&dyn Shape` (borrowed).

The trait:

```text
pub trait Shape {
    fn name(&self) -> String;
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}
```

Implement it for `Circle { radius }`, `Rect { width, height }` and `Triangle { a, b, c }` (side
lengths; use Heron's formula `sqrt(s(s-a)(s-b)(s-c))` with `s` the half-perimeter). Names are
`"circle"`, `"rect"` and `"triangle"`.

Then write:

- `total_area(shapes: &[Box<dyn Shape>]) -> f64`
- `largest(shapes: &[Box<dyn Shape>]) -> Option<&dyn Shape>`: the shape with the largest area (first
  on ties).
- `describe(shapes: &[&dyn Shape]) -> Vec<String>`: `"{name}: area {area:.2}, perimeter {perimeter:.2}"`.
- `total_area_static<T: Shape>(shapes: &[T]) -> f64`: the same sum with **static** dispatch, for a
  slice of one concrete shape type.

## Examples

```text
let shapes: Vec<Box<dyn Shape>> = vec![Box::new(Circle { radius: 1.0 }), Box::new(Rect { width: 2.0, height: 3.0 })];
total_area(&shapes)                     => 9.14159...    (PI + 6)
largest(&shapes).map(|s| s.name())      => Some("rect")
describe(&[&Triangle { a: 3.0, b: 4.0, c: 5.0 }]) => ["triangle: area 6.00, perimeter 12.00"]
total_area_static(&[Rect { width: 1.0, height: 1.0 }; 3]) => 3.0
```

## Constraints

- All dimensions are positive and triangles are valid.

## Hints

1. `Box::new(Circle { .. })` coerces to `Box<dyn Shape>` when the expected type says so.
2. `shapes.iter().map(|s| s.area())` works on a `&Box<dyn Shape>` thanks to auto-deref.
3. In `largest`, `s.as_ref()` turns a `&Box<dyn Shape>` into a `&dyn Shape`.

## Starter code

Write your code in [`src/practice/p0581_shapes_trait_objects.rs`](../src/practice/p0581_shapes_trait_objects.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0581
```

```rust
use std::f64::consts::PI;

/// A 2D shape.
pub trait Shape {
    fn name(&self) -> String;
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circle {
    pub radius: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

impl Shape for Circle {
    fn name(&self) -> String {
        todo!("Write your solution here")
    }

    fn area(&self) -> f64 {
        todo!("Write your solution here")
    }

    fn perimeter(&self) -> f64 {
        todo!("Write your solution here")
    }
}

impl Shape for Rect {
    fn name(&self) -> String {
        todo!("Write your solution here")
    }

    fn area(&self) -> f64 {
        todo!("Write your solution here")
    }

    fn perimeter(&self) -> f64 {
        todo!("Write your solution here")
    }
}

impl Shape for Triangle {
    fn name(&self) -> String {
        todo!("Write your solution here")
    }

    fn area(&self) -> f64 {
        todo!("Write your solution here")
    }

    fn perimeter(&self) -> f64 {
        todo!("Write your solution here")
    }
}

/// The sum of all areas (dynamic dispatch).
pub fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    todo!("Write your solution here")
}

/// The shape with the largest area (first on ties).
pub fn largest(shapes: &[Box<dyn Shape>]) -> Option<&dyn Shape> {
    todo!("Write your solution here")
}

/// One description line per shape.
pub fn describe(shapes: &[&dyn Shape]) -> Vec<String> {
    todo!("Write your solution here")
}

/// The sum of all areas (static dispatch, one concrete type).
pub fn total_area_static<T: Shape>(shapes: &[T]) -> f64 {
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

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    fn mixed() -> Vec<Box<dyn Shape>> {
        vec![
            Box::new(Circle { radius: 1.0 }),
            Box::new(Rect {
                width: 2.0,
                height: 3.0,
            }),
            Box::new(Triangle {
                a: 3.0,
                b: 4.0,
                c: 5.0,
            }),
        ]
    }

    #[test]
    fn individual_shapes() {
        assert!(close(Circle { radius: 2.0 }.area(), 4.0 * PI));
        assert!(close(Circle { radius: 1.0 }.perimeter(), 2.0 * PI));
        assert_eq!(
            Rect {
                width: 2.0,
                height: 3.0
            }
            .perimeter(),
            10.0
        );
        assert!(close(
            Triangle {
                a: 3.0,
                b: 4.0,
                c: 5.0
            }
            .area(),
            6.0
        ));
    }

    #[test]
    fn heterogeneous_collection() {
        let shapes = mixed();
        assert!(close(total_area(&shapes), PI + 6.0 + 6.0));
        assert_eq!(largest(&shapes).map(|s| s.name()), Some("rect".to_string()));
        assert!(largest(&[]).is_none());
    }

    #[test]
    fn borrowed_trait_objects() {
        let t = Triangle {
            a: 3.0,
            b: 4.0,
            c: 5.0,
        };
        let c = Circle { radius: 0.5 };
        assert_eq!(
            describe(&[&t, &c]),
            vec![
                "triangle: area 6.00, perimeter 12.00",
                "circle: area 0.79, perimeter 3.14"
            ]
        );
    }

    #[test]
    fn static_dispatch_for_one_type() {
        let squares = [Rect {
            width: 1.0,
            height: 1.0,
        }; 3];
        assert_eq!(total_area_static(&squares), 3.0);
        let many: Vec<Box<dyn Shape>> = (1..=1000)
            .map(|i| {
                Box::new(Rect {
                    width: f64::from(i),
                    height: 1.0,
                }) as Box<dyn Shape>
            })
            .collect();
        assert_eq!(total_area(&many), 500_500.0);
        assert_eq!(largest(&many).map(|s| s.area()), Some(1000.0));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** every function only calls trait methods, so it works for any mix of shapes. The trait
object version looks methods up in a vtable at run time; the generic version is compiled once per
concrete type.

**Rust concepts:**
- `dyn Shape` is a dynamically sized type; it always lives behind a pointer (`Box`, `&`, `Rc`). The
  pointer is "fat": data pointer plus vtable pointer.
- `Vec<Box<dyn Shape>>` can hold different concrete types; `Vec<T>` with `T: Shape` cannot.
- Static dispatch (`T: Shape`) allows inlining; dynamic dispatch trades a little speed for
  flexibility and smaller code.
- `Option<&dyn Shape>` in `largest` borrows from the slice (lifetime elision ties them).

```rust
use std::f64::consts::PI;

/// A 2D shape.
pub trait Shape {
    fn name(&self) -> String;
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circle {
    pub radius: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

impl Shape for Circle {
    fn name(&self) -> String {
        "circle".to_string()
    }

    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }
}

impl Shape for Rect {
    fn name(&self) -> String {
        "rect".to_string()
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

impl Shape for Triangle {
    fn name(&self) -> String {
        "triangle".to_string()
    }

    fn area(&self) -> f64 {
        let s = self.perimeter() / 2.0;
        (s * (s - self.a) * (s - self.b) * (s - self.c)).sqrt()
    }

    fn perimeter(&self) -> f64 {
        self.a + self.b + self.c
    }
}

/// The sum of all areas (dynamic dispatch).
pub fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

/// The shape with the largest area (first on ties).
pub fn largest(shapes: &[Box<dyn Shape>]) -> Option<&dyn Shape> {
    let mut best: Option<&dyn Shape> = None;
    for s in shapes {
        if best.is_none_or(|b| s.area() > b.area()) {
            best = Some(s.as_ref());
        }
    }
    best
}

/// One description line per shape.
pub fn describe(shapes: &[&dyn Shape]) -> Vec<String> {
    shapes
        .iter()
        .map(|s| {
            format!(
                "{}: area {:.2}, perimeter {:.2}",
                s.name(),
                s.area(),
                s.perimeter()
            )
        })
        .collect()
}

/// The sum of all areas (static dispatch, one concrete type).
pub fn total_area_static<T: Shape>(shapes: &[T]) -> f64 {
    shapes.iter().map(Shape::area).sum()
}
```

Full tested solution: [`src/solutions/p0581_shapes_trait_objects.rs`](../src/solutions/p0581_shapes_trait_objects.rs)

</details>
