# Level 8: Algorithm Techniques (problems 0701-0800)

> Focus: two pointers, sliding window, prefix sums, binary search, stacks, BFS/DFS, backtracking, greedy, bits.

This level is about the classic techniques behind most interview-style problems: two pointers,
sliding windows, prefix sums, binary search (including binary search on the answer), stacks and
monotonic stacks, intervals, greedy choices, sorting algorithms, matrix walks, bit manipulation,
DFS/BFS on grids and graphs, and backtracking. Problems are grouped by technique and go from Easy
to Medium inside each group; every solution names its technique, explains why it applies, and
states its complexity. 95 problems come from LeetCode (93 with LeetCode's exact Rust signature and
2 adapted; 13 Easy and 82 Medium), and 5 original problems build small reusable Rust tools: a
three-way partition, lower/upper bounds, an overflow-safe integer root, inversion counting and grid
neighbour iterators. Hard problems are kept for level 10.

## Prerequisites

- Levels 1-7, or the same knowledge: ownership and borrowing, slices, `String` vs `&str`, structs
  and enums, the standard collections (`Vec`, `HashMap`, `VecDeque`), closures, iterator adapters,
  and generics with trait bounds.
- Comfort reading `usize` index arithmetic: many bugs in this level are underflows such as
  `i - 1` at `i == 0`. Re-read [integer overflow in The Rust Book 3.2](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow)
  if needed.
- Dynamic programming, weighted graphs (Dijkstra, topological sort, union-find) and concurrency
  come in level 9.

## What you will practice

