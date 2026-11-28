# Problem 0556: Polynomial Arithmetic

- **Difficulty:** Medium
- **Topics:** operator traits implemented for references (`impl Add for &Polynomial`), normalised representation, Horner's method, `Display` with signs
- **Source:** Original · [std::ops::Add](https://doc.rust-lang.org/std/ops/trait.Add.html)

## Problem

A polynomial with integer coefficients is stored as `Polynomial { coeffs: Vec<i64> }` (private
field), where `coeffs[i]` is the coefficient of `x^i`. The representation is **normalised**: no
trailing zero coefficients, so the zero polynomial has an empty vector.

Implement:

- `Polynomial::new(coeffs: Vec<i64>) -> Self` (removes trailing zeros), `coeffs(&self) -> &[i64]`,
  `degree(&self) -> Option<usize>` (`None` for the zero polynomial).
- `eval(&self, x: i64) -> i64` using Horner's method.
- `derivative(&self) -> Polynomial`.
- Operators on **references**, so that polynomials are not consumed: `&p + &q`, `&p - &q`, `&p * &q`
  and `-&p`, each returning a new `Polynomial`. Also `impl Add for Polynomial` (by value) that
  delegates to the reference version.
- `Display`, from the highest power down: `"3x^2 - 2x + 1"`, `"-x^3 + 5"`, `"x"`, `"0"`. Rules:
  zero coefficients are skipped; a coefficient of `1` or `-1` is not written for non-constant terms;
  `x^1` is written `x`; terms after the first are joined with `" + "` or `" - "`; a negative first
  term starts with `"-"`.

## Examples

```text
p = Polynomial::new(vec![1, -2, 3])   // 3x^2 - 2x + 1
q = Polynomial::new(vec![0, 1])       // x
(&p + &q).to_string()                 => "3x^2 - x + 1"
(&p * &q).to_string()                 => "3x^3 - 2x^2 + x"
(&p - &p).degree()                    => None
p.eval(2)                             => 9
p.derivative().to_string()            => "6x - 2"
```

## Constraints

- All results fit in `i64` in the tests.

## Hints

1. Normalise in one place: `new` pops zeros from the end. Every operator builds its coefficient
   vector and calls `new`.
2. For multiplication, `out[i + j] += a[i] * b[j]`.
3. Horner: `((c_n * x + c_{n-1}) * x + ...) * x + c_0`, i.e. fold the coefficients in reverse.

## Starter code

Write your code in [`src/practice/p0556_polynomial_arithmetic.rs`](../src/practice/p0556_polynomial_arithmetic.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0556
```

```rust
use std::fmt;
use std::ops::{Add, Mul, Neg, Sub};

/// A polynomial with integer coefficients; `coeffs[i]` multiplies `x^i`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Polynomial {}

impl Polynomial {
    pub fn new(coeffs: Vec<i64>) -> Self {
        todo!("Write your solution here")
    }

    pub fn coeffs(&self) -> &[i64] {
        todo!("Write your solution here")
    }

    pub fn degree(&self) -> Option<usize> {
        todo!("Write your solution here")
    }

    pub fn eval(&self, x: i64) -> i64 {
        todo!("Write your solution here")
    }

    pub fn derivative(&self) -> Polynomial {
        todo!("Write your solution here")
    }
}

impl Add for &Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: &Polynomial) -> Polynomial {
        todo!("Write your solution here")
    }
}

impl Neg for &Polynomial {
    type Output = Polynomial;

    fn neg(self) -> Polynomial {
        todo!("Write your solution here")
    }
}

impl Sub for &Polynomial {
    type Output = Polynomial;

    fn sub(self, rhs: &Polynomial) -> Polynomial {
        todo!("Write your solution here")
    }
}

impl Mul for &Polynomial {
    type Output = Polynomial;

    fn mul(self, rhs: &Polynomial) -> Polynomial {
        todo!("Write your solution here")
    }
}

impl Add for Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: Polynomial) -> Polynomial {
        todo!("Write your solution here")
    }
}

impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

    fn poly(c: &[i64]) -> Polynomial {
        Polynomial::new(c.to_vec())
    }

    #[test]
    fn construction_and_display() {
        assert_eq!(poly(&[1, -2, 3]).to_string(), "3x^2 - 2x + 1");
        assert_eq!(poly(&[5, 0, 0, -1]).to_string(), "-x^3 + 5");
        assert_eq!(poly(&[0, 1]).to_string(), "x");
        assert_eq!(poly(&[0, 0, 0]).to_string(), "0");
        assert_eq!(poly(&[-1]).to_string(), "-1");
        assert_eq!(poly(&[0, -1, 1]).to_string(), "x^2 - x");
        assert_eq!(poly(&[4, 0, 0]).coeffs(), &[4]);
        assert_eq!(poly(&[]).degree(), None);
        assert_eq!(poly(&[1, -2, 3]).degree(), Some(2));
    }

    #[test]
    fn operators_on_references() {
        let p = poly(&[1, -2, 3]);
        let q = poly(&[0, 1]);
        assert_eq!((&p + &q).to_string(), "3x^2 - x + 1");
        assert_eq!((&p - &q).to_string(), "3x^2 - 3x + 1");
        assert_eq!((&p * &q).to_string(), "3x^3 - 2x^2 + x");
        assert_eq!((-&q).to_string(), "-x");
        assert_eq!((&p - &p).degree(), None);
        // p and q are still usable: the operators only borrowed them.
        assert_eq!(p.clone() + q.clone(), &p + &q);
    }

    #[test]
    fn evaluation_and_derivative() {
        let p = poly(&[1, -2, 3]);
        assert_eq!(p.eval(2), 9);
        assert_eq!(p.eval(0), 1);
        assert_eq!(p.eval(-1), 6);
        assert_eq!(p.derivative().to_string(), "6x - 2");
        assert_eq!(poly(&[7]).derivative(), poly(&[]));
        assert_eq!(poly(&[]).eval(100), 0);
    }

    #[test]
    fn binomial_expansion() {
        // (x + 1)^10 has the binomial coefficients 1, 10, 45, 120, 210, 252, ...
        let base = poly(&[1, 1]);
        let mut p = poly(&[1]);
        for _ in 0..10 {
            p = &p * &base;
        }
        assert_eq!(p.coeffs(), &[1, 10, 45, 120, 210, 252, 210, 120, 45, 10, 1]);
        assert_eq!(p.eval(1), 1024);
        assert_eq!((&p * &poly(&[])).to_string(), "0");
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** coefficient vectors make addition a component-wise sum (padding the shorter one with
zeros) and multiplication a double loop. `Display` walks the coefficients from the highest power,
printing a sign separator and then the magnitude.

**Rust concepts:**
- Implementing `Add` for `&Polynomial` (with `type Output = Polynomial`) allows `&p + &q` without
  moving or cloning `p` and `q`; the lifetime of the references is elided in `impl Add for
  &Polynomial`.
- The by-value impl can delegate: `fn add(self, rhs) -> Polynomial { &self + &rhs }`.
- Keeping the representation normalised makes the derived `PartialEq` mean mathematical equality.

```rust
use std::fmt;
use std::ops::{Add, Mul, Neg, Sub};

/// A polynomial with integer coefficients; `coeffs[i]` multiplies `x^i`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Polynomial {
    coeffs: Vec<i64>,
}

impl Polynomial {
    pub fn new(coeffs: Vec<i64>) -> Self {
        let mut coeffs = coeffs;
        while coeffs.last() == Some(&0) {
            coeffs.pop();
        }
        Polynomial { coeffs }
    }

    pub fn coeffs(&self) -> &[i64] {
        &self.coeffs
    }

    pub fn degree(&self) -> Option<usize> {
        self.coeffs.len().checked_sub(1)
    }

    pub fn eval(&self, x: i64) -> i64 {
        self.coeffs.iter().rev().fold(0, |acc, &c| acc * x + c)
    }

    pub fn derivative(&self) -> Polynomial {
        Polynomial::new(
            self.coeffs
                .iter()
                .enumerate()
                .skip(1)
                .map(|(i, &c)| c * i as i64)
                .collect(),
        )
    }
}

impl Add for &Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: &Polynomial) -> Polynomial {
        let len = self.coeffs.len().max(rhs.coeffs.len());
        let at = |p: &Polynomial, i: usize| p.coeffs.get(i).copied().unwrap_or(0);
        Polynomial::new((0..len).map(|i| at(self, i) + at(rhs, i)).collect())
    }
}

impl Neg for &Polynomial {
    type Output = Polynomial;

    fn neg(self) -> Polynomial {
        Polynomial::new(self.coeffs.iter().map(|&c| -c).collect())
    }
}

impl Sub for &Polynomial {
    type Output = Polynomial;

    fn sub(self, rhs: &Polynomial) -> Polynomial {
        self + &(-rhs)
    }
}

impl Mul for &Polynomial {
    type Output = Polynomial;

    fn mul(self, rhs: &Polynomial) -> Polynomial {
        if self.coeffs.is_empty() || rhs.coeffs.is_empty() {
            return Polynomial::new(Vec::new());
        }
        let mut out = vec![0; self.coeffs.len() + rhs.coeffs.len() - 1];
        for (i, &a) in self.coeffs.iter().enumerate() {
            for (j, &b) in rhs.coeffs.iter().enumerate() {
                out[i + j] += a * b;
            }
        }
        Polynomial::new(out)
    }
}

impl Add for Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: Polynomial) -> Polynomial {
        &self + &rhs
    }
}

impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.coeffs.is_empty() {
            return write!(f, "0");
        }
        let mut first = true;
        for (power, &c) in self.coeffs.iter().enumerate().rev() {
            if c == 0 {
                continue;
            }
            let sign = match (first, c < 0) {
                (true, true) => "-",
                (true, false) => "",
                (false, true) => " - ",
                (false, false) => " + ",
            };
            first = false;
            let magnitude = c.unsigned_abs();
            let number = if magnitude == 1 && power > 0 {
                String::new()
            } else {
                magnitude.to_string()
            };
            match power {
                0 => write!(f, "{sign}{number}")?,
                1 => write!(f, "{sign}{number}x")?,
                _ => write!(f, "{sign}{number}x^{power}")?,
            }
        }
        Ok(())
    }
}
```

Full tested solution: [`src/solutions/p0556_polynomial_arithmetic.rs`](../src/solutions/p0556_polynomial_arithmetic.rs)

</details>
