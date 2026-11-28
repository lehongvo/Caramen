# Problem 0289: Basic Calculator II

- **Difficulty:** Medium
- **Topics:** tokenizing into an enum, operator precedence with a "pending operator" state, `match` on `(Op, value)`, a stack of terms
- **Source:** [LeetCode 227. Basic Calculator II](https://leetcode.com/problems/basic-calculator-ii/)

## Problem

Evaluate an expression string `s` made of non-negative integers, the operators `+ - * /` and
spaces. There are no parentheses. The usual precedence applies: `*` and `/` bind tighter than
`+` and `-`, and operators of the same precedence are evaluated from left to right. Integer
division truncates toward zero. You may **not** use any built-in "eval" facility.

Implement `Solution::calculate(s: String) -> i32`.

For this level, first turn the string into tokens (`enum Token { Num(i32), Op(Op) }`), then
evaluate the tokens.

## Examples

```text
Input:  s = "3+2*2"
Output: 7

Input:  s = " 3/2 "
Output: 1

Input:  s = " 3+5 / 2 "
Output: 5        (5 / 2 = 2 is computed first)
```

## Constraints

- `1 <= s.len() <= 3 * 10^5`; the expression is valid.
- All numbers are in `[0, 2^31 - 1]`, and the answer and every intermediate value fit in an `i32`.

## Hints

1. Keep a stack of **terms** that will be added at the end. Remember the operator that came
   before the current number (start with `+`).
2. When a number arrives: after `+` push it, after `-` push its negation, after `*` or `/`
   replace the top of the stack with `top * n` or `top / n`.

## Starter code

Write your code in [`src/practice/p0289_basic_calculator_ii.rs`](../src/practice/p0289_basic_calculator_ii.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0289
```

```rust
pub struct Solution;

impl Solution {
    pub fn calculate(s: String) -> i32 {
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

    fn calc(s: &str) -> i32 {
        Solution::calculate(s.to_string())
    }

    #[test]
    fn example_precedence() {
        assert_eq!(calc("3+2*2"), 7);
    }

    #[test]
    fn example_division() {
        assert_eq!(calc(" 3/2 "), 1);
    }

    #[test]
    fn example_mixed() {
        assert_eq!(calc(" 3+5 / 2 "), 5);
    }

    #[test]
    fn left_to_right_and_negative_terms() {
        assert_eq!(calc("14-3/2"), 13);
        assert_eq!(calc("2*3*4/5"), 4);
        assert_eq!(calc("1-1-1"), -1);
        assert_eq!(calc("1-3*2"), -5);
        assert_eq!(calc("0-7/2"), -3);
    }

    #[test]
    fn large_numbers_and_long_input() {
        assert_eq!(calc("2147483647"), i32::MAX);
        assert_eq!(calc("2147483647 - 2147483647 * 1"), 0);
        let long = "1 + ".repeat(50_000) + "1";
        assert_eq!(calc(&long), 50_001);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** precedence is handled by deciding *when* to apply each operator. Additive operators
only create a new term on the stack; multiplicative ones combine immediately with the last term.
The result is the sum of the terms.

**Complexity:** `O(n)` time, `O(n)` space.

**Rust concepts:**
- The tokenizer accumulates digits with `while let Some(d) = chars.next_if(char::is_ascii_digit)`,
  a `Peekable` method that only consumes matching characters.
- The evaluator's state is just `pending: Op` and the stack; `match pending` decides what a
  number does.
- `if let Some(top) = terms.last_mut() { *top *= n; }` edits the last term in place.

```rust
pub struct Solution;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Token {
    Num(i32),
    Op(Op),
}

fn tokenize(s: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        let token = match c {
            '0'..='9' => {
                let mut n = c as i32 - '0' as i32;
                while let Some(d) = chars.next_if(char::is_ascii_digit) {
                    n = n * 10 + (d as i32 - '0' as i32);
                }
                Token::Num(n)
            }
            '+' => Token::Op(Op::Add),
            '-' => Token::Op(Op::Sub),
            '*' => Token::Op(Op::Mul),
            '/' => Token::Op(Op::Div),
            _ => continue,
        };
        tokens.push(token);
    }
    tokens
}

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut terms: Vec<i32> = Vec::new();
        let mut pending = Op::Add;
        for token in tokenize(&s) {
            match token {
                Token::Op(op) => pending = op,
                Token::Num(n) => match pending {
                    Op::Add => terms.push(n),
                    Op::Sub => terms.push(-n),
                    Op::Mul => {
                        if let Some(top) = terms.last_mut() {
                            *top *= n;
                        }
                    }
                    Op::Div => {
                        if let Some(top) = terms.last_mut() {
                            *top /= n;
                        }
                    }
                },
            }
        }
        terms.iter().sum()
    }
}
```

Full tested solution: [`src/solutions/p0289_basic_calculator_ii.rs`](../src/solutions/p0289_basic_calculator_ii.rs)

</details>
