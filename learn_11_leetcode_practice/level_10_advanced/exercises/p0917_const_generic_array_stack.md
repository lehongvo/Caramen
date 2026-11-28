# Problem 0917: A Fixed-Capacity Stack

- **Difficulty:** Medium
- **Topics:** const generics `<T, const N: usize>`, `[Option<T>; N]`, `std::array::from_fn`, associated consts, `Result<(), T>` to give a value back
- **Source:** Original · [Reference: Const generics](https://doc.rust-lang.org/reference/items/generics.html#const-generics) · [std: `array::from_fn`](https://doc.rust-lang.org/std/array/fn.from_fn.html)

## Problem

Embedded and real-time code often cannot allocate. Build a stack whose storage is an array that
lives **inside** the struct, with the capacity chosen at compile time:

```text
pub struct ArrayStack<T, const N: usize> { items: [Option<T>; N], len: usize }
```

- `ArrayStack::<T, N>::CAPACITY` is an associated `const` equal to `N`.
- `new()` (and `Default`): an empty stack. `T` does **not** need to be `Copy` or `Default`.
- `push(&mut self, value: T) -> Result<(), T>`: when the stack is full, hand the value back as
  `Err(value)` instead of losing it.
- `pop(&mut self) -> Option<T>`, `peek(&self) -> Option<&T>`.
- `len()`, `is_empty()`, `is_full()`.
- `iter(&self) -> impl Iterator<Item = &T>`: bottom to top.
- `from_array(items: [T; N]) -> ArrayStack<T, N>`: a **full** stack whose top is the last element.

Then use it in `eval_rpn<const DEPTH: usize>(tokens: &[&str]) -> Result<i64, RpnError>`, which
evaluates Reverse Polish Notation with integers and `+ - * /` (integer division truncating toward
zero) using an `ArrayStack<i64, DEPTH>`. Errors, checked while scanning from left to right:

- `RpnError::Overflow` when a number does not fit in the stack;
- `RpnError::Underflow` when an operator finds fewer than two operands;
- `RpnError::DivisionByZero`;
- `RpnError::BadToken(token)` for anything that is neither an operator nor an `i64`;
- at the end, `RpnError::Leftover(n)` if the stack holds `n != 1` values (including `0`).

## Examples

```text
let mut s: ArrayStack<String, 2> = ArrayStack::new();
s.push("a".into()) => Ok(());  s.push("b".into()) => Ok(());  s.push("c".into()) => Err("c")
eval_rpn::<4>(&["2", "3", "4", "*", "+"])  => Ok(14)
eval_rpn::<2>(&["1", "2", "3", "+", "+"])  => Err(Overflow)
eval_rpn::<4>(&["7", "-2", "/"])           => Ok(-3)
```

## Constraints

- Intermediate RPN results fit in `i64`.

## Hints

1. `std::array::from_fn(|_| None)` builds `[Option<T>; N]` for any `T`.
2. `Option::take` moves the top value out of its slot.
3. `from_array` can turn each element into `Some(element)` with `items.map(Some)`.

## Starter code

Write your code in [`src/practice/p0917_const_generic_array_stack.rs`](../src/practice/p0917_const_generic_array_stack.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0917
```

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RpnError {
    Overflow,
    Underflow,
    DivisionByZero,
    BadToken(String),
    Leftover(usize),
}

#[derive(Debug, Clone)]
pub struct ArrayStack<T, const N: usize> {
    items: [Option<T>; N],
    len: usize,
}

impl<T, const N: usize> Default for ArrayStack<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> ArrayStack<T, N> {
    pub const CAPACITY: usize = N;

    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn from_array(items: [T; N]) -> Self {
        todo!("Write your solution here")
    }

    pub fn push(&mut self, value: T) -> Result<(), T> {
        todo!("Write your solution here")
    }

    pub fn pop(&mut self) -> Option<T> {
        todo!("Write your solution here")
    }

    pub fn peek(&self) -> Option<&T> {
        todo!("Write your solution here")
    }

    pub fn len(&self) -> usize {
        todo!("Write your solution here")
    }

    pub fn is_empty(&self) -> bool {
        todo!("Write your solution here")
    }

    pub fn is_full(&self) -> bool {
        todo!("Write your solution here")
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        std::iter::from_fn(|| todo!())
    }
}

pub fn eval_rpn<const DEPTH: usize>(tokens: &[&str]) -> Result<i64, RpnError> {
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
    fn push_returns_the_value_when_full() {
        let mut s: ArrayStack<String, 2> = ArrayStack::new();
        assert_eq!(ArrayStack::<String, 2>::CAPACITY, 2);
        assert_eq!(s.push("a".to_string()), Ok(()));
        assert_eq!(s.push("b".to_string()), Ok(()));
        assert!(s.is_full());
        assert_eq!(s.push("c".to_string()), Err("c".to_string()));
        assert_eq!(s.peek().map(String::as_str), Some("b"));
        assert_eq!(s.pop(), Some("b".to_string()));
        assert_eq!(s.len(), 1);
    }

    #[test]
    fn iter_and_from_array() {
        let s = ArrayStack::from_array([1, 2, 3]);
        assert!(s.is_full());
        assert_eq!(s.iter().copied().collect::<Vec<_>>(), vec![1, 2, 3]);
        assert_eq!(s.peek(), Some(&3));
        let mut empty: ArrayStack<u8, 3> = ArrayStack::default();
        assert!(empty.is_empty());
        assert_eq!((empty.pop(), empty.peek()), (None, None));
        assert_eq!(empty.iter().count(), 0);
    }

    #[test]
    fn zero_capacity_stack() {
        let mut s: ArrayStack<char, 0> = ArrayStack::new();
        assert!(s.is_empty() && s.is_full());
        assert_eq!(s.push('x'), Err('x'));
    }

    #[test]
    fn rpn_success() {
        assert_eq!(eval_rpn::<4>(&["2", "3", "4", "*", "+"]), Ok(14));
        assert_eq!(eval_rpn::<4>(&["7", "-2", "/"]), Ok(-3));
        assert_eq!(eval_rpn::<2>(&["10", "4", "-", "3", "*"]), Ok(18));
        assert_eq!(eval_rpn::<1>(&["42"]), Ok(42));
    }

    #[test]
    fn rpn_errors() {
        assert_eq!(
            eval_rpn::<2>(&["1", "2", "3", "+", "+"]),
            Err(RpnError::Overflow)
        );
        assert_eq!(eval_rpn::<4>(&["1", "+"]), Err(RpnError::Underflow));
        assert_eq!(
            eval_rpn::<4>(&["1", "0", "/"]),
            Err(RpnError::DivisionByZero)
        );
        assert_eq!(
            eval_rpn::<4>(&["1", "x", "+"]),
            Err(RpnError::BadToken("x".to_string()))
        );
        assert_eq!(eval_rpn::<4>(&["1", "2"]), Err(RpnError::Leftover(2)));
        assert_eq!(eval_rpn::<4>(&[]), Err(RpnError::Leftover(0)));
    }

    #[test]
    fn deep_expression_needs_a_deep_stack() {
        // 1 1 1 ... 1 (100 times) then 99 "+" operators: needs a depth of 100.
        let mut tokens = vec!["1"; 100];
        tokens.extend(vec!["+"; 99]);
        assert_eq!(eval_rpn::<100>(&tokens), Ok(100));
        assert_eq!(eval_rpn::<99>(&tokens), Err(RpnError::Overflow));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** slots `0..len` are `Some`, the rest are `None`. `push` writes into slot `len`, `pop`
takes slot `len - 1`. `eval_rpn` pushes numbers and, for each operator, pops the right operand,
then the left one, and pushes the result; `push` failing becomes `Overflow`.

**Complexity:** every stack operation is `O(1)`; `eval_rpn` is `O(tokens)`; memory is fixed at
`N` slots.

**Rust concepts:**
- A const generic parameter is a value known at compile time; `ArrayStack<i64, 4>` and
  `ArrayStack<i64, 8>` are different types.
- `[None; N]` would require `Option<T>: Copy`; `std::array::from_fn` builds the array element by
  element instead.
- Returning `Result<(), T>` gives ownership of a rejected value back to the caller.
- Calls such as `eval_rpn::<4>(...)` pass the const argument with the turbofish.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RpnError {
    Overflow,
    Underflow,
    DivisionByZero,
    BadToken(String),
    Leftover(usize),
}

#[derive(Debug, Clone)]
pub struct ArrayStack<T, const N: usize> {
    items: [Option<T>; N],
    len: usize,
}

impl<T, const N: usize> Default for ArrayStack<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> ArrayStack<T, N> {
    pub const CAPACITY: usize = N;

    pub fn new() -> Self {
        ArrayStack {
            items: std::array::from_fn(|_| None),
            len: 0,
        }
    }

    pub fn from_array(items: [T; N]) -> Self {
        ArrayStack {
            items: items.map(Some),
            len: N,
        }
    }

    pub fn push(&mut self, value: T) -> Result<(), T> {
        if self.len == N {
            return Err(value);
        }
        self.items[self.len] = Some(value);
        self.len += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        self.items[self.len].take()
    }

    pub fn peek(&self) -> Option<&T> {
        self.len
            .checked_sub(1)
            .and_then(|top| self.items[top].as_ref())
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn is_full(&self) -> bool {
        self.len == N
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items[..self.len].iter().flatten()
    }
}

pub fn eval_rpn<const DEPTH: usize>(tokens: &[&str]) -> Result<i64, RpnError> {
    let mut stack: ArrayStack<i64, DEPTH> = ArrayStack::new();
    for &token in tokens {
        let value = match token {
            "+" | "-" | "*" | "/" => {
                let right = stack.pop().ok_or(RpnError::Underflow)?;
                let left = stack.pop().ok_or(RpnError::Underflow)?;
                match token {
                    "+" => left + right,
                    "-" => left - right,
                    "*" => left * right,
                    _ => left.checked_div(right).ok_or(RpnError::DivisionByZero)?,
                }
            }
            number => number
                .parse::<i64>()
                .map_err(|_| RpnError::BadToken(number.to_string()))?,
        };
        stack.push(value).map_err(|_| RpnError::Overflow)?;
    }
    match (stack.pop(), stack.len()) {
        (Some(result), 0) => Ok(result),
        (None, _) => Err(RpnError::Leftover(0)),
        (Some(_), rest) => Err(RpnError::Leftover(rest + 1)),
    }
}
```

Full tested solution: [`src/solutions/p0917_const_generic_array_stack.rs`](../src/solutions/p0917_const_generic_array_stack.rs)

</details>
