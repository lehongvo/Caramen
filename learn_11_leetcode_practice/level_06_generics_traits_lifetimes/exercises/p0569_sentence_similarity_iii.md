# Problem 0569: Sentence Similarity III

- **Difficulty:** Medium
- **Topics:** `Vec<&str>` word slices, comparing slices with `==`, common prefix/suffix with `zip` and `take_while`
- **Source:** [LeetCode 1813. Sentence Similarity III](https://leetcode.com/problems/sentence-similarity-iii/)

## Problem

Two sentences (words separated by single spaces) are **similar** if you can insert an arbitrary
sentence, possibly empty, at **one** position of one of them (between two words, at the start or at
the end) so that the two become equal. Insertions must consist of whole words: `"Hello"` and
`"Helloworld"` are not similar.

Return whether `sentence1` and `sentence2` are similar.

Implement `Solution::are_sentences_similar(sentence1: String, sentence2: String) -> bool`.

## Examples

```text
Input:  "My name is Haley", "My Haley"        Output: true   (insert "name is" into the second)
Input:  "of", "A lot of words"                Output: false  (would need two insertions)
Input:  "Eating right now", "Eating"          Output: true   (insert "right now" at the end)
Input:  "Luky", "Lucccky"                     Output: false
```

## Constraints

- `1 <= sentence1.len(), sentence2.len() <= 100`
- Sentences contain English letters and single spaces, with no leading or trailing spaces.

## Hints

1. Let `short` be the sentence with fewer words. The insertion goes into the other one, so `short`
   must equal a prefix of `long` followed by a suffix of `long`.
2. Count the common words at the start (`p`) and at the end (`s`). The answer is `p + s >=
   short.len()`; make sure not to count the same word of `short` twice.

## Starter code

Write your code in [`src/practice/p0569_sentence_similarity_iii.rs`](../src/practice/p0569_sentence_similarity_iii.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0569
```

```rust
pub struct Solution;

impl Solution {
    pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
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

    fn similar(a: &str, b: &str) -> bool {
        Solution::are_sentences_similar(a.to_string(), b.to_string())
    }

    #[test]
    fn example_inputs() {
        assert!(similar("My name is Haley", "My Haley"));
        assert!(!similar("of", "A lot of words"));
        assert!(similar("Eating right now", "Eating"));
        assert!(!similar("Luky", "Lucccky"));
    }

    #[test]
    fn insertion_at_the_start_or_end() {
        assert!(similar("now", "right now"));
        assert!(similar("right now", "now"));
        assert!(similar("a", "a"));
        assert!(!similar("Hello", "Helloworld"));
    }

    #[test]
    fn repeated_words_are_not_double_counted() {
        assert!(similar("A A", "A A A"));
        assert!(!similar("A B", "A C B D"));
        assert!(similar("x y z", "x y q z"));
        assert!(similar("c h p Ny", "c BDQ r h p Ny")); // insert "BDQ r" after "c"
        assert!(!similar("A B C", "A X B Y C"));
    }

    #[test]
    fn longer_sentences() {
        let base: Vec<String> = (0..20).map(|i| format!("w{i}")).collect();
        let mut with_gap = base[..10].to_vec();
        with_gap.extend(["extra".to_string(), "words".to_string()]);
        with_gap.extend_from_slice(&base[10..]);
        assert!(similar(&base.join(" "), &with_gap.join(" ")));
        let mut two_gaps = with_gap.clone();
        two_gaps.insert(3, "oops".to_string());
        assert!(!similar(&base.join(" "), &two_gaps.join(" ")));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** split both sentences into word slices. Measure the common prefix, then the common suffix of
the remaining words. The sentences are similar when prefix and suffix together cover the shorter
sentence.

**Complexity:** `O(n + m)` time and space.

**Rust concepts:**
- `Vec<&str>` holds slices of the input `String`s; comparisons such as `a == b` on `&str` compare
  contents.
- `zip(..).take_while(|(a, b)| a == b).count()` measures a common prefix in one expression; reversing
  both iterators measures the common suffix.
- `std::mem::swap` on the two vectors avoids duplicating the logic for "which one is shorter".

```rust
pub struct Solution;

impl Solution {
    pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
        let mut short: Vec<&str> = sentence1.split(' ').collect();
        let mut long: Vec<&str> = sentence2.split(' ').collect();
        if short.len() > long.len() {
            std::mem::swap(&mut short, &mut long);
        }
        let prefix = short.iter().zip(&long).take_while(|(a, b)| a == b).count();
        let suffix = short[prefix..]
            .iter()
            .rev()
            .zip(long.iter().rev())
            .take_while(|(a, b)| a == b)
            .count();
        prefix + suffix >= short.len()
    }
}
```

Full tested solution: [`src/solutions/p0569_sentence_similarity_iii.rs`](../src/solutions/p0569_sentence_similarity_iii.rs)

</details>
