# Level 2: Ownership & Strings (problems 0101-0200)

> Focus: ownership, moves, borrowing, slices, `String` vs `&str`, chars and bytes.

This level covers chapter 4 (ownership) and section 8.2 (strings) of The Rust Book: who owns a
value, when it moves, how to borrow it, and how to work with UTF-8 text through `String` and `&str`.
You modify vectors and strings in place through `&mut`, return slices that borrow from your inputs,
and fix 11 small programs that the borrow checker rejects. 55 problems come from LeetCode (43 Easy
and 12 Medium, using LeetCode's exact Rust signatures) and focus on strings and in-place arrays; the
other 45 are original problems, each built around one ownership or string feature.

## Prerequisites

- Level 1, or the same knowledge: functions, `if`/`match`, loops, `Vec` and basic slices.
- Read [The Rust Book 4.1-4.3](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
  and [8.2](https://doc.rust-lang.org/book/ch08-02-strings.html) before or alongside this level.
- `Option` appears in some signatures (`Some(x)` / `None`); a basic `match` on it is all you need.

## What you will practice

- Moves, ownership transfer into and out of functions, `Clone` vs `Copy`:
  [Book 4.1](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html),
  [`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html),
  [`Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html).
- Shared and mutable references, the borrowing rules, and dangling references:
  [Book 4.2](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html).
- Moving out of a `&mut` with [`std::mem::take`](https://doc.rust-lang.org/std/mem/fn.take.html) and
  [`std::mem::swap`](https://doc.rust-lang.org/std/mem/fn.swap.html).
- Slices `&[T]` and `&mut [T]`: ranges, `windows`, `chunks_mut`, `split_at_mut`, `swap`, `reverse`,
  and functions that return sub-slices of their input:
  [Book 4.3](https://doc.rust-lang.org/book/ch04-03-slices.html),
  [slice methods](https://doc.rust-lang.org/std/primitive.slice.html).
- In-place algorithms on `&mut Vec<T>`: read/write indices, two pointers, filling from the back,
  three-reversal rotation.
- `String` vs `&str`, building strings with `push`, `push_str`, `+`, `format!`, `join` and `repeat`:
  [Book 8.2](https://doc.rust-lang.org/book/ch08-02-strings.html),
  [`String`](https://doc.rust-lang.org/std/string/struct.String.html),
  [`std::fmt`](https://doc.rust-lang.org/std/fmt/index.html).
- `chars()`, `bytes()`, `char_indices()`, `as_bytes()`, byte offsets vs character positions,
  `is_char_boundary`, and why `s[0]` does not compile:
  [`str` methods](https://doc.rust-lang.org/std/primitive.str.html),
  [`char` methods](https://doc.rust-lang.org/std/primitive.char.html).
- Case conversion (ASCII and full Unicode), `trim`, `split_whitespace`, `split_once`, `strip_prefix`.
- Choosing `&str` or `String` for parameters and return values.
- Reading compiler errors E0382, E0106, E0594, E0507, E0502, E0499, E0308, E0277 and E0515, and
  fixing them: [error code index](https://doc.rust-lang.org/error_codes/E0382.html).

## Roadmap

| Problems | Theme |
| -------- | ----- |
| 0101-0106 | Ownership: moves, `Clone` vs `Copy`, moving out of a `Vec`, `mem::take`, `mem::swap`, `&mut` |
| 0107-0111 | Fix the borrow checker, part 1: use after move, dangling references, writing through `&` |
| 0112-0119 | Slices `&[T]`: windows, checked ranges, returning sub-slices, owned `mut` vectors |
| 0120-0131 | In place through `&mut Vec<T>` / `&mut [T]`: swaps, write indices, merging, `split_at_mut`, two more fixes |
| 0132-0137 | `&str` vs `String`: bytes vs chars, the first word, deref coercion, ASCII bytes |
| 0138-0146 | Building strings: `format!`, `+`, `push_str`, `join`, slice patterns, run-length decoding |
| 0147-0156 | `chars`, `char_indices`, UTF-8 boundaries, Unicode case conversion |
| 0157-0171 | Words: `split_whitespace`, `trim`, `lines`, `split_once`, returning `&str`, choosing signatures |
| 0172-0183 | Two indices on strings, prefixes and searching |
| 0184-0188 | Strings as stacks and digit-by-digit arithmetic |
| 0189-0200 | Medium challenges: rotation, Dutch flag, next permutation, compression, atoi, zigzag, multiplication, palindromes |

## How to work on a problem

```bash
cd learn_11_leetcode_practice/level_02_ownership_strings

cargo test practice::p0101      # run the tests for the problem you are solving
cargo test solutions::p0101     # run the tests against the reference solution
cargo test solutions                # run every reference solution in this level
```

1. Read the statement in `exercises/` (click a problem in the table below).
2. Write your code in the file with the same name in `src/practice/`: replace each `todo!(...)`. **Do not edit the tests.**
3. Run `cargo test practice::pNNNN` until every test passes.
4. Open the **Answer** section at the bottom of the Markdown file to compare (the tested reference lives in `src/solutions/`).

Problems whose source is **LC** use LeetCode's exact Rust signature, so you can paste the `impl Solution { ... }` block into LeetCode and submit it.

## Problems

<!-- PROBLEM_TABLE:START -->

| # | Problem | Difficulty | Topics | Source |
| --- | ------- | ---------- | ------ | ------ |
| 0101 | [Take Ownership and Give It Back](exercises/p0101_take_and_give_back.md) | Easy | ownership, moving a `String` into a function, returning ownership, `push` | Original |
| 0102 | [Copy vs Clone: Largest Number and Longest Name](exercises/p0102_copy_vs_clone.md) | Easy | `Copy` vs `Clone`, borrowing a slice, `copied()`, `cloned()`, `Option` | Original |
| 0103 | [Split Off the Head](exercises/p0103_split_off_the_head.md) | Easy | moving values out of a `Vec`, `into_iter`, `collect`, `Option` | Original |
| 0104 | [Empty the Buffer](exercises/p0104_empty_the_buffer.md) | Easy | `&mut String`, `std::mem::take`, moving out of a mutable reference, `Option` | Original |
| 0105 | [Keep the Shorter String First](exercises/p0105_shorter_string_first.md) | Easy | two `&mut String` parameters, `std::mem::swap`, comparing through references | Original |
| 0106 | [Add a Suffix to Every Word](exercises/p0106_add_suffix_to_every_word.md) | Easy | `&mut [String]`, `iter_mut`, `push_str`, `ends_with`, `&str` parameters | Original |
| 0107 | [Fix: Value Moved in a Loop](exercises/p0107_fix_value_moved_in_loop.md) | Easy | fix the borrow checker, error E0382, moves inside loops, `clone` | Original |
| 0108 | [Fix: Use After a Consuming Loop](exercises/p0108_fix_use_after_consuming_loop.md) | Easy | fix the borrow checker, error E0382, `for x in v` vs `for x in &v` | Original |
| 0109 | [Fix: Returning a Reference to a Local](exercises/p0109_fix_dangling_reference.md) | Easy | fix the borrow checker, error E0106, dangling references, returning `String`, `format!` | Original |
| 0110 | [Fix: Assigning Through a Shared Reference](exercises/p0110_fix_assign_through_shared_reference.md) | Easy | fix the borrow checker, error E0594, `&` vs `&mut`, `&mut [i32]`, `iter_mut`, `clamp` | Original |
| 0111 | [Fix: Moving Out of an Index](exercises/p0111_fix_move_out_of_index.md) | Easy | fix the borrow checker, error E0507, keeping a `&String`, cloning once at the end | Original |
| 0112 | [Count the Increases](exercises/p0112_count_the_increases.md) | Easy | slices `&[T]`, `windows(2)`, slice patterns, `filter().count()` | Original |
| 0113 | [Sum of a Window](exercises/p0113_sum_of_a_window.md) | Easy | range slicing `&v[a..b]`, `slice::get` with a range, `Option`, avoiding panics | Original |
| 0114 | [Trim Zeros From Both Ends](exercises/p0114_trim_zeros_from_both_ends.md) | Easy | returning a sub-slice `&[T]` that borrows from the input, `position`, `rposition` | Original |
| 0115 | [Split at the First Zero](exercises/p0115_split_at_the_first_zero.md) | Easy | returning two borrowed slices, `position`, `split_at`, `Option` with `match` | Original |
| 0116 | [Longest Increasing Run](exercises/p0116_longest_increasing_run.md) | Easy | returning a sub-slice, run detection with indices, slice ranges | Original |
| 0117 | [Plus One](exercises/p0117_plus_one.md) | Easy | owned `Vec` parameter made `mut`, `iter_mut().rev()`, carrying, `Vec::insert` | [LC 66](https://leetcode.com/problems/plus-one/) |
| 0118 | [Replace Elements with Greatest Element on Right Side](exercises/p0118_replace_elements_with_greatest_element_on_right_side.md) | Easy | reusing an owned `Vec`, `iter_mut().rev()`, a running maximum | [LC 1299](https://leetcode.com/problems/replace-elements-with-greatest-element-on-right-side/) |
| 0119 | [Squares of a Sorted Array](exercises/p0119_squares_of_a_sorted_array.md) | Easy | two indices moving inwards, filling an output from the back, `iter_mut().rev()` | [LC 977](https://leetcode.com/problems/squares-of-a-sorted-array/) |
| 0120 | [Swap Adjacent Pairs](exercises/p0120_swap_adjacent_pairs.md) | Easy | `&mut [String]`, `slice::swap`, `chunks_exact_mut`, moving non-`Copy` elements | Original |
| 0121 | [Reverse String](exercises/p0121_reverse_string.md) | Easy | in-place modification through `&mut Vec<char>`, two indices, `slice::swap` | [LC 344](https://leetcode.com/problems/reverse-string/) |
| 0122 | [Sort Array By Parity](exercises/p0122_sort_array_by_parity.md) | Easy | owned `mut` parameter, write index, `slice::swap` | [LC 905](https://leetcode.com/problems/sort-array-by-parity/) |
| 0123 | [Move Zeroes](exercises/p0123_move_zeroes.md) | Easy | in-place modification through `&mut Vec<i32>`, read/write indices, `slice::swap` | [LC 283](https://leetcode.com/problems/move-zeroes/) |
| 0124 | [Remove Element](exercises/p0124_remove_element.md) | Easy | in-place compaction through `&mut Vec<i32>`, read/write indices, returning a length | [LC 27](https://leetcode.com/problems/remove-element/) |
| 0125 | [Remove Duplicates from Sorted Array](exercises/p0125_remove_duplicates_from_sorted_array.md) | Easy | in-place compaction through `&mut Vec<i32>`, write index, comparing with the last kept value | [LC 26](https://leetcode.com/problems/remove-duplicates-from-sorted-array/) |
| 0126 | [Merge Sorted Array](exercises/p0126_merge_sorted_array.md) | Easy | two `&mut Vec<i32>` parameters, filling from the back, `usize` index arithmetic | [LC 88](https://leetcode.com/problems/merge-sorted-array/) |
| 0127 | [Duplicate Zeros](exercises/p0127_duplicate_zeros.md) | Easy | in-place shifting through `&mut Vec<i32>`, working from the back, virtual indices | [LC 1089](https://leetcode.com/problems/duplicate-zeros/) |
| 0128 | [Apply Operations to an Array](exercises/p0128_apply_operations_to_an_array.md) | Easy | owned `mut` vector, sequential in-place updates, moving zeros with a write index | [LC 2460](https://leetcode.com/problems/apply-operations-to-an-array/) |
| 0129 | [Fix: Pushing While Iterating](exercises/p0129_fix_push_while_iterating.md) | Easy | fix the borrow checker, error E0502, shared vs mutable borrows, `collect` then `extend` | Original |
| 0130 | [Fix: Two Mutable Borrows of One Vec](exercises/p0130_fix_two_mutable_borrows.md) | Easy | fix the borrow checker, error E0499, `slice::swap`, `split_first_mut`, `split_last_mut` | Original |
| 0131 | [Fold a Slice in Half](exercises/p0131_fold_in_half.md) | Easy | `split_at_mut`, a shared and a mutable view of one slice, `zip`, `rev` | Original |
| 0132 | [Count Bytes and Characters](exercises/p0132_count_bytes_and_chars.md) | Easy | `&str`, UTF-8 encoding, `len()` vs `chars().count()`, `char::len_utf8` | Original |
| 0133 | [The First Word](exercises/p0133_the_first_word.md) | Easy | string slices, returning `&str` that borrows from the input, `as_bytes`, `enumerate` | Original |
| 0134 | [Shout Any Text](exercises/p0134_shout_any_text.md) | Easy | `&str` parameters accept `&String` (deref coercion), `trim`, `to_uppercase`, `ends_with` | Original |
| 0135 | [Score of a String](exercises/p0135_score_of_a_string.md) | Easy | `as_bytes`, `windows(2)`, `u8::abs_diff`, `sum` | [LC 3110](https://leetcode.com/problems/score-of-a-string/) |
| 0136 | [To Lower Case](exercises/p0136_to_lower_case.md) | Easy | case conversion, `make_ascii_lowercase` in place, owned `String` parameter | [LC 709](https://leetcode.com/problems/to-lower-case/) |
| 0137 | [Defanging an IP Address](exercises/p0137_defanging_an_ip_address.md) | Easy | building a `String` with `with_capacity`, `push`, `push_str`, `str::replace` | [LC 1108](https://leetcode.com/problems/defanging-an-ip-address/) |
| 0138 | [Format a Receipt Line](exercises/p0138_format_a_receipt_line.md) | Easy | `format!`, width and alignment (`{:<10}`, `{:>3}`), zero padding (`{:02}`), `u64` | Original |
| 0139 | [Fix: Adding Two Strings](exercises/p0139_fix_adding_two_strings.md) | Easy | fix the compiler error, E0308, the `+` operator on `String` (`String + &str`), `format!` | Original |
| 0140 | [Repeat With a Separator](exercises/p0140_repeat_with_a_separator.md) | Easy | `String::with_capacity`, `push_str`, `join`, `str::repeat` | Original |
| 0141 | [List Names in English](exercises/p0141_list_names_in_english.md) | Easy | slice patterns (`[a, b]`, `[rest @ .., last]`), `join`, `format!`, `&[&str]` | Original |
| 0142 | [Merge Strings Alternately](exercises/p0142_merge_strings_alternately.md) | Easy | two `Chars` iterators, `String::with_capacity`, `String: Extend<char>`, `Option` as an iterator | [LC 1768](https://leetcode.com/problems/merge-strings-alternately/) |
| 0143 | [Goal Parser Interpretation](exercises/p0143_goal_parser_interpretation.md) | Easy | scanning `as_bytes()` with an index, `match` with guards, `slice::get`, `str::replace` | [LC 1678](https://leetcode.com/problems/goal-parser-interpretation/) |
| 0144 | [Shuffle String](exercises/p0144_shuffle_string.md) | Easy | a `Vec<char>` buffer, `zip`, `i32` to `usize` indices, collecting into a `String` | [LC 1528](https://leetcode.com/problems/shuffle-string/) |
| 0145 | [Goat Latin](exercises/p0145_goat_latin.md) | Easy | `split(' ')`, `enumerate`, `split_at`, `push_str`, `push`, building one output `String` | [LC 824](https://leetcode.com/problems/goat-latin/) |
| 0146 | [Run-Length Decode](exercises/p0146_run_length_decode.md) | Easy | `chars()`, `char::to_digit`, accumulating a number, `Option::take`, `push` in a loop | Original |
| 0147 | [Caesar Cipher](exercises/p0147_caesar_cipher.md) | Easy | `chars().map(..).collect()`, `is_ascii_lowercase`, `u8` arithmetic, `as char` | Original |
| 0148 | [Byte Positions of a Character](exercises/p0148_byte_positions_of_a_character.md) | Easy | `char_indices`, byte offsets vs character positions, `enumerate`, UTF-8 | Original |
| 0149 | [Reverse by Characters](exercises/p0149_reverse_by_characters.md) | Easy | `chars().rev()`, `collect::<String>()`, `Iterator::eq`, why reversing bytes breaks UTF-8 | Original |
| 0150 | [Fix: Indexing a String](exercises/p0150_fix_indexing_a_string.md) | Easy | fix the compiler error, E0277, strings cannot be indexed by integers, `chars().next()`, `Chars::as_str` | Original |
| 0151 | [Take the First N Characters](exercises/p0151_take_the_first_n_characters.md) | Easy | returning `&str` from the input, `char_indices().nth`, slicing on a character boundary | Original |
| 0152 | [Truncate to a Byte Budget](exercises/p0152_truncate_to_a_byte_budget.md) | Easy | `str::is_char_boundary`, returning a `&str` prefix, UTF-8 byte lengths | Original |
| 0153 | [Swap Case](exercises/p0153_swap_case.md) | Easy | case conversion, `char::is_uppercase`, `char::to_lowercase` returns an iterator, `extend` | Original |
| 0154 | [Detect Capital](exercises/p0154_detect_capital.md) | Easy | `bytes()`, `u8::is_ascii_uppercase`, counting with `filter`, `as_bytes()[0]` | [LC 520](https://leetcode.com/problems/detect-capital/) |
| 0155 | [Capitalize the Title](exercises/p0155_capitalize_the_title.md) | Easy | case conversion on sub-slices (`out[a..b].make_ascii_lowercase()`), `split(' ')`, building one `String` | [LC 2129](https://leetcode.com/problems/capitalize-the-title/) |
| 0156 | [snake_case to camelCase](exercises/p0156_snake_case_to_camel_case.md) | Easy | `split('_')`, skipping empty pieces, `to_ascii_uppercase` / `to_ascii_lowercase`, `push` | Original |
| 0157 | [Normalize Whitespace](exercises/p0157_normalize_whitespace.md) | Easy | `split_whitespace`, `Vec<&str>` borrowing from the input, `join` | Original |
| 0158 | [Length of Last Word](exercises/p0158_length_of_last_word.md) | Easy | `split_whitespace().next_back()`, `Option::map_or`, `trim_end`, `rsplit` | [LC 58](https://leetcode.com/problems/length-of-last-word/) |
| 0159 | [Number of Segments in a String](exercises/p0159_number_of_segments_in_a_string.md) | Easy | `split(' ')` vs `split_whitespace`, filtering empty pieces, `count` | [LC 434](https://leetcode.com/problems/number-of-segments-in-a-string/) |
| 0160 | [Truncate Sentence](exercises/p0160_truncate_sentence.md) | Easy | finding a byte index with `match_indices`, `String::truncate` on an owned `String` | [LC 1816](https://leetcode.com/problems/truncate-sentence/) |
| 0161 | [Maximum Number of Words Found in Sentences](exercises/p0161_maximum_number_of_words_found_in_sentences.md) | Easy | iterating a `Vec<String>` by reference, counting bytes, `max` | [LC 2114](https://leetcode.com/problems/maximum-number-of-words-found-in-sentences/) |
| 0162 | [Initials](exercises/p0162_initials.md) | Easy | `split_whitespace`, `chars().next()`, `char::to_uppercase`, building a `String` | Original |
| 0163 | [Reverse Words in a String III](exercises/p0163_reverse_words_in_a_string_iii.md) | Easy | `into_bytes`, `split_mut` on `&mut [u8]`, `slice::reverse`, `String::from_utf8` | [LC 557](https://leetcode.com/problems/reverse-words-in-a-string-iii/) |
| 0164 | [Sorting the Sentence](exercises/p0164_sorting_the_sentence.md) | Easy | `split_at` to separate a suffix, `Vec<&str>` slots borrowing from the input, `join` | [LC 1859](https://leetcode.com/problems/sorting-the-sentence/) |
| 0165 | [Trim Each Line](exercises/p0165_trim_each_line.md) | Easy | `lines()`, `trim`, returning `Vec<&str>` that borrows from the input | Original |
| 0166 | [Parse a Key=Value Pair](exercises/p0166_parse_a_key_value_pair.md) | Easy | `split_once`, `trim`, returning `Option<(&str, &str)>` borrowed from the input | Original |
| 0167 | [Longest Word](exercises/p0167_longest_word.md) | Easy | `split` with a closure pattern, `char::is_alphanumeric`, returning the best `&str` | Original |
| 0168 | [Strip a Title Prefix](exercises/p0168_strip_a_title_prefix.md) | Easy | choosing `&str` or `String` as a return type, `strip_prefix`, `Option::filter`, `format!` | Original |
| 0169 | [Fix: Clearing a String While Borrowed](exercises/p0169_fix_clear_while_borrowed.md) | Easy | fix the borrow checker, error E0502, a slice keeps its `String` borrowed, `to_string` before `clear` | Original |
| 0170 | [Fix: Returning a Slice of a Local String](exercises/p0170_fix_return_slice_of_local.md) | Easy | fix the borrow checker, error E0515, slices of temporaries, choosing `String` as the return type | Original |
| 0171 | [Slugify a Title](exercises/p0171_slugify.md) | Medium | choosing `&str` in and `String` out, a "pending separator" flag, `is_ascii_alphanumeric`, validating with `bytes().all` | Original |
| 0172 | [Valid Palindrome](exercises/p0172_valid_palindrome.md) | Easy | two indices over `as_bytes()`, `is_ascii_alphanumeric`, `eq_ignore_ascii_case` | [LC 125](https://leetcode.com/problems/valid-palindrome/) |
| 0173 | [Find First Palindromic String in the Array](exercises/p0173_find_first_palindromic_string_in_the_array.md) | Easy | `into_iter().find` to move a `String` out of a `Vec`, `bytes().eq(bytes().rev())`, `unwrap_or_default` | [LC 2108](https://leetcode.com/problems/find-first-palindromic-string-in-the-array/) |
| 0174 | [Reverse Vowels of a String](exercises/p0174_reverse_vowels_of_a_string.md) | Easy | `into_bytes`, two indices with `swap`, `matches!`, `String::from_utf8` | [LC 345](https://leetcode.com/problems/reverse-vowels-of-a-string/) |
| 0175 | [Reverse String II](exercises/p0175_reverse_string_ii.md) | Easy | `chunks_mut` on a byte buffer, reversing a sub-slice `chunk[..end].reverse()`, `min` | [LC 541](https://leetcode.com/problems/reverse-string-ii/) |
| 0176 | [Reverse Prefix of Word](exercises/p0176_reverse_prefix_of_word.md) | Easy | `str::find`, `split_at`, `chars().rev().chain(..)`, returning the input unchanged | [LC 2000](https://leetcode.com/problems/reverse-prefix-of-word/) |
| 0177 | [Lexicographically Smallest Palindrome](exercises/p0177_lexicographically_smallest_palindrome.md) | Easy | mirrored indices `i` and `n - 1 - i`, writing two positions of one `Vec<u8>`, `min` | [LC 2697](https://leetcode.com/problems/lexicographically-smallest-palindrome/) |
| 0178 | [Is Subsequence](exercises/p0178_is_subsequence.md) | Easy | one iterator shared across a loop, `Iterator::any` consuming items, `all` | [LC 392](https://leetcode.com/problems/is-subsequence/) |
| 0179 | [Longest Common Prefix](exercises/p0179_longest_common_prefix.md) | Easy | shrinking a borrowed `&str` prefix, `split_first`, `bytes().zip(..)`, `take_while` | [LC 14](https://leetcode.com/problems/longest-common-prefix/) |
| 0180 | [Find the Index of the First Occurrence in a String](exercises/p0180_find_the_index_of_the_first_occurrence_in_a_string.md) | Easy | comparing byte slices, `windows(n)`, `position`, `str::find` | [LC 28](https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/) |
| 0181 | [Counting Words With a Given Prefix](exercises/p0181_counting_words_with_a_given_prefix.md) | Easy | `starts_with` with a `&String` pattern, borrowing in `filter`, `count` | [LC 2185](https://leetcode.com/problems/counting-words-with-a-given-prefix/) |
| 0182 | [Largest Odd Number in String](exercises/p0182_largest_odd_number_in_string.md) | Easy | `rfind` with a `[char; N]` pattern, `String::truncate` on an owned `String`, prefixes | [LC 1903](https://leetcode.com/problems/largest-odd-number-in-string/) |
| 0183 | [Largest 3-Same-Digit Number in String](exercises/p0183_largest_3_same_digit_number_in_string.md) | Easy | `as_bytes().windows(3)`, slice patterns on windows, `max`, building a `String` from a byte | [LC 2264](https://leetcode.com/problems/largest-3-same-digit-number-in-string/) |
| 0184 | [Remove All Adjacent Duplicates In String](exercises/p0184_remove_all_adjacent_duplicates_in_string.md) | Easy | a `String` used as a stack (`push`, `pop`, `ends_with`) | [LC 1047](https://leetcode.com/problems/remove-all-adjacent-duplicates-in-string/) |
| 0185 | [Make The String Great](exercises/p0185_make_the_string_great.md) | Easy | `String` as a stack, `chars().next_back()`, match guards, `eq_ignore_ascii_case` | [LC 1544](https://leetcode.com/problems/make-the-string-great/) |
| 0186 | [Backspace String Compare](exercises/p0186_backspace_string_compare.md) | Easy | a helper taking `&str`, `String` as a stack, comparing owned results | [LC 844](https://leetcode.com/problems/backspace-string-compare/) |
| 0187 | [Add Binary](exercises/p0187_add_binary.md) | Easy | walking two byte slices from the back, carry, `Vec<u8>` + `reverse`, `String::from_utf8` | [LC 67](https://leetcode.com/problems/add-binary/) |
| 0188 | [Add Strings](exercises/p0188_add_strings.md) | Easy | two `bytes().rev()` iterators, `Option::map_or`, carry, collecting digits in reverse | [LC 415](https://leetcode.com/problems/add-strings/) |
| 0189 | [Rotate Array](exercises/p0189_rotate_array.md) | Medium | in-place rotation with three `reverse` calls on sub-slices, `k % n`, `rotate_right` | [LC 189](https://leetcode.com/problems/rotate-array/) |
| 0190 | [Sort Colors](exercises/p0190_sort_colors.md) | Medium | three indices on one `&mut Vec<i32>` (Dutch national flag), `swap`, `match` on values | [LC 75](https://leetcode.com/problems/sort-colors/) |
| 0191 | [Remove Duplicates from Sorted Array II](exercises/p0191_remove_duplicates_from_sorted_array_ii.md) | Medium | in-place compaction with a write index, comparing with `nums[write - 2]` | [LC 80](https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/) |
| 0192 | [Next Permutation](exercises/p0192_next_permutation.md) | Medium | in-place `swap` + `reverse` of a sub-slice, searching from the back with `rev().find` | [LC 31](https://leetcode.com/problems/next-permutation/) |
| 0193 | [String Compression](exercises/p0193_string_compression.md) | Medium | in-place rewrite of a `&mut Vec<char>` with read/write indices, `to_string().chars()`, `truncate` | [LC 443](https://leetcode.com/problems/string-compression/) |
| 0194 | [Reverse Words in a String](exercises/p0194_reverse_words_in_a_string.md) | Medium | in-place word reversal on the string's own bytes, compacting spaces, `split_whitespace().rev()` | [LC 151](https://leetcode.com/problems/reverse-words-in-a-string/) |
| 0195 | [Compare Version Numbers](exercises/p0195_compare_version_numbers.md) | Medium | two `split('.')` iterators, `str::parse`, `Ordering`, treating a missing part as `0` | [LC 165](https://leetcode.com/problems/compare-version-numbers/) |
| 0196 | [String to Integer (atoi)](exercises/p0196_string_to_integer_atoi.md) | Medium | `trim_start_matches`, matching on `as_bytes().first()`, `take_while`, overflow handling, `clamp` | [LC 8](https://leetcode.com/problems/string-to-integer-atoi/) |
| 0197 | [Count and Say](exercises/p0197_count_and_say.md) | Medium | building a new `String` from the previous one, `slice::chunk_by`, replacing an owned value in a loop | [LC 38](https://leetcode.com/problems/count-and-say/) |
| 0198 | [Zigzag Conversion](exercises/p0198_zigzag_conversion.md) | Medium | a `Vec<String>` of rows, `vec![String::new(); n]`, direction flag, `concat` | [LC 6](https://leetcode.com/problems/zigzag-conversion/) |
| 0199 | [Multiply Strings](exercises/p0199_multiply_strings.md) | Medium | digit vectors, schoolbook multiplication, `enumerate().rev()`, `char::from_digit` | [LC 43](https://leetcode.com/problems/multiply-strings/) |
| 0200 | [Longest Palindromic Substring](exercises/p0200_longest_palindromic_substring.md) | Medium | expanding around centers with byte indices, a helper taking `&[u8]`, slicing the answer out of the input | [LC 5](https://leetcode.com/problems/longest-palindromic-substring/) |

<!-- PROBLEM_TABLE:END -->
