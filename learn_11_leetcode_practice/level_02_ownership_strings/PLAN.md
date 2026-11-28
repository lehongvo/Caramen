# Level 2 plan (temporary; delete when the level is complete)

55 LeetCode problems (43 Easy, 12 Medium, all checked against tools/.cache/leetcode_index.csv, none used by
level 1) + 45 original problems. Groups go from easier to harder; the last group holds the Medium challenges.

| # | Module | Title | Source | Rust focus |
|---|--------|-------|--------|------------|
| **Ownership, moves, Clone vs Copy, &mut** | | | | |
| 0101 | p0101_take_and_give_back | Take Ownership and Give It Back | Original | move into a fn, return ownership |
| 0102 | p0102_copy_vs_clone | Copy vs Clone: Largest Number and Longest Name | Original | `Copy` (`copied`) vs `Clone` (`cloned`) |
| 0103 | p0103_split_off_the_head | Split Off the Head | Original | moving out of a `Vec` (`into_iter`, `remove`) |
| 0104 | p0104_empty_the_buffer | Empty the Buffer | Original | `std::mem::take`, `&mut String` |
| 0105 | p0105_shorter_string_first | Keep the Shorter String First | Original | two `&mut`, `std::mem::swap` |
| 0106 | p0106_add_suffix_to_every_word | Add a Suffix to Every Word | Original | `iter_mut`, `push_str` through `&mut` |
| **Fix the borrow checker (part 1)** | | | | |
| 0107 | p0107_fix_value_moved_in_loop | Fix: Value Moved in a Loop | Original | E0382, `clone` |
| 0108 | p0108_fix_use_after_consuming_loop | Fix: Use After a Consuming Loop | Original | E0382, `for x in &v` |
| 0109 | p0109_fix_dangling_reference | Fix: Returning a Reference to a Local | Original | E0106, return owned `String` |
| 0110 | p0110_fix_assign_through_shared_reference | Fix: Assigning Through a Shared Reference | Original | E0594, `&mut [i32]`, `iter_mut` |
| 0111 | p0111_fix_move_out_of_index | Fix: Moving Out of an Index | Original | E0507, borrow then clone once |
| **Slices &[T]** | | | | |
| 0112 | p0112_count_the_increases | Count the Increases | Original | `windows(2)` |
| 0113 | p0113_sum_of_a_window | Sum of a Window | Original | range slicing, `get(range)` |
| 0114 | p0114_trim_zeros_from_both_ends | Trim Zeros From Both Ends | Original | returning `&[T]`, `position`, `rposition` |
| 0115 | p0115_split_at_the_first_zero | Split at the First Zero | Original | `position`, `split_at`, tuple of slices |
| 0116 | p0116_longest_increasing_run | Longest Increasing Run | Original | returning a sub-slice |
| 0117 | p0117_plus_one | Plus One | LC 66 Easy | owned `Vec`, carry, `insert` |
| 0118 | p0118_replace_elements_with_greatest_element_on_right_side | Replace Elements with Greatest Element on Right Side | LC 1299 Easy | `iter_mut().rev()`, owned `mut` param |
| 0119 | p0119_squares_of_a_sorted_array | Squares of a Sorted Array | LC 977 Easy | two indices, fill from the back |
| **In-place &mut Vec / &mut [T]** | | | | |
| 0120 | p0120_swap_adjacent_pairs | Swap Adjacent Pairs | Original | `chunks_exact_mut`, `slice::swap` |
| 0121 | p0121_reverse_string | Reverse String | LC 344 Easy | `&mut Vec<char>`, two indices + `swap` |
| 0122 | p0122_sort_array_by_parity | Sort Array By Parity | LC 905 Easy | `mut` owned param, two pointers |
| 0123 | p0123_move_zeroes | Move Zeroes | LC 283 Easy | write index, `swap` |
| 0124 | p0124_remove_element | Remove Element | LC 27 Easy | read/write indices, `truncate` |
| 0125 | p0125_remove_duplicates_from_sorted_array | Remove Duplicates from Sorted Array | LC 26 Easy | write index on `&mut Vec` |
| 0126 | p0126_merge_sorted_array | Merge Sorted Array | LC 88 Easy | fill from the back, two `&mut` params |
| 0127 | p0127_duplicate_zeros | Duplicate Zeros | LC 1089 Easy | in-place shifting from the back |
| 0128 | p0128_apply_operations_to_an_array | Apply Operations to an Array | LC 2460 Easy | in-place pass + compaction |
| 0129 | p0129_fix_push_while_iterating | Fix: Pushing While Iterating | Original | E0502, collect first then `extend` |
| 0130 | p0130_fix_two_mutable_borrows | Fix: Two Mutable Borrows of One Vec | Original | E0499, `slice::swap` |
| 0131 | p0131_fold_in_half | Fold a Slice in Half | Original | `split_at_mut` |
| **&str vs String basics** | | | | |
| 0132 | p0132_count_bytes_and_chars | Count Bytes and Characters | Original | `len()` vs `chars().count()`, UTF-8 |
| 0133 | p0133_the_first_word | The First Word | Original (Book 4.3) | `&str -> &str`, `as_bytes`, slicing |
| 0134 | p0134_shout_any_text | Shout Any Text | Original | `&str` params accept `&String`, `to_uppercase` |
| 0135 | p0135_score_of_a_string | Score of a String | LC 3110 Easy | `as_bytes`, `windows`, `abs_diff` |
| 0136 | p0136_to_lower_case | To Lower Case | LC 709 Easy | `to_ascii_lowercase`, `make_ascii_lowercase` |
| 0137 | p0137_defanging_an_ip_address | Defanging an IP Address | LC 1108 Easy | `replace`, building with `push_str` |
| **Building strings** | | | | |
| 0138 | p0138_format_a_receipt_line | Format a Receipt Line | Original | `format!` width/precision |
| 0139 | p0139_fix_adding_two_strings | Fix: Adding Two Strings | Original | E0308, `String + &str` |
| 0140 | p0140_repeat_with_a_separator | Repeat With a Separator | Original | `push_str`, `join`, `repeat` |
| 0141 | p0141_list_names_in_english | List Names in English | Original | slice patterns, `join`, `format!` |
| 0142 | p0142_merge_strings_alternately | Merge Strings Alternately | LC 1768 Easy | `String::with_capacity`, `push`, `chars` |
| 0143 | p0143_goal_parser_interpretation | Goal Parser Interpretation | LC 1678 Easy | `replace` chains / byte scanning |
| 0144 | p0144_shuffle_string | Shuffle String | LC 1528 Easy | `Vec<char>` buffer, `collect` |
| 0145 | p0145_goat_latin | Goat Latin | LC 824 Easy | `split_whitespace`, `push_str`, `repeat` |
| 0146 | p0146_run_length_decode | Run-Length Decode | Original | `chars`, `to_digit`, `repeat` |
| **chars, bytes, char_indices, UTF-8, case** | | | | |
| 0147 | p0147_caesar_cipher | Caesar Cipher | Original | `u8` arithmetic, `as char` |
| 0148 | p0148_byte_positions_of_a_character | Byte Positions of a Character | Original | `char_indices`, byte offsets |
| 0149 | p0149_reverse_by_characters | Reverse by Characters | Original | `chars().rev()`, why bytes break UTF-8 |
| 0150 | p0150_fix_indexing_a_string | Fix: Indexing a String | Original | E0277, `chars().next()`, `len_utf8` |
| 0151 | p0151_take_the_first_n_characters | Take the First N Characters | Original | `char_indices().nth`, `&str` result |
| 0152 | p0152_truncate_to_a_byte_budget | Truncate to a Byte Budget | Original | `is_char_boundary` |
| 0153 | p0153_swap_case | Swap Case | Original | `char::to_uppercase` iterators, `ß` |
| 0154 | p0154_detect_capital | Detect Capital | LC 520 Easy | `chars().all`, `is_ascii_uppercase` |
| 0155 | p0155_capitalize_the_title | Capitalize the Title | LC 2129 Easy | case conversion per word, `join` |
| 0156 | p0156_snake_case_to_camel_case | snake_case to camelCase | Original | `split('_')`, capitalizing |
| **Words: split_whitespace, trim, returning &str** | | | | |
| 0157 | p0157_normalize_whitespace | Normalize Whitespace | Original | `split_whitespace`, `join` |
| 0158 | p0158_length_of_last_word | Length of Last Word | LC 58 Easy | `trim_end`, `rsplit` |
| 0159 | p0159_number_of_segments_in_a_string | Number of Segments in a String | LC 434 Easy | `split_whitespace().count()` |
| 0160 | p0160_truncate_sentence | Truncate Sentence | LC 1816 Easy | `take`, `join` / byte cut |
| 0161 | p0161_maximum_number_of_words_found_in_sentences | Maximum Number of Words Found in Sentences | LC 2114 Easy | iterating `&[String]` |
| 0162 | p0162_initials | Initials | Original | `split_whitespace`, `chars().next()`, `to_uppercase` |
| 0163 | p0163_reverse_words_in_a_string_iii | Reverse Words in a String III | LC 557 Easy | `split(' ')`, `chars().rev()` |
| 0164 | p0164_sorting_the_sentence | Sorting the Sentence | LC 1859 Easy | `split_at`, placing `&str`s |
| 0165 | p0165_trim_each_line | Trim Each Line | Original | `lines`, `trim`, `Vec<&str>` |
| 0166 | p0166_parse_a_key_value_pair | Parse a Key=Value Pair | Original | `split_once`, `Option<(&str, &str)>` |
| 0167 | p0167_longest_word | Longest Word | Original | returning `&str` from input |
| 0168 | p0168_strip_a_title_prefix | Strip a Title Prefix | Original | `strip_prefix`, `&str` vs `String` result |
| 0169 | p0169_fix_clear_while_borrowed | Fix: Clearing a String While Borrowed | Original | E0502, `to_string` before `clear` |
| 0170 | p0170_fix_return_slice_of_local | Fix: Returning a Slice of a Local String | Original | E0515 |
| 0171 | p0171_slugify | Slugify a Title | Original | choosing `&str` in / `String` out |
| **Two indices on strings, prefixes, searching** | | | | |
| 0172 | p0172_valid_palindrome | Valid Palindrome | LC 125 Easy | `bytes`, two indices |
| 0173 | p0173_find_first_palindromic_string_in_the_array | Find First Palindromic String in the Array | LC 2108 Easy | `iter().find`, `bytes().rev()` |
| 0174 | p0174_reverse_vowels_of_a_string | Reverse Vowels of a String | LC 345 Easy | `into_bytes`, `String::from_utf8` |
| 0175 | p0175_reverse_string_ii | Reverse String II | LC 541 Easy | `chunks_mut`, `reverse` on a prefix |
| 0176 | p0176_reverse_prefix_of_word | Reverse Prefix of Word | LC 2000 Easy | `find`, `split_at`, `chars().rev()` |
| 0177 | p0177_lexicographically_smallest_palindrome | Lexicographically Smallest Palindrome | LC 2697 Easy | `into_bytes`, two indices |
| 0178 | p0178_is_subsequence | Is Subsequence | LC 392 Easy | iterator consumed across a loop |
| 0179 | p0179_longest_common_prefix | Longest Common Prefix | LC 14 Easy | shrinking a `&str` prefix |
| 0180 | p0180_find_the_index_of_the_first_occurrence_in_a_string | Find the Index of the First Occurrence in a String | LC 28 Easy | `windows` over bytes / `find` |
| 0181 | p0181_counting_words_with_a_given_prefix | Counting Words With a Given Prefix | LC 2185 Easy | `starts_with` |
| 0182 | p0182_largest_odd_number_in_string | Largest Odd Number in String | LC 1903 Easy | `rfind`, prefix slice |
| 0183 | p0183_largest_3_same_digit_number_in_string | Largest 3-Same-Digit Number in String | LC 2264 Easy | `windows(3)`, slices of bytes |
| **Strings as stacks, digit arithmetic** | | | | |
| 0184 | p0184_remove_all_adjacent_duplicates_in_string | Remove All Adjacent Duplicates In String | LC 1047 Easy | `String` as a stack (`push`/`pop`) |
| 0185 | p0185_make_the_string_great | Make The String Great | LC 1544 Easy | stack, `eq_ignore_ascii_case` |
| 0186 | p0186_backspace_string_compare | Backspace String Compare | LC 844 Easy | helper taking `&str` |
| 0187 | p0187_add_binary | Add Binary | LC 67 Easy | bytes from the back, `reverse` |
| 0188 | p0188_add_strings | Add Strings | LC 415 Easy | digit arithmetic, `u8` to `char` |
| **Medium challenges** | | | | |
| 0189 | p0189_rotate_array | Rotate Array | LC 189 Medium | three reversals on `&mut [T]` |
| 0190 | p0190_sort_colors | Sort Colors | LC 75 Medium | Dutch flag, `swap` |
| 0191 | p0191_remove_duplicates_from_sorted_array_ii | Remove Duplicates from Sorted Array II | LC 80 Medium | write index, look back 2 |
| 0192 | p0192_next_permutation | Next Permutation | LC 31 Medium | `swap` + `reverse` on a sub-slice |
| 0193 | p0193_string_compression | String Compression | LC 443 Medium | in-place `&mut Vec<char>` |
| 0194 | p0194_reverse_words_in_a_string | Reverse Words in a String | LC 151 Medium | `split_whitespace().rev()` |
| 0195 | p0195_compare_version_numbers | Compare Version Numbers | LC 165 Medium | `split('.')`, `parse` |
| 0196 | p0196_string_to_integer_atoi | String to Integer (atoi) | LC 8 Medium | bytes, checked arithmetic |
| 0197 | p0197_count_and_say | Count and Say | LC 38 Medium | building `String`s in a loop |
| 0198 | p0198_zigzag_conversion | Zigzag Conversion | LC 6 Medium | `Vec<String>` rows |
| 0199 | p0199_multiply_strings | Multiply Strings | LC 43 Medium | digit vector, `char::from_digit` |
| 0200 | p0200_longest_palindromic_substring | Longest Palindromic Substring | LC 5 Medium | expand around center, slicing |
