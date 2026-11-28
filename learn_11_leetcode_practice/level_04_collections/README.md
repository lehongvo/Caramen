# Level 4: Collections (problems 0301-0400)

> Focus: `Vec`, `HashMap`, `HashSet`, `BTreeMap`, `VecDeque`, `BinaryHeap`, sorting.

This level covers chapter 8 of The Rust Book and the `std::collections` module: when to reach for a
`Vec`, a `HashMap`, a `HashSet`, an ordered `BTreeMap`/`BTreeSet`, a `VecDeque` or a `BinaryHeap`, and
how to sort and order data with `Ordering`, `then_with` and `Reverse`. Problems 0301-0368 are Easy and
introduce each collection in turn; problems 0369-0400 are Medium and revisit them in the same order.
74 problems come from LeetCode (45 Easy and 29 Medium, using LeetCode's exact Rust signatures,
including 12 "design" problems built as structs); the other 26 are original problems, each built
around one collection API.

## Prerequisites

- Levels 1-3, or the same knowledge: ownership and borrowing, slices, `String` vs `&str`, structs,
  `impl` blocks, enums, `match` and `Option`.
- Read [The Rust Book 8.1](https://doc.rust-lang.org/book/ch08-01-vectors.html) and
  [8.3](https://doc.rust-lang.org/book/ch08-03-hash-maps.html) before or alongside this level, and
  skim the overview of [`std::collections`](https://doc.rust-lang.org/std/collections/index.html),
  which explains when to use which collection.
- Missing values are reported with `Option` throughout. A few solutions (0308, 0337, 0369) use `?` on
  an `Option`; it is explained where it appears and covered in depth in level 5.

## What you will practice

- `Vec<T>`: `get` vs indexing, `insert`, `remove`, `swap`, `retain`, `dedup`, `dedup_by_key`,
  `drain`, `extend`, `append`, `truncate`:
  [Book 8.1](https://doc.rust-lang.org/book/ch08-01-vectors.html),
  [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html).
- Slice tools: `windows`, `chunks`, `chunks_exact`, `split_at`, `concat`, `join`, `contains`,
  `binary_search`, `partition_point`, `split_at_mut`:
  [slice methods](https://doc.rust-lang.org/std/primitive.slice.html).
- Sorting: `sort` vs `sort_unstable` (stable vs unstable), `sort_by`, `sort_by_key`, tuple keys,
  comparators built with [`Ordering`](https://doc.rust-lang.org/std/cmp/enum.Ordering.html),
  `then_with` and [`Reverse`](https://doc.rust-lang.org/std/cmp/struct.Reverse.html), and implementing
  [`Ord`](https://doc.rust-lang.org/std/cmp/trait.Ord.html) by hand.
- 2-D vectors `Vec<Vec<T>>`: building, transposing, reshaping, sorting rows, rotating in place.
- `HashMap`: the entry API (`or_insert`, `or_default`, `or_insert_with`, `and_modify`), `get`,
  `get_mut`, `insert` returning the old value, counting, grouping, two-way mappings, and why
  iteration order is unspecified:
  [Book 8.3](https://doc.rust-lang.org/book/ch08-03-hash-maps.html),
  [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html),
  [`Entry`](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html).
- `HashSet`: `insert` returning `bool`, `intersection`, `union`, `difference`,
  `symmetric_difference`, `is_subset`, `is_disjoint`:
  [`HashSet`](https://doc.rust-lang.org/std/collections/struct.HashSet.html).
- `BTreeMap` and `BTreeSet`: sorted iteration, `range` queries (including `Bound`), `first_key_value`,
  `last_key_value`, `pop_first`, `pop_last`:
  [`BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html),
  [`BTreeSet`](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html).
- `VecDeque` as a queue and a double-ended queue: `push_back`, `pop_front`, `push_front`, `pop_back`,
  sliding windows and simulations:
  [`VecDeque`](https://doc.rust-lang.org/std/collections/struct.VecDeque.html).
- `BinaryHeap` as a max-heap, `Reverse` for a min-heap, bounded heaps for top-k, k-way merging, and
  heaps of structs with a custom `Ord`:
  [`BinaryHeap`](https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html).
- Writing tests that do not depend on hash order: sorting results, or checking any valid answer
  with a small checker function.

## Roadmap

| Problems | Theme |
| -------- | ----- |
| 0301-0311 | `Vec` basics: safe reads, `insert`/`remove`, `retain`, `dedup`, `windows`, `chunks`, `split_at`/`concat`/`join`, `drain`/`extend` |
| 0312-0318 | Sorting: stable vs unstable, `sort_by_key`, `Reverse`, tuple keys, `Ordering` and `then_with` |
| 0319-0323 | Searching sorted data: `binary_search`, `partition_point`, ranks, prefix sums |
| 0324-0328 | 2-D vectors: building grids, Pascal's triangle, transpose, reshape, sorting rows |
| 0329-0338 | `HashSet`: `insert` returns `bool`, set operations, sets of tuples and strings, a hash set by hand |
| 0339-0352 | `HashMap`: counting with the entry API, `get_mut`, Two Sum, anagrams, bijections, `or_default`, iteration order, a hash map by hand |
| 0353-0358 | `BTreeMap` / `BTreeSet`: ordered output, `range` queries, first/last entries |
| 0359-0363 | `VecDeque`: round-robin, sliding time windows, stacks from queues, bounded history |
| 0364-0368 | `BinaryHeap`: priorities, simulations, bounded heaps, `Reverse` min-heaps |
| 0369-0375 | Medium: custom `Ord`, stable sorting with comparators, sort + insert, rotating a matrix in place |
| 0376-0383 | Medium `HashMap` / `HashSet`: grouping, prefix-sum counting, consecutive runs, top-k frequent |
| 0384-0388 | Medium ordered collections: greedy with `BTreeMap`, sorted tables, calendars, time-versioned values |
| 0389-0391 | Medium `VecDeque`: simulations on positions, two-queue games, a circular deque |
| 0392-0398 | Medium `BinaryHeap`: k-th largest, k closest, seat manager, triage with `impl Ord`, k-way merge, schedulers |
| 0399-0400 | Design capstones combining several collections: food ratings, a tiny Twitter |

## How to work on a problem

```bash
cd learn_11_leetcode_practice/level_04_collections

cargo test practice::p0301      # run the tests for the problem you are solving
cargo test solutions::p0301     # run the tests against the reference solution
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
| 0301 | [Inspect a Vector](exercises/p0301_inspect_a_vector.md) | Easy | `Vec<T>`, `len`, `first`, `last`, `get` vs indexing, `contains`, `iter().position` | Original |
| 0302 | [Edit a Playlist](exercises/p0302_edit_a_playlist.md) | Easy | `Vec::push`, `insert`, `remove`, `swap`, `pop`, avoiding index panics, `match` on an enum | Original |
| 0303 | [Create Target Array in the Given Order](exercises/p0303_create_target_array_in_the_given_order.md) | Easy | `Vec::insert`, `zip`, `as usize` casts | [LC 1389](https://leetcode.com/problems/create-target-array-in-the-given-order/) |
| 0304 | [Clean Sensor Readings](exercises/p0304_clean_sensor_readings.md) | Easy | `Vec::retain`, `&mut Vec<T>` parameters, in-place filtering | Original |
| 0305 | [Collapse Repeated Values](exercises/p0305_collapse_repeated_values.md) | Easy | `Vec::dedup`, `dedup_by_key`, `dedup_by`, sort + dedup | Original |
| 0306 | [Count Hills and Valleys in an Array](exercises/p0306_count_hills_and_valleys_in_an_array.md) | Easy | `Vec::dedup`, `slice::windows`, `filter` + `count` | [LC 2210](https://leetcode.com/problems/count-hills-and-valleys-in-an-array/) |
| 0307 | [Can Make Arithmetic Progression From Sequence](exercises/p0307_can_make_arithmetic_progression_from_sequence.md) | Easy | `sort_unstable`, `slice::windows`, `Iterator::all` | [LC 1502](https://leetcode.com/problems/can-make-arithmetic-progression-from-sequence/) |
| 0308 | [Paginate a List](exercises/p0308_paginate_a_list.md) | Easy | `slice::chunks`, `chunks_exact`, `remainder`, `Iterator::nth`, returning sub-slices | Original |
| 0309 | [Array Partition](exercises/p0309_array_partition.md) | Easy | `sort_unstable`, `slice::chunks_exact`, greedy pairing, `sum` | [LC 561](https://leetcode.com/problems/array-partition/) |
| 0310 | [Split, Swap and Join](exercises/p0310_split_swap_and_join.md) | Easy | `slice::split_at`, `concat`, `join`, slices of `Vec`s | Original |
| 0311 | [Move Finished Tasks](exercises/p0311_move_finished_tasks.md) | Easy | `Vec::drain`, `extend`, `append`, range arguments, `saturating_sub` | Original |
| 0312 | [Stable Sort by Department](exercises/p0312_stable_sort_by_department.md) | Easy | stable `sort_by` vs `sort_unstable_by`, `String::cmp`, sorting structs | Original |
| 0313 | [Minimum Absolute Difference](exercises/p0313_minimum_absolute_difference.md) | Easy | `sort_unstable`, `slice::windows`, building a `Vec<Vec<i32>>` | [LC 1200](https://leetcode.com/problems/minimum-absolute-difference/) |
| 0314 | [Sort the People](exercises/p0314_sort_the_people.md) | Easy | `zip`, `Vec<(i32, String)>`, `sort_unstable_by_key`, `std::cmp::Reverse` | [LC 2418](https://leetcode.com/problems/sort-the-people/) |
| 0315 | [Sort Integers by The Number of 1 Bits](exercises/p0315_sort_integers_by_the_number_of_1_bits.md) | Easy | `sort_unstable_by_key` with a tuple key, `count_ones`, lexicographic tuple order | [LC 1356](https://leetcode.com/problems/sort-integers-by-the-number-of-1-bits/) |
| 0316 | [A Multi-Key Leaderboard](exercises/p0316_multi_key_leaderboard.md) | Easy | `std::cmp::Ordering`, `Ord::cmp`, `Ordering::then_with`, `Ordering::reverse`, comparator functions | Original |
| 0317 | [Sort Array by Increasing Frequency](exercises/p0317_sort_array_by_increasing_frequency.md) | Easy | counting with a fixed-size array, `sort_by` with a two-level `Ordering`, `then_with` | [LC 1636](https://leetcode.com/problems/sort-array-by-increasing-frequency/) |
| 0318 | [Relative Sort Array](exercises/p0318_relative_sort_array.md) | Easy | rank table in a `Vec<Option<usize>>`, `sort_unstable_by_key` with a tuple key | [LC 1122](https://leetcode.com/problems/relative-sort-array/) |
| 0319 | [Search Insert Position](exercises/p0319_search_insert_position.md) | Easy | `slice::binary_search`, `Result<usize, usize>`, or-patterns in `match` | [LC 35](https://leetcode.com/problems/search-insert-position/) |
| 0320 | [Keep a Sorted List](exercises/p0320_keep_a_sorted_list.md) | Easy | `slice::partition_point`, `binary_search`, `Vec::insert`/`remove` on sorted data | Original |
| 0321 | [How Many Numbers Are Smaller Than the Current Number](exercises/p0321_how_many_numbers_are_smaller_than_the_current_number.md) | Easy | sorted copy with `clone` + `sort_unstable`, `partition_point`, `map` + `collect` | [LC 1365](https://leetcode.com/problems/how-many-numbers-are-smaller-than-the-current-number/) |
| 0322 | [Rank Transform of an Array](exercises/p0322_rank_transform_of_an_array.md) | Easy | `sort_unstable` + `dedup` to get distinct sorted values, `binary_search` as a lookup | [LC 1331](https://leetcode.com/problems/rank-transform-of-an-array/) |
| 0323 | [Longest Subsequence With Limited Sum](exercises/p0323_longest_subsequence_with_limited_sum.md) | Easy | sort, in-place prefix sums, `partition_point` per query | [LC 2389](https://leetcode.com/problems/longest-subsequence-with-limited-sum/) |
| 0324 | [Build a Grid](exercises/p0324_build_a_grid.md) | Easy | `Vec<Vec<T>>`, `vec![vec![x; cols]; rows]`, safe neighbour access with `get` and `checked_add_signed` | Original |
| 0325 | [Pascal's Triangle](exercises/p0325_pascals_triangle.md) | Easy | `Vec<Vec<i32>>`, borrowing the previous row, `windows(2)`, `extend` | [LC 118](https://leetcode.com/problems/pascals-triangle/) |
| 0326 | [Transpose Matrix](exercises/p0326_transpose_matrix.md) | Easy | non-square `Vec<Vec<i32>>`, nested `map` + `collect`, row/column indices | [LC 867](https://leetcode.com/problems/transpose-matrix/) |
| 0327 | [Reshape the Matrix](exercises/p0327_reshape_the_matrix.md) | Easy | flattening with `concat`, regrouping with `chunks`, `to_vec`, returning the input unchanged | [LC 566](https://leetcode.com/problems/reshape-the-matrix/) |
| 0328 | [Delete Greatest Value in Each Row](exercises/p0328_delete_greatest_value_in_each_row.md) | Easy | `iter_mut` over the rows of a 2-D `Vec`, sorting each row, column maxima | [LC 2500](https://leetcode.com/problems/delete-greatest-value-in-each-row/) |
| 0329 | [First Repeated Visitor](exercises/p0329_first_repeated_visitor.md) | Easy | `HashSet::insert` returns `bool`, `remove` returns `bool`, `len`, sorting a set's contents | Original |
| 0330 | [Contains Duplicate](exercises/p0330_contains_duplicate.md) | Easy | `HashSet::with_capacity`, `insert` returns `bool`, `Iterator::any` | [LC 217](https://leetcode.com/problems/contains-duplicate/) |
| 0331 | [Single Number](exercises/p0331_single_number.md) | Easy | toggling membership in a `HashSet`, `remove`, `into_iter().next()`, XOR | [LC 136](https://leetcode.com/problems/single-number/) |
| 0332 | [Missing Number](exercises/p0332_missing_number.md) | Easy | building a `HashSet` with `collect`, `contains`, `find` over a range, the sum formula | [LC 268](https://leetcode.com/problems/missing-number/) |
| 0333 | [Intersection of Two Arrays](exercises/p0333_intersection_of_two_arrays.md) | Easy | `HashSet::intersection`, `copied`, results in unspecified order | [LC 349](https://leetcode.com/problems/intersection-of-two-arrays/) |
| 0334 | [Find the Difference of Two Arrays](exercises/p0334_find_the_difference_of_two_arrays.md) | Easy | `HashSet::difference`, `vec![a, b]` of results, unspecified order | [LC 2215](https://leetcode.com/problems/find-the-difference-of-two-arrays/) |
| 0335 | [Compare Course Enrollments](exercises/p0335_compare_course_enrollments.md) | Easy | `HashSet::union`, `intersection`, `difference`, `symmetric_difference`, `is_subset`, `is_disjoint` | Original |
| 0336 | [Path Crossing](exercises/p0336_path_crossing.md) | Easy | `HashSet<(i32, i32)>` of tuples, `insert` returns `bool`, `match` on `char` | [LC 1496](https://leetcode.com/problems/path-crossing/) |
| 0337 | [Unique Email Addresses](exercises/p0337_unique_email_addresses.md) | Easy | `HashSet<String>`, `split_once`, normalising strings before inserting them | [LC 929](https://leetcode.com/problems/unique-email-addresses/) |
| 0338 | [Design HashSet](exercises/p0338_design_hashset.md) | Easy | hashing by hand: buckets `Vec<Vec<i32>>`, a modulo hash, `contains`, `retain` | [LC 705](https://leetcode.com/problems/design-hashset/) |
| 0339 | [Count Words](exercises/p0339_count_words.md) | Easy | `HashMap::new`, `entry(key).or_insert(0)`, `*count += 1`, `get` + `copied` | Original |
| 0340 | [An Inventory with get and get_mut](exercises/p0340_inventory_with_get_mut.md) | Easy | `HashMap::insert` returns the old value, `get`, `get_mut`, `remove`, `contains_key`, `#[derive(Default)]` | Original |
| 0341 | [Two Sum](exercises/p0341_two_sum.md) | Easy | `HashMap<i32, usize>` value -> index, `get` before `insert`, one pass | [LC 1](https://leetcode.com/problems/two-sum/) |
| 0342 | [Valid Anagram](exercises/p0342_valid_anagram.md) | Easy | counting with `entry().or_insert(0)`, comparing two `HashMap`s with `==` | [LC 242](https://leetcode.com/problems/valid-anagram/) |
| 0343 | [Ransom Note](exercises/p0343_ransom_note.md) | Easy | count map with `entry`, consuming counts with `get_mut`, `Iterator::all` | [LC 383](https://leetcode.com/problems/ransom-note/) |
| 0344 | [First Unique Character in a String](exercises/p0344_first_unique_character_in_a_string.md) | Easy | two passes: count with a `HashMap`, then scan in the original order; `position` | [LC 387](https://leetcode.com/problems/first-unique-character-in-a-string/) |
| 0345 | [Majority Element](exercises/p0345_majority_element.md) | Easy | count map, `into_iter().max_by_key`, Boyer-Moore voting (follow-up) | [LC 169](https://leetcode.com/problems/majority-element/) |
| 0346 | [Intersection of Two Arrays II](exercises/p0346_intersection_of_two_arrays_ii.md) | Easy | count map, decrementing through `get_mut`, a match guard on `Option<&mut usize>` | [LC 350](https://leetcode.com/problems/intersection-of-two-arrays-ii/) |
| 0347 | [Unique Number of Occurrences](exercises/p0347_unique_number_of_occurrences.md) | Easy | `HashMap` counts, `values()`, feeding values into a `HashSet` | [LC 1207](https://leetcode.com/problems/unique-number-of-occurrences/) |
| 0348 | [Isomorphic Strings](exercises/p0348_isomorphic_strings.md) | Easy | two `HashMap`s for a bijection, `entry(k).or_insert(v)` as "insert or check" | [LC 205](https://leetcode.com/problems/isomorphic-strings/) |
| 0349 | [Word Pattern](exercises/p0349_word_pattern.md) | Easy | `HashMap<char, &str>` + `HashMap<&str, char>`, borrowing keys from the input, `split_whitespace` | [LC 290](https://leetcode.com/problems/word-pattern/) |
| 0350 | [Group by First Letter](exercises/p0350_group_by_first_letter.md) | Easy | `Entry::or_default`, `and_modify(...).or_insert(...)`, `or_insert_with`, `HashMap<char, Vec<String>>` | Original |
| 0351 | [A Deterministic Report from a HashMap](exercises/p0351_deterministic_report.md) | Easy | unspecified `HashMap` iteration order, collect + sort, `sort_by` with `then_with`, collecting into a `BTreeMap` | Original |
| 0352 | [Design HashMap](exercises/p0352_design_hashmap.md) | Easy | buckets `Vec<Vec<(i32, i32)>>`, `iter_mut().find`, `iter().find`, `retain` | [LC 706](https://leetcode.com/problems/design-hashmap/) |
| 0353 | [Histogram in Key Order](exercises/p0353_histogram_in_key_order.md) | Easy | `BTreeMap` with sorted iteration, `entry().or_insert`, `str::repeat`, `rev()` on a `BTreeMap` | Original |
| 0354 | [Merge Similar Items](exercises/p0354_merge_similar_items.md) | Easy | accumulating into a `BTreeMap`, `chain`, ordered output without sorting | [LC 2363](https://leetcode.com/problems/merge-similar-items/) |
| 0355 | [Third Maximum Number](exercises/p0355_third_maximum_number.md) | Easy | `BTreeSet` as a small sorted window, `pop_first`, `first`, `last` | [LC 414](https://leetcode.com/problems/third-maximum-number/) |
| 0356 | [Intersection of Multiple Arrays](exercises/p0356_intersection_of_multiple_arrays.md) | Easy | `BTreeSet::retain`, `split_first`, sorted output from a set | [LC 2248](https://leetcode.com/problems/intersection-of-multiple-arrays/) |
| 0357 | [Events in a Time Window](exercises/p0357_events_in_a_time_window.md) | Easy | `BTreeMap::range` with `a..b`, `..=b`, `a..` and `(Bound::Excluded(a), Bound::Unbounded)`, `flat_map` | Original |
| 0358 | [Best Bid and Ask](exercises/p0358_best_bid_and_ask.md) | Easy | `BTreeMap::first_key_value`, `last_key_value`, `pop_first`, `pop_last`, `entry().or_insert` | Original |
| 0359 | [Round-Robin Scheduler](exercises/p0359_round_robin_scheduler.md) | Easy | `VecDeque` as a FIFO queue: `push_back`, `pop_front`, `while let`, a queue of queues | Original |
| 0360 | [Number of Recent Calls](exercises/p0360_number_of_recent_calls.md) | Easy | `VecDeque` sliding time window, `front`, `pop_front`, `while let ... if` loop, design struct | [LC 933](https://leetcode.com/problems/number-of-recent-calls/) |
| 0361 | [Implement Stack using Queues](exercises/p0361_implement_stack_using_queues.md) | Easy | `VecDeque` restricted to queue operations, rotating with `pop_front` + `push_back`, `front` | [LC 225](https://leetcode.com/problems/implement-stack-using-queues/) |
| 0362 | [Number of Students Unable to Eat Lunch](exercises/p0362_number_of_students_unable_to_eat_lunch.md) | Easy | `VecDeque` simulation, detecting a stalled queue, counting as a shortcut | [LC 1700](https://leetcode.com/problems/number-of-students-unable-to-eat-lunch/) |
| 0363 | [Bounded Recent History](exercises/p0363_bounded_recent_history.md) | Easy | `VecDeque::push_front`, `pop_back`, `pop_front`, `remove`, `iter().position`, `truncate` | Original |
| 0364 | [Process Jobs by Priority](exercises/p0364_process_jobs_by_priority.md) | Easy | `BinaryHeap` (a max-heap): `push`, `pop`, `peek`, `into_sorted_vec`, tuples as priorities, `Reverse` | Original |
| 0365 | [Last Stone Weight](exercises/p0365_last_stone_weight.md) | Easy | `BinaryHeap::from(Vec)`, repeated `pop` of the two largest, `while let` with a tuple | [LC 1046](https://leetcode.com/problems/last-stone-weight/) |
| 0366 | [Take Gifts From the Richest Pile](exercises/p0366_take_gifts_from_the_richest_pile.md) | Easy | `BinaryHeap` simulation with `pop` + `push`, integer square root, summing into `i64` | [LC 2558](https://leetcode.com/problems/take-gifts-from-the-richest-pile/) |
| 0367 | [The K Weakest Rows in a Matrix](exercises/p0367_the_k_weakest_rows_in_a_matrix.md) | Easy | a max-heap bounded to `k` elements, `(strength, index)` tuples, `into_sorted_vec`, `partition_point` | [LC 1337](https://leetcode.com/problems/the-k-weakest-rows-in-a-matrix/) |
| 0368 | [Kth Largest Element in a Stream](exercises/p0368_kth_largest_element_in_a_stream.md) | Easy | min-heap with `BinaryHeap<Reverse<i32>>`, keeping only `k` elements, `peek`, design struct | [LC 703](https://leetcode.com/problems/kth-largest-element-in-a-stream/) |
| 0369 | [Sort Version Numbers](exercises/p0369_sort_version_numbers.md) | Medium | implementing `Ord` and `PartialOrd` by hand, `Ordering::then_with`, why `#[derive(Ord)]` is sometimes wrong, `max` on `Ord` items | Original |
| 0370 | [Reorder Data in Log Files](exercises/p0370_reorder_data_in_log_files.md) | Medium | a **stable** `sort_by` with a custom `Ordering`, `split_once`, matching on a pair of `bool`s | [LC 937](https://leetcode.com/problems/reorder-data-in-log-files/) |
| 0371 | [Largest Number](exercises/p0371_largest_number.md) | Medium | a comparator that compares concatenations, `Iterator::cmp` on chained byte iterators, `concat` | [LC 179](https://leetcode.com/problems/largest-number/) |
| 0372 | [Custom Sort String](exercises/p0372_custom_sort_string.md) | Medium | a rank table `[usize; 26]`, `sort_by_key` on `Vec<u8>`, `String::from_utf8`, checking answers that allow many outputs | [LC 791](https://leetcode.com/problems/custom-sort-string/) |
| 0373 | [Sort the Students by Their Kth Score](exercises/p0373_sort_the_students_by_their_kth_score.md) | Medium | sorting the rows of a `Vec<Vec<i32>>` by one column, `sort_unstable_by_key` with `Reverse`, moving rows instead of copying | [LC 2545](https://leetcode.com/problems/sort-the-students-by-their-kth-score/) |
| 0374 | [Queue Reconstruction by Height](exercises/p0374_queue_reconstruction_by_height.md) | Medium | two-key `sort_unstable_by` (descending, then ascending), greedy `Vec::insert` at an index | [LC 406](https://leetcode.com/problems/queue-reconstruction-by-height/) |
| 0375 | [Rotate Image](exercises/p0375_rotate_image.md) | Medium | in-place 2-D `Vec` updates, two mutable borrows with `split_at_mut`, `std::mem::swap`, `reverse` | [LC 48](https://leetcode.com/problems/rotate-image/) |
| 0376 | [Group Anagrams](exercises/p0376_group_anagrams.md) | Medium | `HashMap<Vec<u8>, Vec<String>>` with a computed key, `entry().or_default()`, `into_values` | [LC 49](https://leetcode.com/problems/group-anagrams/) |
| 0377 | [Determine if Two Strings Are Close](exercises/p0377_determine_if_two_strings_are_close.md) | Medium | count arrays `[usize; 26]`, comparing key sets and sorted counts, reasoning about invariants | [LC 1657](https://leetcode.com/problems/determine-if-two-strings-are-close/) |
| 0378 | [Equal Row and Column Pairs](exercises/p0378_equal_row_and_column_pairs.md) | Medium | `HashMap<&[i32], i32>` keyed by borrowed slices, building columns, `get` with a slice key | [LC 2352](https://leetcode.com/problems/equal-row-and-column-pairs/) |
| 0379 | [Subarray Sum Equals K](exercises/p0379_subarray_sum_equals_k.md) | Medium | running prefix sums, `HashMap<i32, i32>` counting earlier prefixes, `get` before `entry` | [LC 560](https://leetcode.com/problems/subarray-sum-equals-k/) |
| 0380 | [Longest Consecutive Sequence](exercises/p0380_longest_consecutive_sequence.md) | Medium | `HashSet` membership in `O(1)`, starting runs only at their first element, linear-time reasoning | [LC 128](https://leetcode.com/problems/longest-consecutive-sequence/) |
| 0381 | [Top K Frequent Elements](exercises/p0381_top_k_frequent_elements.md) | Medium | counting with a `HashMap`, a min-heap of `Reverse((count, value))` bounded to `k`, top-k pattern | [LC 347](https://leetcode.com/problems/top-k-frequent-elements/) |
| 0382 | [Sort Characters By Frequency](exercises/p0382_sort_characters_by_frequency.md) | Medium | `HashMap<char, usize>` counts into a `Vec<(char, usize)>`, `sort_unstable_by`, `String` building with `extend` + `repeat_n` | [LC 451](https://leetcode.com/problems/sort-characters-by-frequency/) |
| 0383 | [Top K Frequent Words](exercises/p0383_top_k_frequent_words.md) | Medium | `HashMap<&str, usize>` borrowing words, a two-level comparator (count descending, word ascending), `truncate` | [LC 692](https://leetcode.com/problems/top-k-frequent-words/) |
| 0384 | [Hand of Straights](exercises/p0384_hand_of_straights.md) | Medium | `BTreeMap` counts, `first_key_value` for the smallest key, decrement-and-remove with `get_mut`, `while let` | [LC 846](https://leetcode.com/problems/hand-of-straights/) |
| 0385 | [Display Table of Food Orders in a Restaurant](exercises/p0385_display_table_of_food_orders_in_a_restaurant.md) | Medium | `BTreeSet<&str>` for sorted column names, `BTreeMap<u32, HashMap<&str, u32>>` for sorted rows, building a `Vec<Vec<String>>` | [LC 1418](https://leetcode.com/problems/display-table-of-food-orders-in-a-restaurant/) |
| 0386 | [Smallest Number in Infinite Set](exercises/p0386_smallest_number_in_infinite_set.md) | Medium | `BTreeSet::pop_first`, representing an infinite set with a counter plus a small set, design struct | [LC 2336](https://leetcode.com/problems/smallest-number-in-infinite-set/) |
| 0387 | [My Calendar I](exercises/p0387_my_calendar_i.md) | Medium | `BTreeMap<i32, i32>` of intervals, `range(..end).next_back()` to find the neighbour, half-open intervals | [LC 729](https://leetcode.com/problems/my-calendar-i/) |
| 0388 | [Time Based Key-Value Store](exercises/p0388_time_based_key_value_store.md) | Medium | nested collections `HashMap<String, BTreeMap<i32, String>>`, `range(..=t).next_back()`, `and_then` on `Option` | [LC 981](https://leetcode.com/problems/time-based-key-value-store/) |
| 0389 | [Reveal Cards In Increasing Order](exercises/p0389_reveal_cards_in_increasing_order.md) | Medium | simulating a process on a `VecDeque` of **positions**, `pop_front` + `push_back`, filling a result by index | [LC 950](https://leetcode.com/problems/reveal-cards-in-increasing-order/) |
| 0390 | [Dota2 Senate](exercises/p0390_dota2_senate.md) | Medium | two `VecDeque` queues of turn numbers, greedy simulation, re-queueing with an offset | [LC 649](https://leetcode.com/problems/dota2-senate/) |
| 0391 | [Design Circular Deque](exercises/p0391_design_circular_deque.md) | Medium | a capacity-limited `VecDeque`, `push_front`/`push_back`/`pop_front`/`pop_back`, `front`/`back`, design struct | [LC 641](https://leetcode.com/problems/design-circular-deque/) |
| 0392 | [Kth Largest Element in an Array](exercises/p0392_kth_largest_element_in_an_array.md) | Medium | a min-heap of size `k` with `Reverse`, `BinaryHeap::peek`, `select_nth_unstable` (quickselect) | [LC 215](https://leetcode.com/problems/kth-largest-element-in-an-array/) |
| 0393 | [K Closest Points to Origin](exercises/p0393_k_closest_points_to_origin.md) | Medium | a max-heap of `(distance, index)` bounded to `k`, comparing squared distances, `into_iter` over a heap | [LC 973](https://leetcode.com/problems/k-closest-points-to-origin/) |
| 0394 | [Seat Reservation Manager](exercises/p0394_seat_reservation_manager.md) | Medium | `BinaryHeap<Reverse<i32>>` as a min-heap, collecting a range into a heap, design struct | [LC 1845](https://leetcode.com/problems/seat-reservation-manager/) |
| 0395 | [Emergency Room Triage](exercises/p0395_emergency_room_triage.md) | Medium | `impl Ord` for a struct stored in a `BinaryHeap`, keeping `Ord` consistent with `Eq`, `peek`, `clone` + `into_sorted_vec` | Original |
| 0396 | [Merge K Sorted Vectors](exercises/p0396_merge_k_sorted_vectors.md) | Medium | k-way merge with `BinaryHeap<Reverse<(i32, usize, usize)>>`, `get` for safe look-ahead, laziness | Original |
| 0397 | [Task Scheduler](exercises/p0397_task_scheduler.md) | Medium | counting into `[usize; 26]`, a max-heap of remaining counts, a cooldown `VecDeque<(ready_time, count)>` | [LC 621](https://leetcode.com/problems/task-scheduler/) |
| 0398 | [Reorganize String](exercises/p0398_reorganize_string.md) | Medium | max-heap of `(count, byte)`, holding the previous character back with `Option::take`, impossible-case detection | [LC 767](https://leetcode.com/problems/reorganize-string/) |
| 0399 | [Design a Food Rating System](exercises/p0399_design_a_food_rating_system.md) | Medium | two indexes kept in sync: `HashMap<String, (String, i32)>` + `HashMap<String, BTreeSet<(Reverse<i32>, String)>>`, ordered-set updates | [LC 2353](https://leetcode.com/problems/design-a-food-rating-system/) |
| 0400 | [Design Twitter](exercises/p0400_design_twitter.md) | Medium | `HashMap<i32, Vec<(u32, i32)>>` + `HashMap<i32, HashSet<i32>>`, k-way merge with a `BinaryHeap`, a global clock | [LC 355](https://leetcode.com/problems/design-twitter/) |

<!-- PROBLEM_TABLE:END -->
