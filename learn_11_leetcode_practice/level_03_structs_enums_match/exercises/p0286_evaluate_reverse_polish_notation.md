# Problem 0286: Evaluate Reverse Polish Notation

- **Difficulty:** Medium
- **Topics:** parsing strings into a `Token` enum, an `Op` enum with an `apply` method, a stack, slice patterns on the stack
- **Source:** [LeetCode 150. Evaluate Reverse Polish Notation](https://leetcode.com/problems/evaluate-reverse-polish-notation/)

## Problem

In **Reverse Polish Notation** (postfix notation) an operator comes after its two operands:
`3 4 +` means `3 + 4`, and `2 1 + 3 *` means `(2 + 1) * 3`. No parentheses are ever needed.

You are given the expression as a list of tokens: integers (possibly negative, like `"-11"`) and
the operators `"+"`, `"-"`, `"*"`, `"/"`. Division between integers **truncates toward zero**.
The expression is always valid, never divides by zero, and every intermediate result fits in a
32-bit integer. Return its value.

Implement `Solution::eval_rpn(tokens: Vec<String>) -> i32`.

For this level, parse each string into `enum Token { Num(i32), Op(Op) }` with
`enum Op { Add, Sub, Mul, Div }`, and give `Op` a method `apply(self, a: i32, b: i32) -> i32`.

## Examples

```text
Input:  tokens = ["2", "1", "+", "3", "*"]
Output: 9       ((2 + 1) * 3)

Input:  tokens = ["4", "13", "5", "/", "+"]
Output: 6       (4 + 13 / 5 = 4 + 2)

Input:  tokens = ["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"]
Output: 22
```

## Constraints

- `1 <= tokens.len() <= 10^4`
- Each token is an operator or an integer in `[-200, 200]`.

## Hints

1. `"-"` alone is an operator but `"-11"` is a number: match the four operator strings first,
   then try `parse::<i32>()` on everything else.
2. Push numbers on a stack. For an operator, pop `b` first, then `a`, and push `op.apply(a, b)`.

## Starter code

Write your code in [`src/practice/p0286_evaluate_reverse_polish_notation.rs`](../src/practice/p0286_evaluate_reverse_polish_notation.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0286
```

```rust
pub struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
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

    fn rpn(tokens: &[&str]) -> i32 {
        Solution::eval_rpn(tokens.iter().map(|t| t.to_string()).collect())
    }

    #[test]
    fn example_one() {
        assert_eq!(rpn(&["2", "1", "+", "3", "*"]), 9);
    }

    #[test]
    fn example_two() {
        assert_eq!(rpn(&["4", "13", "5", "/", "+"]), 6);
    }

    #[test]
    fn example_three() {
        assert_eq!(
            rpn(&[
                "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"
            ]),
            22
        );
    }

    #[test]
    fn subtraction_order_and_truncation() {
        assert_eq!(rpn(&["3", "5", "-"]), -2);
        assert_eq!(rpn(&["-7", "2", "/"]), -3);
        assert_eq!(rpn(&["7", "-2", "/"]), -3);
        assert_eq!(rpn(&["42"]), 42);
        assert_eq!(rpn(&["-200"]), -200);
    }

    #[test]
    fn long_expression() {
        // 1 1 + 1 + 1 + ... : 5_000 ones summed.
        let mut tokens = vec!["1"];
        for _ in 0..4_999 {
            tokens.push("1");
            tokens.push("+");
        }
        assert_eq!(rpn(&tokens), 5_000);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** a stack machine. Numbers are pushed; an operator pops the two most recent values,
applies itself and pushes the result. The single value left at the end is the answer.

**Complexity:** `O(n)` time, `O(n)` space.

**Rust concepts:**
- The `match` on `&str` lists the operators, and its last arm binds the text (`num => ...`) to
  parse it as a number.
- `Op::apply` is an exhaustive `match`; Rust's `/` on integers already truncates toward zero.
- `let (Some(b), Some(a)) = (stack.pop(), stack.pop()) else { ... }` pops both operands and
  handles a malformed expression without panicking. Tuple fields are evaluated left to right,
  so `b` is popped first.

```rust
pub struct Solution;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn apply(self, a: i32, b: i32) -> i32 {
        match self {
            Op::Add => a + b,
            Op::Sub => a - b,
            Op::Mul => a * b,
            Op::Div => a / b,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Token {
    Num(i32),
    Op(Op),
}

impl Token {
    fn parse(s: &str) -> Option<Token> {
        match s {
            "+" => Some(Token::Op(Op::Add)),
            "-" => Some(Token::Op(Op::Sub)),
            "*" => Some(Token::Op(Op::Mul)),
            "/" => Some(Token::Op(Op::Div)),
            num => num.parse().ok().map(Token::Num),
        }
    }
}

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::with_capacity(tokens.len());
        for token in tokens.iter().filter_map(|t| Token::parse(t)) {
            match token {
                Token::Num(n) => stack.push(n),
                Token::Op(op) => {
                    let (Some(b), Some(a)) = (stack.pop(), stack.pop()) else {
                        // Not reachable for valid input.
                        return 0;
                    };
                    stack.push(op.apply(a, b));
                }
            }
        }
        stack.pop().unwrap_or(0)
    }
}
```

Full tested solution: [`src/solutions/p0286_evaluate_reverse_polish_notation.rs`](../src/solutions/p0286_evaluate_reverse_polish_notation.rs)

</details>