- Two pointers and sliding windows over slices: half-open ranges `lo..hi`, `as_bytes`,
  `windows`, `zip`, letter-count arrays `[u32; 26]`, and comparisons with `cmp` and
  [`Ordering`](https://doc.rust-lang.org/std/cmp/enum.Ordering.html):
  [slice methods](https://doc.rust-lang.org/std/primitive.slice.html).
- Prefix sums, difference arrays and "count earlier prefixes" with the
  [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html) entry API, and
  [`rem_euclid`](https://doc.rust-lang.org/std/primitive.i32.html#method.rem_euclid) for negative
  remainders.
- Binary search written by hand and with
  [`partition_point`](https://doc.rust-lang.org/std/primitive.slice.html#method.partition_point) /
  [`binary_search`](https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search), and
  binary search on the answer with overflow-safe arithmetic
  ([`checked_pow`](https://doc.rust-lang.org/std/primitive.u64.html#method.checked_pow),
  [`div_ceil`](https://doc.rust-lang.org/std/primitive.u64.html#method.div_ceil), widening to
  `i64` / `u64`).
- `Vec` as a stack (`push`, `last`, `last_mut`, and
  [`pop_if`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.pop_if)) for monotonic
  stacks and simulations, and
  [`VecDeque`](https://doc.rust-lang.org/std/collections/struct.VecDeque.html) as the BFS queue.
- Greedy algorithms and interval sweeps with `sort_unstable_by_key`,
  [`Reverse`](https://doc.rust-lang.org/std/cmp/struct.Reverse.html) and
  [`Peekable::next_if`](https://doc.rust-lang.org/std/iter/struct.Peekable.html#method.next_if).
- Sorting algorithms by hand: merge sort on sub-slices with `split_at` and `copy_from_slice`,
  counting sort, and radix sort with
  [`std::mem::swap`](https://doc.rust-lang.org/std/mem/fn.swap.html).
- Matrices: in-place markers, diagonals, spirals with signed coordinates, rotating layers with
  `rotate_left`, and two mutable borrows of one grid with
  [`split_at_mut`](https://doc.rust-lang.org/std/primitive.slice.html#method.split_at_mut).
- Bit manipulation: `&`, `|`, `^`, `!`, `<<`, `>>`
  ([The Rust Reference](https://doc.rust-lang.org/reference/expressions/operator-expr.html#arithmetic-and-logical-binary-operators)),
  `count_ones`, `reverse_bits`, `wrapping_neg` and two's complement:
  [`u32` methods](https://doc.rust-lang.org/std/primitive.u32.html).
- Grid and graph search: safe neighbours with
  [`checked_add_signed`](https://doc.rust-lang.org/std/primitive.usize.html#method.checked_add_signed),
  [`let ... else`](https://doc.rust-lang.org/rust-by-example/flow_control/let_else.html), returning
  [`impl Iterator`](https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits),
  DFS with an explicit stack or recursion, multi-source BFS, and reverse flood fills.
- Backtracking (choose / explore / un-choose) with one shared `&mut Vec` buffer, `used` masks,
  rules that skip duplicates, and pruning:
  [The Rust Book 4.2: references and borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html).

## Roadmap

| Problems | Theme |
| -------- | ----- |
| 0701-0709 | Two pointers: palindromes, sorted pairs, greedy pairing, three-way partition, 3Sum / 3Sum Closest / 4Sum |
| 0710-0719 | Sliding window: fixed windows, variable windows with a budget, letter counts, anagrams, counting windows |
| 0720-0727 | Prefix sums: running totals, prefix + suffix counts, difference arrays, prefix remainders with maps, 2-D prefix sums |
| 0728-0740 | Binary search: exact search, boundaries, rotated arrays, peaks, binary search on the answer, a design problem |
| 0741-0749 | Stacks and monotonic stacks: next greater / smaller elements, simulations, greedy digit removal, contribution counting |
| 0750-0754 | Intervals: merging, inserting, intersecting, greedy by earliest end |
| 0755-0761 | Greedy: matching, collecting rises, jump games as BFS levels, restarts, cuts, sorting by cost difference |
| 0762-0765 | Sorting algorithms: merge sort, counting inversions, counting sort, radix sort |
| 0766-0769 | Matrices: in-place markers, diagonal and spiral walks, rotating layers |
| 0770-0777 | Bit manipulation: XOR, population count, `n & (n - 1)`, reversing bits, bit counts modulo 3, adding with carries |
| 0778-0789 | DFS and BFS: neighbour iterators, flood fills, islands, border floods, graph reachability, multi-source BFS, shortest paths, Word Search |
| 0790-0800 | Backtracking: subsets, case permutations, combinations, permutations, combination sums, parentheses, palindrome partitions |

## How to work on a problem

```bash
cd learn_11_leetcode_practice/level_08_algorithms

cargo test practice::p0701      # run the tests for the problem you are solving
cargo test solutions::p0701     # run the tests against the reference solution
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
| 0701 | [Valid Palindrome II](exercises/p0701_valid_palindrome_ii.md) | Easy | two pointers, byte slices `&[u8]`, half-open ranges, `Iterator::eq` | [LC 680](https://leetcode.com/problems/valid-palindrome-ii/) |
| 0702 | [Two Sum II - Input Array Is Sorted](exercises/p0702_two_sum_ii_input_array_is_sorted.md) | Medium | two pointers on a sorted array, `match` on `Ordering`, `cmp` | [LC 167](https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/) |
| 0703 | [Boats to Save People](exercises/p0703_boats_to_save_people.md) | Medium | sorting + two pointers, greedy pairing, `sort_unstable` | [LC 881](https://leetcode.com/problems/boats-to-save-people/) |
| 0704 | [Max Number of K-Sum Pairs](exercises/p0704_max_number_of_k_sum_pairs.md) | Medium | sorting + two pointers, `match` on `Ordering`, `i64` sums | [LC 1679](https://leetcode.com/problems/max-number-of-k-sum-pairs/) |
| 0705 | [Container With Most Water](exercises/p0705_container_with_most_water.md) | Medium | two pointers, greedy elimination argument, `max` | [LC 11](https://leetcode.com/problems/container-with-most-water/) |
| 0706 | [Three-Way Partition](exercises/p0706_three_way_partition.md) | Medium | Dutch national flag (three pointers), generic `T: Ord`, `slice::swap`, loop invariants | Original |
| 0707 | [3Sum](exercises/p0707_3sum.md) | Medium | sorting + two pointers, skipping duplicates, `enumerate`, `match` on `Ordering` | [LC 15](https://leetcode.com/problems/3sum/) |
| 0708 | [3Sum Closest](exercises/p0708_3sum_closest.md) | Medium | sorting + two pointers, tracking the best candidate, `abs`, early return | [LC 16](https://leetcode.com/problems/3sum-closest/) |
| 0709 | [4Sum](exercises/p0709_4sum.md) | Medium | sorting + nested two pointers, skipping duplicates, `i64` to avoid overflow | [LC 18](https://leetcode.com/problems/4sum/) |
| 0710 | [Maximum Average Subarray I](exercises/p0710_maximum_average_subarray_i.md) | Easy | fixed-size sliding window, `zip`, `f64` conversion | [LC 643](https://leetcode.com/problems/maximum-average-subarray-i/) |
| 0711 | [Maximum Number of Vowels in a Substring of Given Length](exercises/p0711_maximum_number_of_vowels_in_a_substring_of_given_length.md) | Medium | fixed-size sliding window, `matches!`, `usize::from(bool)`, `zip` | [LC 1456](https://leetcode.com/problems/maximum-number-of-vowels-in-a-substring-of-given-length/) |
| 0712 | [Longest Substring Without Repeating Characters](exercises/p0712_longest_substring_without_repeating_characters.md) | Medium | variable-size sliding window, last-seen table `[Option<usize>; 256]`, `if let` | [LC 3](https://leetcode.com/problems/longest-substring-without-repeating-characters/) |
| 0713 | [Minimum Size Subarray Sum](exercises/p0713_minimum_size_subarray_sum.md) | Medium | shrinking sliding window, positive values, `usize::MAX` as "not found" | [LC 209](https://leetcode.com/problems/minimum-size-subarray-sum/) |
| 0714 | [Max Consecutive Ones III](exercises/p0714_max_consecutive_ones_iii.md) | Medium | sliding window with a budget, counting inside the window | [LC 1004](https://leetcode.com/problems/max-consecutive-ones-iii/) |
| 0715 | [Fruit Into Baskets](exercises/p0715_fruit_into_baskets.md) | Medium | sliding window with at most two distinct values, count table `Vec<usize>` | [LC 904](https://leetcode.com/problems/fruit-into-baskets/) |
| 0716 | [Longest Repeating Character Replacement](exercises/p0716_longest_repeating_character_replacement.md) | Medium | sliding window, count array `[usize; 26]`, the "window never shrinks" trick | [LC 424](https://leetcode.com/problems/longest-repeating-character-replacement/) |
| 0717 | [Permutation in String](exercises/p0717_permutation_in_string.md) | Medium | fixed-size sliding window, letter counts `[u32; 26]`, comparing arrays with `==` | [LC 567](https://leetcode.com/problems/permutation-in-string/) |
| 0718 | [Find All Anagrams in a String](exercises/p0718_find_all_anagrams_in_a_string.md) | Medium | fixed-size sliding window, letter counts, collecting window start indices | [LC 438](https://leetcode.com/problems/find-all-anagrams-in-a-string/) |
| 0719 | [Subarray Product Less Than K](exercises/p0719_subarray_product_less_than_k.md) | Medium | sliding window that counts subarrays, `i64` products, guard clauses | [LC 713](https://leetcode.com/problems/subarray-product-less-than-k/) |
| 0720 | [Find Pivot Index](exercises/p0720_find_pivot_index.md) | Easy | prefix sums, total minus prefix, `iter().sum()`, `enumerate` | [LC 724](https://leetcode.com/problems/find-pivot-index/) |
| 0721 | [Minimum Penalty for a Shop](exercises/p0721_minimum_penalty_for_a_shop.md) | Medium | prefix + suffix counts, running update, earliest minimum | [LC 2483](https://leetcode.com/problems/minimum-penalty-for-a-shop/) |
| 0722 | [Corporate Flight Bookings](exercises/p0722_corporate_flight_bookings.md) | Medium | difference array, prefix sums with `scan`, slice patterns `[a, b, c]` | [LC 1109](https://leetcode.com/problems/corporate-flight-bookings/) |
| 0723 | [Contiguous Array](exercises/p0723_contiguous_array.md) | Medium | prefix balance, first occurrence in a `HashMap`, `entry().or_insert` | [LC 525](https://leetcode.com/problems/contiguous-array/) |
| 0724 | [Continuous Subarray Sum](exercises/p0724_continuous_subarray_sum.md) | Medium | prefix remainders, `HashMap` of first positions, `i64` to avoid overflow | [LC 523](https://leetcode.com/problems/continuous-subarray-sum/) |
| 0725 | [Subarray Sums Divisible by K](exercises/p0725_subarray_sums_divisible_by_k.md) | Medium | prefix remainders, counting equal pairs, `rem_euclid` for negative numbers | [LC 974](https://leetcode.com/problems/subarray-sums-divisible-by-k/) |
| 0726 | [Binary Subarrays With Sum](exercises/p0726_binary_subarrays_with_sum.md) | Medium | prefix sums + `HashMap` of counts ("subarray sum equals k"), `entry().or_insert` | [LC 930](https://leetcode.com/problems/binary-subarrays-with-sum/) |
| 0727 | [Matrix Block Sum](exercises/p0727_matrix_block_sum.md) | Medium | 2-D prefix sums, inclusion-exclusion, `saturating_sub`, nested `map`/`collect` | [LC 1314](https://leetcode.com/problems/matrix-block-sum/) |
| 0728 | [Binary Search](exercises/p0728_binary_search.md) | Easy | binary search, half-open bounds `lo..hi`, `match` on `Ordering`, overflow-free midpoint | [LC 704](https://leetcode.com/problems/binary-search/) |
| 0729 | [Find Smallest Letter Greater Than Target](exercises/p0729_find_smallest_letter_greater_than_target.md) | Easy | binary search for an upper bound, wrapping around, `char` comparison | [LC 744](https://leetcode.com/problems/find-smallest-letter-greater-than-target/) |
| 0730 | [Lower and Upper Bound](exercises/p0730_lower_and_upper_bound.md) | Easy | `slice::partition_point`, `slice::binary_search` and its `Result`, generic `T: Ord`, `Vec::insert` | Original |
| 0731 | [Find First and Last Position of Element in Sorted Array](exercises/p0731_find_first_and_last_position_of_element_in_sorted_array.md) | Medium | two boundary binary searches, `partition_point`, `slice::get` | [LC 34](https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/) |
| 0732 | [Search a 2D Matrix](exercises/p0732_search_a_2d_matrix.md) | Medium | binary search over a virtual flattened array, index arithmetic `k / n`, `k % n` | [LC 74](https://leetcode.com/problems/search-a-2d-matrix/) |
| 0733 | [Find Peak Element](exercises/p0733_find_peak_element.md) | Medium | binary search on a slope (unsorted input), invariants, checking "any valid answer" | [LC 162](https://leetcode.com/problems/find-peak-element/) |
| 0734 | [Find Minimum in Rotated Sorted Array](exercises/p0734_find_minimum_in_rotated_sorted_array.md) | Medium | binary search on a rotated array, comparing with the right end, `rotate_left` in tests | [LC 153](https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/) |
| 0735 | [Search in Rotated Sorted Array](exercises/p0735_search_in_rotated_sorted_array.md) | Medium | binary search, deciding which half is sorted, half-open bounds | [LC 33](https://leetcode.com/problems/search-in-rotated-sorted-array/) |
| 0736 | [Integer N-th Root](exercises/p0736_integer_nth_root.md) | Medium | binary search on the answer, monotonic predicate, `checked_pow`, `Option::is_some_and` | Original |
| 0737 | [Koko Eating Bananas](exercises/p0737_koko_eating_bananas.md) | Medium | binary search on the answer, feasibility check, `u64::div_ceil`, `u64` sums | [LC 875](https://leetcode.com/problems/koko-eating-bananas/) |
| 0738 | [Capacity To Ship Packages Within D Days](exercises/p0738_capacity_to_ship_packages_within_d_days.md) | Medium | binary search on the answer, greedy feasibility check, iterator `max` and `sum` | [LC 1011](https://leetcode.com/problems/capacity-to-ship-packages-within-d-days/) |
| 0739 | [Minimum Number of Days to Make m Bouquets](exercises/p0739_minimum_number_of_days_to_make_m_bouquets.md) | Medium | binary search on the answer, counting runs, `u64` overflow guard | [LC 1482](https://leetcode.com/problems/minimum-number-of-days-to-make-m-bouquets/) |
| 0740 | [Online Election](exercises/p0740_online_election.md) | Medium | design struct, precomputation + binary search, `partition_point`, private fields | [LC 911](https://leetcode.com/problems/online-election/) |
| 0741 | [Final Prices With a Special Discount in a Shop](exercises/p0741_final_prices_with_a_special_discount_in_a_shop.md) | Easy | monotonic stack of indices, `Vec::pop_if`, `while let` | [LC 1475](https://leetcode.com/problems/final-prices-with-a-special-discount-in-a-shop/) |
| 0742 | [Next Greater Element I](exercises/p0742_next_greater_element_i.md) | Easy | monotonic decreasing stack, `HashMap` for lookups, `pop_if` | [LC 496](https://leetcode.com/problems/next-greater-element-i/) |
| 0743 | [Daily Temperatures](exercises/p0743_daily_temperatures.md) | Medium | monotonic stack of indices, distances between indices, `pop_if` | [LC 739](https://leetcode.com/problems/daily-temperatures/) |
| 0744 | [Next Greater Element II](exercises/p0744_next_greater_element_ii.md) | Medium | circular monotonic stack, two passes with `i % n`, `pop_if` | [LC 503](https://leetcode.com/problems/next-greater-element-ii/) |
| 0745 | [Asteroid Collision](exercises/p0745_asteroid_collision.md) | Medium | stack simulation, `match` with guards on `last()`, `Ordering` | [LC 735](https://leetcode.com/problems/asteroid-collision/) |
| 0746 | [Minimum Remove to Make Valid Parentheses](exercises/p0746_minimum_remove_to_make_valid_parentheses.md) | Medium | stack of indices, marking with a `Vec<bool>`, `zip` + `filter`, building a `String` | [LC 1249](https://leetcode.com/problems/minimum-remove-to-make-valid-parentheses/) |
| 0747 | [Remove K Digits](exercises/p0747_remove_k_digits.md) | Medium | greedy monotonic stack, `truncate`, `skip_while` for leading zeros | [LC 402](https://leetcode.com/problems/remove-k-digits/) |
| 0748 | [Car Fleet](exercises/p0748_car_fleet.md) | Medium | sorting by position, monotonic stack of arrival times, exact comparison of fractions | [LC 853](https://leetcode.com/problems/car-fleet/) |
| 0749 | [Sum of Subarray Minimums](exercises/p0749_sum_of_subarray_minimums.md) | Medium | previous / next smaller element with monotonic stacks, contribution counting, modular sums | [LC 907](https://leetcode.com/problems/sum-of-subarray-minimums/) |
| 0750 | [Merge Intervals](exercises/p0750_merge_intervals.md) | Medium | sort by start, sweep with `last_mut`, `match` with a guard | [LC 56](https://leetcode.com/problems/merge-intervals/) |
| 0751 | [Insert Interval](exercises/p0751_insert_interval.md) | Medium | three-phase scan of sorted intervals, `Peekable::next_if`, `extend` | [LC 57](https://leetcode.com/problems/insert-interval/) |
| 0752 | [Interval List Intersections](exercises/p0752_interval_list_intersections.md) | Medium | two pointers over two sorted lists, `max`/`min` of bounds | [LC 986](https://leetcode.com/problems/interval-list-intersections/) |
| 0753 | [Non-overlapping Intervals](exercises/p0753_non_overlapping_intervals.md) | Medium | greedy by earliest end (interval scheduling), `sort_unstable_by_key` | [LC 435](https://leetcode.com/problems/non-overlapping-intervals/) |
| 0754 | [Minimum Number of Arrows to Burst Balloons](exercises/p0754_minimum_number_of_arrows_to_burst_balloons.md) | Medium | greedy by earliest end, extreme `i32` values without overflow, `Option` state | [LC 452](https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/) |
| 0755 | [Assign Cookies](exercises/p0755_assign_cookies.md) | Easy | greedy matching, sorting both inputs, two pointers | [LC 455](https://leetcode.com/problems/assign-cookies/) |
| 0756 | [Best Time to Buy and Sell Stock II](exercises/p0756_best_time_to_buy_and_sell_stock_ii.md) | Medium | greedy sum of rises, `windows(2)`, iterator `map` + `sum` | [LC 122](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/) |
| 0757 | [Jump Game](exercises/p0757_jump_game.md) | Medium | greedy farthest reach, early return | [LC 55](https://leetcode.com/problems/jump-game/) |
| 0758 | [Jump Game II](exercises/p0758_jump_game_ii.md) | Medium | greedy BFS by levels, window `current_end`, `take(n - 1)` | [LC 45](https://leetcode.com/problems/jump-game-ii/) |
| 0759 | [Gas Station](exercises/p0759_gas_station.md) | Medium | greedy restart, running sums, `zip` + `enumerate` | [LC 134](https://leetcode.com/problems/gas-station/) |
| 0760 | [Partition Labels](exercises/p0760_partition_labels.md) | Medium | last occurrence table, greedy cut, interval-style merging on a string | [LC 763](https://leetcode.com/problems/partition-labels/) |
| 0761 | [Two City Scheduling](exercises/p0761_two_city_scheduling.md) | Medium | greedy by cost difference, `sort_unstable_by_key`, `enumerate` + `map` + `sum` | [LC 1029](https://leetcode.com/problems/two-city-scheduling/) |
| 0762 | [Sort an Array](exercises/p0762_sort_an_array.md) | Medium | merge sort, recursion on sub-slices, one reusable buffer, `split_at`, `copy_from_slice` | [LC 912](https://leetcode.com/problems/sort-an-array/) |
| 0763 | [Count Inversions](exercises/p0763_count_inversions.md) | Medium | merge sort with counting (divide and conquer), `u64` results, `to_vec` | Original |
| 0764 | [H-Index](exercises/p0764_h_index.md) | Medium | counting sort / buckets, capping values with `min`, reverse scan with `enumerate().rev()` | [LC 274](https://leetcode.com/problems/h-index/) |
| 0765 | [Maximum Gap](exercises/p0765_maximum_gap.md) | Medium | radix sort (LSD, base 256), counting with prefix positions, `std::mem::swap`, `windows` | [LC 164](https://leetcode.com/problems/maximum-gap/) |
| 0766 | [Set Matrix Zeroes](exercises/p0766_set_matrix_zeroes.md) | Medium | in-place markers in the first row and column, `split_at_mut`, `fill`, `&mut Vec<Vec<i32>>` | [LC 73](https://leetcode.com/problems/set-matrix-zeroes/) |
| 0767 | [Diagonal Traverse](exercises/p0767_diagonal_traverse.md) | Medium | index arithmetic on anti-diagonals (`r + c = d`), inclusive ranges, `rev`, `extend` | [LC 498](https://leetcode.com/problems/diagonal-traverse/) |
| 0768 | [Spiral Matrix III](exercises/p0768_spiral_matrix_iii.md) | Medium | simulation with direction vectors, signed coordinates, `Range::contains` | [LC 885](https://leetcode.com/problems/spiral-matrix-iii/) |
| 0769 | [Cyclically Rotating a Grid](exercises/p0769_cyclically_rotating_a_grid.md) | Medium | matrix layers, collecting a layer's positions, `slice::rotate_left`, `k % len` | [LC 1914](https://leetcode.com/problems/cyclically-rotating-a-grid/) |
| 0770 | [Hamming Distance](exercises/p0770_hamming_distance.md) | Easy | bit manipulation, XOR `^`, `count_ones`, clearing the lowest set bit | [LC 461](https://leetcode.com/problems/hamming-distance/) |
| 0771 | [Number of 1 Bits](exercises/p0771_number_of_1_bits.md) | Easy | bit manipulation, `n & (n - 1)` clears the lowest set bit, `u32` | [Adapted from LC 191](https://leetcode.com/problems/number-of-1-bits/) |
| 0772 | [Counting Bits](exercises/p0772_counting_bits.md) | Easy | bit manipulation + dynamic programming, `i >> 1`, `i & 1` | [LC 338](https://leetcode.com/problems/counting-bits/) |
| 0773 | [Reverse Bits](exercises/p0773_reverse_bits.md) | Easy | bit manipulation, shifting bits out and in, `u32::reverse_bits` | [Adapted from LC 190](https://leetcode.com/problems/reverse-bits/) |
| 0774 | [Minimum Flips to Make a OR b Equal to c](exercises/p0774_minimum_flips_to_make_a_or_b_equal_to_c.md) | Medium | per-bit case analysis, extracting a bit with `(x >> i) & 1`, `match` on a bit | [LC 1318](https://leetcode.com/problems/minimum-flips-to-make-a-or-b-equal-to-c/) |
| 0775 | [Single Number II](exercises/p0775_single_number_ii.md) | Medium | counting bits modulo 3, arithmetic shift of negative numbers, `fold` | [LC 137](https://leetcode.com/problems/single-number-ii/) |
| 0776 | [Single Number III](exercises/p0776_single_number_iii.md) | Medium | XOR of everything, isolating the lowest set bit `x & x.wrapping_neg()`, partitioning | [LC 260](https://leetcode.com/problems/single-number-iii/) |
| 0777 | [Sum of Two Integers](exercises/p0777_sum_of_two_integers.md) | Medium | addition with XOR and carries, `<<` on signed integers, two's complement | [LC 371](https://leetcode.com/problems/sum-of-two-integers/) |
| 0778 | [Grid Neighbors](exercises/p0778_grid_neighbors.md) | Easy | returning `impl Iterator`, `usize::checked_add_signed`, `filter_map` with `?`, `bool::then_some` | Original |
| 0779 | [Flood Fill](exercises/p0779_flood_fill.md) | Easy | DFS on a grid with an explicit stack, `let ... else`, `checked_add_signed` | [LC 733](https://leetcode.com/problems/flood-fill/) |
| 0780 | [Number of Islands](exercises/p0780_number_of_islands.md) | Medium | connected components with DFS, sinking visited land, `Vec<Vec<char>>` | [LC 200](https://leetcode.com/problems/number-of-islands/) |
| 0781 | [Max Area of Island](exercises/p0781_max_area_of_island.md) | Medium | recursive DFS returning a count, `&mut [Vec<i32>]`, `checked_sub` + `map_or` | [LC 695](https://leetcode.com/problems/max-area-of-island/) |
| 0782 | [Number of Enclaves](exercises/p0782_number_of_enclaves.md) | Medium | flood fill from the border, iterator `chain` of border cells, counting with `flatten` | [LC 1020](https://leetcode.com/problems/number-of-enclaves/) |
| 0783 | [Surrounded Regions](exercises/p0783_surrounded_regions.md) | Medium | border flood fill with a temporary marker, in-place `&mut Vec<Vec<char>>`, `iter_mut().flatten()` | [LC 130](https://leetcode.com/problems/surrounded-regions/) |
| 0784 | [Pacific Atlantic Water Flow](exercises/p0784_pacific_atlantic_water_flow.md) | Medium | reverse flood fill from two sources, `Vec<Vec<bool>>`, helper taking `impl Iterator` | [LC 417](https://leetcode.com/problems/pacific-atlantic-water-flow/) |
| 0785 | [Keys and Rooms](exercises/p0785_keys_and_rooms.md) | Medium | graph DFS with an adjacency list, `visited: Vec<bool>`, `extend` from an iterator | [LC 841](https://leetcode.com/problems/keys-and-rooms/) |
| 0786 | [Rotting Oranges](exercises/p0786_rotting_oranges.md) | Medium | multi-source BFS level by level, `VecDeque`, counting what is left | [LC 994](https://leetcode.com/problems/rotting-oranges/) |
| 0787 | [01 Matrix](exercises/p0787_01_matrix.md) | Medium | multi-source BFS for distances, `VecDeque`, sentinel for "not visited yet" | [LC 542](https://leetcode.com/problems/01-matrix/) |
| 0788 | [Shortest Path in Binary Matrix](exercises/p0788_shortest_path_in_binary_matrix.md) | Medium | BFS for shortest paths, 8 directions, `VecDeque` of `(row, col, length)` | [LC 1091](https://leetcode.com/problems/shortest-path-in-binary-matrix/) |
| 0789 | [Word Search](exercises/p0789_word_search.md) | Medium | DFS with backtracking on a grid, temporary marking and restoring, pruning, `Iterator::any` | [LC 79](https://leetcode.com/problems/word-search/) |
| 0790 | [Subsets](exercises/p0790_subsets.md) | Medium | backtracking (choose / explore / un-choose), recursion with `&mut Vec`, `clone` of the path | [LC 78](https://leetcode.com/problems/subsets/) |
| 0791 | [Subsets II](exercises/p0791_subsets_ii.md) | Medium | backtracking with duplicates, sort first, "skip equal siblings" rule | [LC 90](https://leetcode.com/problems/subsets-ii/) |
| 0792 | [Letter Case Permutation](exercises/p0792_letter_case_permutation.md) | Medium | backtracking over positions, binary choices, ASCII case methods on `u8` | [LC 784](https://leetcode.com/problems/letter-case-permutation/) |
| 0793 | [Combinations](exercises/p0793_combinations.md) | Medium | backtracking with pruning (enough numbers left), inclusive ranges, `Vec::with_capacity` | [LC 77](https://leetcode.com/problems/combinations/) |
| 0794 | [Permutations](exercises/p0794_permutations.md) | Medium | backtracking with a `used` mask, `Vec<bool>`, recursion depth `n` | [LC 46](https://leetcode.com/problems/permutations/) |
| 0795 | [Permutations II](exercises/p0795_permutations_ii.md) | Medium | backtracking with duplicates, sorted input, the "previous equal value must be used" rule | [LC 47](https://leetcode.com/problems/permutations-ii/) |
| 0796 | [Combination Sum](exercises/p0796_combination_sum.md) | Medium | backtracking with unlimited reuse, sorted pruning with `break`, remaining-target recursion | [LC 39](https://leetcode.com/problems/combination-sum/) |
| 0797 | [Combination Sum II](exercises/p0797_combination_sum_ii.md) | Medium | backtracking, each candidate used once, skipping equal siblings, sorted pruning | [LC 40](https://leetcode.com/problems/combination-sum-ii/) |
| 0798 | [Combination Sum III](exercises/p0798_combination_sum_iii.md) | Medium | backtracking with two constraints (size and sum), pruning on both, `i32` ranges | [LC 216](https://leetcode.com/problems/combination-sum-iii/) |
| 0799 | [Generate Parentheses](exercises/p0799_generate_parentheses.md) | Medium | backtracking with counters, `String::push` / `pop` as the path, Catalan numbers | [LC 22](https://leetcode.com/problems/generate-parentheses/) |
| 0800 | [Palindrome Partitioning](exercises/p0800_palindrome_partitioning.md) | Medium | backtracking over cut positions, precomputed palindrome table, `&str` slicing, `Vec<Vec<String>>` | [LC 131](https://leetcode.com/problems/palindrome-partitioning/) |

<!-- PROBLEM_TABLE:END -->
