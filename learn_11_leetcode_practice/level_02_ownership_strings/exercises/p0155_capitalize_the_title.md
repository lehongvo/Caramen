# Problem 0155: Capitalize the Title

- **Difficulty:** Easy
- **Topics:** case conversion on sub-slices (`out[a..b].make_ascii_lowercase()`), `split(' ')`, building one `String`
- **Source:** [LeetCode 2129. Capitalize the Title](https://leetcode.com/problems/capitalize-the-title/)

## Problem

A title is made of words separated by single spaces. Capitalize it with these rules:

- a word of length 1 or 2 is written entirely in lowercase;
- a longer word gets an uppercase first letter and lowercase remaining letters.

Return the capitalized title, with the same words in the same order separated by single spaces.

Implement `Solution::capitalize_title(title: String) -> String`.

## Examples

```text
Input:  title = "capiTalIze tHe titLe"
Output: "Capitalize The Title"

Input:  title = "First leTTeR of EACH Word"
Output: "First Letter of Each Word"

Input:  title = "i lOve leetcode"
Output: "i Love Leetcode"
```

## Constraints

- `1 <= title.len() <= 100`
- `title` consists of English letters and single spaces, with no leading or trailing space.

## Hints

1. Copy each word into the output, then fix its case in place: `out[start..]` is a `&mut str`
   on which you can call `make_ascii_lowercase()`.
2. For long words, upper-case just `out[start..start + 1]`.

## Starter code

Write your code in [`src/practice/p0155_capitalize_the_title.rs`](../src/practice/p0155_capitalize_the_title.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0155
```

```rust
pub struct Solution;

impl Solution {
    pub fn capitalize_title(title: String) -> String {
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

    fn run(title: &str) -> String {
        Solution::capitalize_title(title.to_string())
    }

    #[test]
    fn example_three_long_words() {
        assert_eq!(run("capiTalIze tHe titLe"), "Capitalize The Title");
    }

    #[test]
    fn example_short_word_in_the_middle() {
        assert_eq!(
            run("First leTTeR of EACH Word"),
            "First Letter of Each Word"
        );
    }

    #[test]
    fn example_one_letter_word() {
        assert_eq!(run("i lOve leetcode"), "i Love Leetcode");
    }

    #[test]
    fn only_short_words() {
        assert_eq!(run("A Bc DE"), "a bc de");
    }

    #[test]
    fn three_letter_boundary() {
        assert_eq!(run("aBC xY"), "Abc xy");
    }

    #[test]
    fn longest_title() {
        let title = "wORD ".repeat(20);
        let result = run(title.trim_end());
        assert_eq!(result, vec!["Word"; 20].join(" "));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** append every word to one output `String`, remember where it starts, then lowercase that
part and, for words longer than two letters, uppercase its first byte. Everything happens inside
the output buffer; no per-word `String` is created.

**Complexity:** `O(n)` time and space.

**Rust concepts:**
- Indexing a `String` with a range gives a `str` place; calling a `&mut self` method on it
  (`out[a..b].make_ascii_uppercase()`) borrows that part mutably through `IndexMut`.
- ASCII case changes keep the byte length, which is why they are available in place on `&mut str`.
- `out.len()` before pushing a word is exactly the byte offset where the word starts.

```rust
pub struct Solution;

impl Solution {
    pub fn capitalize_title(title: String) -> String {
        let mut out = String::with_capacity(title.len());
        for (i, word) in title.split(' ').enumerate() {
            if i > 0 {
                out.push(' ');
            }
            let start = out.len();
            out.push_str(word);
            out[start..].make_ascii_lowercase();
            if word.len() > 2 {
                out[start..start + 1].make_ascii_uppercase();
            }
        }
        out
    }
}
```

Full tested solution: [`src/solutions/p0155_capitalize_the_title.rs`](../src/solutions/p0155_capitalize_the_title.rs)

</details>
