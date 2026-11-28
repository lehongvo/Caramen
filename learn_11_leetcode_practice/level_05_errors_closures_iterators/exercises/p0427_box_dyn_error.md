# Problem 0427: Mixed Errors with Box<dyn Error>

- **Difficulty:** Medium
- **Topics:** `Box<dyn Error>`, `?` boxing any error, `From<&str>` / `From<String>` for `Box<dyn Error>`, `downcast_ref`, `Error::is`
- **Source:** Original · [The Rust Book 9.2: `?` in `main` and `Box<dyn Error>`](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#where-the--operator-can-be-used) · [Rust By Example: Boxing errors](https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html)

## Problem

A function that parses integers, parses floats and has its own rules can fail with several
*different* error types. Instead of writing an enum that wraps them all, you can return
`Result<T, Box<dyn Error>>`: a box can hold **any** type that implements `Error`, and `?`
boxes errors automatically. A caller who needs details can ask the box what is inside with
`downcast_ref::<T>()`.

An order line looks like `item, quantity, unit_price`, for example `apple, 3, 0.5`. Implement:

- `Display` + `Error` for `NegativeQuantity(q)`: message `"quantity cannot be negative: Q"`.
- `parse_order_line(line: &str) -> Result<OrderLine, Box<dyn Error>>`. Split on `','` and trim
  each field. Check, in this order:
  1. exactly 3 fields, else the error message `"expected 3 fields, found N"`;
  2. a non-empty item name, else `"empty item name"`;
  3. the quantity parses as an `i64` (propagate the `ParseIntError` with `?`);
  4. the quantity is not negative, else a boxed `NegativeQuantity(q)`;
  5. the price parses as an `f64` (propagate the `ParseFloatError` with `?`).
- `order_total(text: &str) -> Result<f64, Box<dyn Error>>`: the sum of `quantity * price` over
  all non-blank lines; the first error stops the computation.
- `error_kind(err: &(dyn Error + 'static)) -> &'static str`: `"integer"` for a `ParseIntError`,
  `"float"` for a `ParseFloatError`, `"negative"` for `NegativeQuantity`, `"other"` otherwise.

## Examples

```text
parse_order_line("apple, 3, 0.5")  -> Ok(OrderLine { item: "apple", quantity: 3, price: 0.5 })
parse_order_line("apple, 3")       -> Err: "expected 3 fields, found 2"     (kind "other")
parse_order_line("pear, x, 1")     -> Err: "invalid digit found in string"  (kind "integer")
parse_order_line("pear, -2, 1")    -> Err: "quantity cannot be negative: -2" (kind "negative")
parse_order_line("pear, 2, 1,5")   -> Err: "expected 3 fields, found 4"
parse_order_line("pear, 2, cheap") -> Err: "invalid float literal"          (kind "float")
order_total("apple, 3, 0.5\n\npear, 2, 1.25") -> Ok(4.0)
```

## Constraints

- At most `10^3` lines.

## Hints

1. `Err("empty item name".into())` and `Err(format!(...).into())` create boxed errors from text,
   because `Box<dyn Error>` implements `From<&str>` and `From<String>`.
2. `Err(Box::new(NegativeQuantity(q)))` boxes your own error.
3. `err.is::<ParseIntError>()` or `err.downcast_ref::<ParseIntError>().is_some()` tests the type.

## Starter code

Write your code in [`src/practice/p0427_box_dyn_error.rs`](../src/practice/p0427_box_dyn_error.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0427
```

```rust
use std::error::Error;
use std::fmt;
use std::num::{ParseFloatError, ParseIntError};

#[derive(Debug, PartialEq)]
pub struct NegativeQuantity(pub i64);

#[derive(Debug, PartialEq)]
pub struct OrderLine {
    pub item: String,
    pub quantity: i64,
    pub price: f64,
}

impl fmt::Display for NegativeQuantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("Write your solution here")
    }
}

impl Error for NegativeQuantity {}

pub fn parse_order_line(line: &str) -> Result<OrderLine, Box<dyn Error>> {
    todo!("Write your solution here")
}

pub fn order_total(text: &str) -> Result<f64, Box<dyn Error>> {
    todo!("Write your solution here")
}

