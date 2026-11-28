# Problem 0550: 2D Vector Operators

- **Difficulty:** Easy
- **Topics:** operator overloading with `Add`, `Sub`, `Neg`, `Mul<f64>`, `Mul<Vec2> for f64`, `AddAssign`, `SubAssign`, `iter::Sum`
- **Source:** Original · [The Rust Book 20.2](https://doc.rust-lang.org/book/ch20-02-advanced-traits.html#default-generic-type-parameters-and-operator-overloading)

## Problem

`Vec2 { x: f64, y: f64 }` is a 2D vector (derives `Debug, Clone, Copy, PartialEq, Default`). Make the
usual math notation work by implementing the operator traits from `std::ops`:

| Expression | Trait | Result |
| --- | --- | --- |
| `a + b` | `Add` | component-wise sum |
| `a - b` | `Sub` | component-wise difference |
| `-a` | `Neg` | both components negated |
| `a * 2.0` | `Mul<f64> for Vec2` | both components scaled |
| `2.0 * a` | `Mul<Vec2> for f64` | the same, with the scalar on the left |
| `a += b`, `a -= b` | `AddAssign`, `SubAssign` | in-place versions |
| `iter.sum::<Vec2>()` | `Sum<Vec2>` and `Sum<&Vec2>` for `Vec2` | sum of many vectors (`Vec2::default()` when empty) |

Also add two methods: `new(x, y) -> Self` and `dot(self, other: Vec2) -> f64`.

## Examples

```text
let a = Vec2::new(1.0, 2.0);
let b = Vec2::new(3.0, -1.0);
a + b             => Vec2 { x: 4.0, y: 1.0 }
a - b             => Vec2 { x: -2.0, y: 3.0 }
-a                => Vec2 { x: -1.0, y: -2.0 }
a * 3.0, 3.0 * a  => Vec2 { x: 3.0, y: 6.0 }
a.dot(b)          => 1.0
[a, b, a].iter().sum::<Vec2>() => Vec2 { x: 5.0, y: 3.0 }
```

## Constraints

- No special constraints.

## Hints

1. `impl Add for Vec2 { type Output = Vec2; fn add(self, rhs: Vec2) -> Vec2 { ... } }`. The `Rhs`
   parameter of `Add` defaults to `Self`, which is why `impl Add for Vec2` needs no `<...>`.
2. `impl Mul<Vec2> for f64` is allowed: the orphan rule is satisfied because `Vec2` is local.
3. `impl Sum for Vec2` gets an iterator of `Vec2`; fold it starting at `Vec2::default()`. For
   `Sum<&'a Vec2>` the items are references; `Vec2` is `Copy`, so dereference them.

## Starter code

Write your code in [`src/practice/p0550_vector2_operators.rs`](../src/practice/p0550_vector2_operators.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0550
```

```rust
use std::iter::Sum;
use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        todo!("Write your solution here")
    }

    pub fn dot(self, other: Vec2) -> f64 {
        todo!("Write your solution here")
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        todo!("Write your solution here")
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Vec2 {
        todo!("Write your solution here")
    }
}

impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Vec2 {
        todo!("Write your solution here")
    }
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;

    fn mul(self, k: f64) -> Vec2 {
        todo!("Write your solution here")
    }
}

impl Mul<Vec2> for f64 {
    type Output = Vec2;

    fn mul(self, v: Vec2) -> Vec2 {
        todo!("Write your solution here")
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        todo!("Write your solution here")
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        todo!("Write your solution here")
    }
}

impl Sum for Vec2 {
    fn sum<I: Iterator<Item = Vec2>>(iter: I) -> Vec2 {
        todo!("Write your solution here")
    }
}

impl<'a> Sum<&'a Vec2> for Vec2 {
    fn sum<I: Iterator<Item = &'a Vec2>>(iter: I) -> Vec2 {
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
    fn binary_operators() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, -1.0);
        assert_eq!(a + b, Vec2 { x: 4.0, y: 1.0 });
        assert_eq!(a - b, Vec2 { x: -2.0, y: 3.0 });
        assert_eq!(-a, Vec2 { x: -1.0, y: -2.0 });
        assert_eq!(a.dot(b), 1.0);
    }

    #[test]
    fn scalar_on_both_sides() {
        let a = Vec2::new(1.0, 2.0);
        assert_eq!(a * 3.0, Vec2 { x: 3.0, y: 6.0 });
        assert_eq!(3.0 * a, a * 3.0);
        assert_eq!(0.5 * (a + a), a);
    }

    #[test]
    fn assign_operators() {
        let mut p = Vec2::new(0.0, 0.0);
        p += Vec2::new(1.5, 1.5);
        p += Vec2::new(1.0, -2.0);
        p -= Vec2::new(0.5, 0.5);
        assert_eq!(p, Vec2 { x: 2.0, y: -1.0 });
    }

    #[test]
    fn sums() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, -1.0);
        assert_eq!([a, b, a].iter().sum::<Vec2>(), Vec2 { x: 5.0, y: 3.0 });
        assert_eq!(
            vec![a, b].into_iter().sum::<Vec2>(),
            Vec2 { x: 4.0, y: 1.0 }
        );
        assert_eq!(
            Vec::<Vec2>::new().into_iter().sum::<Vec2>(),
            Vec2::default()
        );
        let many: Vec2 = (0..1000).map(|i| Vec2::new(f64::from(i), 1.0)).sum();
        assert_eq!(
            many,
            Vec2 {
                x: 499_500.0,
                y: 1000.0
            }
        );
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** every operator applies the same operation to `x` and `y`. The assign operators reuse the
binary ones (`*self = *self + rhs`) and `Sum` folds with `+`.

**Rust concepts:**
- Operators are trait method calls: `a + b` is `Add::add(a, b)`. Only the operators listed in
  `std::ops` can be overloaded; you cannot invent new ones.
- `Output` is an associated type: it lets `Vec2 * f64` produce a `Vec2` and `Vec2.dot` style
  operations produce other types.
- Because `Vec2` is `Copy`, the operators can take `self` by value without moving anything the
  caller still needs.
- `Sum<&'a Vec2>` is what makes `slice.iter().sum()` work; `Sum<Vec2>` covers `into_iter()`.

```rust
use std::iter::Sum;
use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Vec2 { x, y }
    }

    pub fn dot(self, other: Vec2) -> f64 {
        self.x * other.x + self.y * other.y
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Vec2 {
        Vec2::new(-self.x, -self.y)
    }
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;

    fn mul(self, k: f64) -> Vec2 {
        Vec2::new(self.x * k, self.y * k)
    }
}

impl Mul<Vec2> for f64 {
    type Output = Vec2;

    fn mul(self, v: Vec2) -> Vec2 {
        v * self
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        *self = *self + rhs;
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        *self = *self - rhs;
    }
}

impl Sum for Vec2 {
    fn sum<I: Iterator<Item = Vec2>>(iter: I) -> Vec2 {
        iter.fold(Vec2::default(), |acc, v| acc + v)
    }
}

impl<'a> Sum<&'a Vec2> for Vec2 {
    fn sum<I: Iterator<Item = &'a Vec2>>(iter: I) -> Vec2 {
        iter.copied().sum()
    }
}
```

Full tested solution: [`src/solutions/p0550_vector2_operators.rs`](../src/solutions/p0550_vector2_operators.rs)

</details>
