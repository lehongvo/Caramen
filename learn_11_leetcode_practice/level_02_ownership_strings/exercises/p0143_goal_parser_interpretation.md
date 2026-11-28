# Problem 0143: Goal Parser Interpretation

- **Difficulty:** Easy
- **Topics:** scanning `as_bytes()` with an index, `match` with guards, `slice::get`, `str::replace`
- **Source:** [LeetCode 1678. Goal Parser Interpretation](https://leetcode.com/problems/goal-parser-interpretation/)

## Problem

A command string is made only of the tokens `"G"`, `"()"` and `"(al)"`, written one after another.
Interpret it by replacing every token:

- `"G"` becomes `"G"`;
- `"()"` becomes `"o"`;
- `"(al)"` becomes `"al"`.

Return the interpreted string, with the tokens in their original order.

Implement `Solution::interpret(command: String) -> String`.

## Examples

```text
Input:  command = "G()(al)"
Output: "Goal"

Input:  command = "G()()()()(al)"
Output: "Gooooal"

Input:  command = "(al)G(al)()()G"
Output: "alGalooG"
```

## Constraints

- `1 <= command.len() <= 100`
- `command` is a valid sequence of `"G"`, `"()"` and `"(al)"` tokens.

## Hints

1. Look at one byte at a time: `G` is a whole token; `(` starts either `()` or `(al)`, and the
   next byte tells you which.
2. Advance the index by the length of the token you just read (1, 2 or 4).
3. The short version chains replacements: `command.replace("()", "o").replace("(al)", "al")`
   (each call allocates a new `String`).

## Starter code

Write your code in [`src/practice/p0143_goal_parser_interpretation.rs`](../src/practice/p0143_goal_parser_interpretation.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0143
```

```rust
pub struct Solution;

impl Solution {
    pub fn interpret(command: String) -> String {
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

    fn run(command: &str) -> String {
        Solution::interpret(command.to_string())
    }

    #[test]
    fn example_goal() {
        assert_eq!(run("G()(al)"), "Goal");
    }

    #[test]
    fn example_many_o() {
        assert_eq!(run("G()()()()(al)"), "Gooooal");
    }

    #[test]
    fn example_mixed_order() {
        assert_eq!(run("(al)G(al)()()G"), "alGalooG");
    }

    #[test]
    fn single_tokens() {
        assert_eq!(run("G"), "G");
        assert_eq!(run("()"), "o");
        assert_eq!(run("(al)"), "al");
    }

    #[test]
    fn longest_input_matches_replace() {
        let command = "G()(al)()".repeat(11);
        let expected = command.replace("()", "o").replace("(al)", "al");
        assert_eq!(run(&command), expected);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** a single left-to-right scan with an index. The byte at the index identifies the token;
push its translation and jump over the token. `bytes.get(i + 1)` peeks at the next byte without
risking an out-of-bounds panic.

**Complexity:** `O(n)` time and space.

**Rust concepts:**
- `as_bytes()` borrows the string as `&[u8]`; byte literals such as `b'('` are `u8` constants.
- Match guards (`b'(' if ...`) refine an arm with an extra condition.
- `slice::get(i)` returns `Option<&u8>`, which compares directly with `Some(&b')')`.

```rust
pub struct Solution;

impl Solution {
    pub fn interpret(command: String) -> String {
        let bytes = command.as_bytes();
        let mut out = String::with_capacity(command.len());
        let mut i = 0;
        while i < bytes.len() {
            match bytes[i] {
                b'G' => {
                    out.push('G');
                    i += 1;
                }
                b'(' if bytes.get(i + 1) == Some(&b')') => {
                    out.push('o');
                    i += 2;
                }
                // The only remaining token is "(al)".
                _ => {
                    out.push_str("al");
                    i += 4;
                }
            }
        }
        out
    }
}
```

Full tested solution: [`src/solutions/p0143_goal_parser_interpretation.rs`](../src/solutions/p0143_goal_parser_interpretation.rs)

</details>