pub fn error_kind(err: &(dyn Error + 'static)) -> &'static str {
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

    fn failure(line: &str) -> (String, &'static str) {
        let err = parse_order_line(line).unwrap_err();
        (err.to_string(), error_kind(err.as_ref()))
    }

    #[test]
    fn valid_line() {
        let order = parse_order_line("apple, 3, 0.5").unwrap();
        assert_eq!(
            order,
            OrderLine {
                item: "apple".to_string(),
                quantity: 3,
                price: 0.5
            }
        );
        assert_eq!(parse_order_line(" kiwi ,0,  2").unwrap().quantity, 0);
    }

    #[test]
    fn text_errors_are_other() {
        assert_eq!(
            failure("apple, 3"),
            ("expected 3 fields, found 2".to_string(), "other")
        );
        assert_eq!(
            failure("pear, 2, 1,5"),
            ("expected 3 fields, found 4".to_string(), "other")
        );
        assert_eq!(failure(" , 2, 1"), ("empty item name".to_string(), "other"));
    }

    #[test]
    fn library_errors_are_boxed_by_question_mark() {
        assert_eq!(
            failure("pear, x, 1"),
            ("invalid digit found in string".to_string(), "integer")
        );
        assert_eq!(
            failure("pear, 2, cheap"),
            ("invalid float literal".to_string(), "float")
        );
    }

    #[test]
    fn custom_error_can_be_downcast() {
        assert_eq!(
            failure("pear, -2, 1"),
            ("quantity cannot be negative: -2".to_string(), "negative")
        );
        let err = parse_order_line("pear, -7, x").unwrap_err();
        assert_eq!(
            err.downcast_ref::<NegativeQuantity>(),
            Some(&NegativeQuantity(-7))
        );
    }

    #[test]
    fn totals() {
        let total = order_total("apple, 3, 0.5\n\npear, 2, 1.25").unwrap();
        assert!((total - 4.0).abs() < 1e-9);
        assert_eq!(order_total("").unwrap(), 0.0);
        let err = order_total("apple, 3, 0.5\nbanana, many, 1").unwrap_err();
        assert_eq!(error_kind(err.as_ref()), "integer");
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** let `?` box every library error and create the remaining errors from text or from our
own `NegativeQuantity` type. All failures share one return type, `Box<dyn Error>`, which is
easy to propagate (`order_total` just uses `?`) and can still be inspected with downcasting.

**Rust concepts:**
- `?` converts any `E: Error` into `Box<dyn Error>` through a blanket `From` implementation.
- `Box<dyn Error>` erases the concrete type; `downcast_ref::<T>()` / `is::<T>()` recover it at
  run time. They require `dyn Error + 'static`, which `Box<dyn Error>` is by default.
- The trade-off: callers cannot `match` on a closed set of variants as with an error enum.
  Boxed errors are great for applications and prototypes; libraries usually prefer enums.
- `let &[a, b, c] = fields.as_slice() else { ... }` binds exactly three fields.

```rust
use std::error::Error;
use std::fmt;
use std::num::{ParseFloatError, ParseIntError};

#[derive(Debug, PartialEq)]
pub struct NegativeQuantity(pub i64);

#[derive(Debug, PartialEq)]
pub struct OrderLine {
    pub item: String,
    pub quantity: i64,
    pub price: f64,
}

impl fmt::Display for NegativeQuantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "quantity cannot be negative: {}", self.0)
    }
}

impl Error for NegativeQuantity {}

pub fn parse_order_line(line: &str) -> Result<OrderLine, Box<dyn Error>> {
    let fields: Vec<&str> = line.split(',').map(str::trim).collect();
    let &[item, quantity, price] = fields.as_slice() else {
        return Err(format!("expected 3 fields, found {}", fields.len()).into());
    };
    if item.is_empty() {
        return Err("empty item name".into());
    }
    let quantity: i64 = quantity.parse()?;
    if quantity < 0 {
        return Err(Box::new(NegativeQuantity(quantity)));
    }
    let price: f64 = price.parse()?;
    Ok(OrderLine {
        item: item.to_string(),
        quantity,
        price,
    })
}

pub fn order_total(text: &str) -> Result<f64, Box<dyn Error>> {
    let mut total = 0.0;
    for line in text.lines().filter(|l| !l.trim().is_empty()) {
        let order = parse_order_line(line)?;
        total += order.quantity as f64 * order.price;
    }
    Ok(total)
}

pub fn error_kind(err: &(dyn Error + 'static)) -> &'static str {
    if err.is::<ParseIntError>() {
        "integer"
    } else if err.is::<ParseFloatError>() {
        "float"
    } else if err.is::<NegativeQuantity>() {
        "negative"
    } else {
        "other"
    }
}
```

Full tested solution: [`src/solutions/p0427_box_dyn_error.rs`](../src/solutions/p0427_box_dyn_error.rs)

</details>
