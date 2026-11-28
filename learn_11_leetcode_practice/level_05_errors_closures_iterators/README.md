# Level 5: Errors, Closures & Iterators (problems 0401-0500)

> Focus: `Result`, the `?` operator, custom errors, closures, iterator adapters, `impl Iterator`.

This level is about writing Rust that fails gracefully and reads like a description of the work.
You will replace `match` ladders with `Option`/`Result` combinators and the `?` operator, design
error types that callers can inspect, pass and return closures, and express loops as iterator
chains, up to writing your own iterators. The level ends with 36 LeetCode problems
(Easy to Hard) solved in that iterator-and-`Result` style.

## Prerequisites

- Levels 1-4: functions and control flow, ownership and borrowing, `String` vs `&str`, structs,
  enums and `match`, and the standard collections (`Vec`, `HashMap`, `BTreeMap`).
- Reading a generic signature such as `fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32`. Writing
  your own generic types and traits is the topic of level 6; here you only *use* the closure and
  iterator traits.

## What you will practice

- `Option` combinators: `map`, `and_then`, `filter`, `ok_or` / `ok_or_else`, `unwrap_or`,
  `unwrap_or_default`, `zip`, `or` / `xor`, `transpose`
  ([std `Option`](https://doc.rust-lang.org/std/option/enum.Option.html)).
- `Result`, `str::parse`, `ParseIntError` and `IntErrorKind`, `map_err`, and the `?` operator on
  both `Option` and `Result`
  ([The Rust Book 9.2](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html),
  [std `Result`](https://doc.rust-lang.org/std/result/enum.Result.html)).
- Choosing between `panic!` and `Result`, and validated types
  ([The Rust Book 9.3](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html)).
- Custom error enums with `Display` and `std::error::Error`, `From` conversions for `?`, `FromStr`,
  `Box<dyn Error>` with downcasting, and `source()` chains
  ([std `Error`](https://doc.rust-lang.org/std/error/trait.Error.html),
  [Rust By Example: Error handling](https://doc.rust-lang.org/rust-by-example/error.html)).
- Collecting iterators of `Result`/`Option` into `Result<Vec<_>, _>` / `Option<Vec<_>>`, and
  `try_fold` for overflow-safe arithmetic.
- Closures: capture modes, `Fn` / `FnMut` / `FnOnce`, `move`, generic `F: Fn` parameters, returning
  `impl Fn` (including edition-2024 `use<>` precise capturing) and `Box<dyn Fn>`, closures stored in
  structs (memoization, lazy values, event listeners, builders)
  ([The Rust Book 13.1](https://doc.rust-lang.org/book/ch13-01-closures.html)).
- Iterator adapters and consumers: `map`, `filter`, `filter_map`, `flat_map`, `flatten`,
  `enumerate`, `zip`, `chain`, `skip`, `take`, `skip_while`, `take_while`, `map_while`, `step_by`,
  `rev`, `peekable`, `scan`, `fold`, `try_fold`, `sum`, `product`, `min_by_key`, `max_by`,
  `position`, `any`, `all`, `find`, `find_map`, `count`, `last`, `nth`, `partition`, `unzip`
  ([The Rust Book 13.2](https://doc.rust-lang.org/book/ch13-02-iterators.html),
  [std `Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html)).
- Writing iterators: `impl Iterator`, `size_hint` / `ExactSizeIterator`, `DoubleEndedIterator`,
  `IntoIterator` and `FromIterator` for your own collection, `iter::successors` / `iter::from_fn`,
  returning `impl Iterator`, and laziness
  ([std `iter` module](https://doc.rust-lang.org/std/iter/index.html)).

## Roadmap

| Problems | Theme |
| -------- | ----- |
| 0401-0410 | `Option` and its combinators, `?` on `Option` |
| 0411-0422 | `Result`, `str::parse`, the `?` operator, collecting results, `panic!` vs `Result`, small parsers |
| 0423-0434 | Custom error types: `Display` + `Error`, `From`, `FromStr`, `Box<dyn Error>`, `source` chains, CSV/INI/command parsers |
| 0435-0450 | Closures: `Fn` / `FnMut` / `FnOnce`, `move`, returning and boxing closures, composition, memoization, retry, builders |
| 0451-0475 | Iterator adapters and consumers, with many Easy LeetCode problems |
| 0476-0486 | Writing your own iterators: `Iterator`, `DoubleEndedIterator`, `IntoIterator`, laziness, LeetCode design iterators |
| 0487-0500 | Medium and Hard LeetCode problems in iterator and `Result` style (Kadane, prefix scans, parsers, calculators) |

## How to work on a problem

```bash
cd learn_11_leetcode_practice/level_05_errors_closures_iterators

cargo test practice::p0401      # run the tests for the problem you are solving
cargo test solutions::p0401     # run the tests against the reference solution
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
| 0401 | [Safe Division with Option](exercises/p0401_safe_division.md) | Easy | `Option<T>`, `checked_div`, `unwrap_or`, `unwrap_or_default`, `match` on `Option` | Original |
| 0402 | [First Even Number, Doubled](exercises/p0402_first_even_doubled.md) | Easy | `Iterator::find`, `Option::map`, `Option::filter`, `Option::map_or` | Original |
| 0403 | [Maximum Value of a String in an Array](exercises/p0403_maximum_value_of_a_string_in_an_array.md) | Easy | `str::parse`, `Result::unwrap_or`, `Iterator::max`, `Option::unwrap_or` | [LC 2496](https://leetcode.com/problems/maximum-value-of-a-string-in-an-array/) |
| 0404 | [Initials with `?` on Option](exercises/p0404_initials_with_question_mark.md) | Easy | the `?` operator on `Option`, `split_whitespace`, `split_once`, `Result::ok` | Original |
| 0405 | [Follow a Lookup Chain with and_then](exercises/p0405_lookup_chain_and_then.md) | Easy | `Option::and_then`, `Option::map`, `Iterator::find`, `Option<&str>` | Original |
| 0406 | [Required Form Fields with ok_or](exercises/p0406_required_fields_ok_or.md) | Easy | `Option::ok_or`, `Option::ok_or_else`, `Option::filter`, `?` on `Result`, `map_err` | Original |
| 0407 | [Combine Optional Values with zip and or](exercises/p0407_combine_optional_values.md) | Easy | `Option::zip`, `Option::or`, `Option::or_else`, `Option::xor`, `Option::map` | Original |
| 0408 | [Settings with Defaults](exercises/p0408_settings_with_defaults.md) | Easy | `unwrap_or`, `unwrap_or_default`, `unwrap_or_else`, `map_or_else`, `as_deref`, `#[derive(Default)]` | Original |
| 0409 | [Check If a Word Occurs As a Prefix of Any Word in a Sentence](exercises/p0409_check_if_a_word_occurs_as_a_prefix_of_any_word_in_a_sentence.md) | Easy | `split`, `Iterator::position`, `str::starts_with`, `Option::map_or` | [LC 1455](https://leetcode.com/problems/check-if-a-word-occurs-as-a-prefix-of-any-word-in-a-sentence/) |
| 0410 | [Parse an Optional Number with transpose](exercises/p0410_transpose_optional_number.md) | Easy | `Option::transpose`, `Option<Result<T, E>>` vs `Result<Option<T>, E>`, `ParseIntError`, `?` | Original |
| 0411 | [Classify Parse Failures](exercises/p0411_classify_parse_failures.md) | Easy | `str::parse`, `ParseIntError`, `ParseIntError::kind`, `IntErrorKind`, `NonZeroU32`, `match` on `Result` | Original |
| 0412 | [Check if Numbers Are Ascending in a Sentence](exercises/p0412_check_if_numbers_are_ascending_in_a_sentence.md) | Easy | `split`, `filter_map`, `Result::ok`, `windows`, `Iterator::all` | [LC 2042](https://leetcode.com/problems/check-if-numbers-are-ascending-in-a-sentence/) |
| 0413 | [Add Two Number Strings with `?`](exercises/p0413_add_number_strings.md) | Easy | the `?` operator on `Result`, `Result::map`, `ParseIntError`, `i64::from` | Original |
| 0414 | [Validate an Age with map_err and and_then](exercises/p0414_validate_age_map_err.md) | Easy | `Result::map_err`, `Result::and_then`, `Result::map`, `Result::or_else`, `String` errors | Original |
| 0415 | [Parse a key=value Pair](exercises/p0415_parse_key_value_pair.md) | Easy | `split_once`, `Option::ok_or_else`, `trim`, `?` in a loop, `str::lines` | Original |
| 0416 | [All Numbers or the First Error](exercises/p0416_collect_results_into_vec.md) | Easy | `collect::<Result<Vec<_>, _>>()`, `Sum` for `Result`, `Iterator::position`, `enumerate`, `map_err` | Original |
| 0417 | [All Digits or Nothing](exercises/p0417_collect_options_digits.md) | Easy | `collect::<Option<Vec<_>>>()`, `Sum` for `Option`, `char::to_digit`, `bool::then`, `?` on `Option` | Original |
| 0418 | [Split Good and Bad Lines](exercises/p0418_split_good_and_bad_lines.md) | Easy | `filter_map(Result::ok)`, `Result::err`, `Iterator::partition`, `Result::is_ok` | Original |
| 0419 | [Panic or Return an Error?](exercises/p0419_panic_or_result.md) | Easy | `panic!`, `Result` for expected failures, validated newtype, `strip_suffix`, `#[should_panic(expected = ...)]` | Original |
| 0420 | [Overflow-Safe Sum and Product with try_fold](exercises/p0420_overflow_safe_sum_try_fold.md) | Easy | `Iterator::try_fold`, `checked_add`, `checked_mul`, `?` inside closures, `Option::filter`, `ok_or` | Original |
| 0421 | [Parse a Duration like 1h30m](exercises/p0421_parse_duration.md) | Medium | character-by-character parsing, `char::to_digit`, `checked_mul` / `checked_add`, `Option::take`, `?` converting `&str` into `String` | Original |
| 0422 | [Parse and Compare Semantic Versions](exercises/p0422_parse_semantic_version.md) | Medium | `split`, `let ... else`, `?`, collecting into `Result<Vec<_>, _>`, derived `Ord`, `Iterator::max` | Original |
| 0423 | [A Custom Error Enum with Display](exercises/p0423_temperature_error_enum.md) | Easy | error enums, `impl fmt::Display`, `impl std::error::Error`, `Box<dyn Error>`, `f64` parsing | Original |
| 0424 | [Automatic Error Conversion with From](exercises/p0424_from_parse_int_error.md) | Easy | `impl From<ParseIntError>`, `?` calls `From::from`, struct-like enum variants, `Error::source` | Original |
| 0425 | [Parse a Hex Color](exercises/p0425_parse_hex_color.md) | Easy | `u8::from_str_radix`, `strip_prefix`, `chunks`, custom error enum, `impl Display`, `?` | Original |
| 0426 | [Implement FromStr for a Point](exercises/p0426_implement_from_str_point.md) | Easy | `std::str::FromStr`, associated type `Err`, `str::parse::<Point>()`, `From`, `Display` round trip, `windows` | Original |
| 0427 | [Mixed Errors with Box<dyn Error>](exercises/p0427_box_dyn_error.md) | Medium | `Box<dyn Error>`, `?` boxing any error, `From<&str>` / `From<String>` for `Box<dyn Error>`, `downcast_ref`, `Error::is` | Original |
| 0428 | [Walk an Error's source Chain](exercises/p0428_error_source_chain.md) | Medium | layered errors, `Error::source`, `&(dyn Error + 'static)`, `iter::successors`, `enumerate` + `map_err` | Original |
| 0429 | [Parse a CSV Record into a Struct](exercises/p0429_parse_csv_record.md) | Medium | error enum with data, slice patterns with `let ... else`, generic helper over `FromStr`, `lines().enumerate().skip(1)`, collecting `Result`s | Original |
| 0430 | [Bank Transfers with Typed Errors](exercises/p0430_bank_transfer_errors.md) | Medium | `Result<T, BankError>` from `&mut self` methods, error variants with data, validate-then-mutate, `iter().position` | Original |
| 0431 | [Add Context to Errors](exercises/p0431_error_context.md) | Medium | wrapper error with `Box<dyn Error>` source, generic `E: Error + 'static`, `map_err` with a capturing closure, lazy messages with `FnOnce` | Original |
| 0432 | [Collect Every Validation Error](exercises/p0432_collect_all_validation_errors.md) | Medium | `Result<(), Vec<E>>`, checks as `(bool, error)` pairs, `filter_map` + `bool::then_some`, `Result::err`, `impl Display` | Original |
| 0433 | [Parse Robot Commands](exercises/p0433_robot_command_parser.md) | Medium | `FromStr` for an enum, `split_whitespace` + `next`, `enumerate` + `map_err` with line numbers, collecting `Result`s, `fold` as an interpreter | Original |
| 0434 | [Parse an INI-style Config](exercises/p0434_parse_ini_config.md) | Medium | line-numbered error enum, `lines().enumerate()`, parser state, `BTreeMap::entry`, `?` on `Option` for lookups | Original |
| 0435 | [Apply a Closure n Times](exercises/p0435_apply_closure_n_times.md) | Easy | closure syntax, generic parameter `F: Fn(..) -> ..`, `impl Fn` in argument position, closures capturing variables, passing functions as values | Original |
| 0436 | [Count Calls with FnMut](exercises/p0436_count_calls_fn_mut.md) | Easy | `FnMut`, closures capturing by mutable reference, `mut f: F`, returning `impl FnMut`, `move` | Original |
| 0437 | [Run a Task Exactly Once with FnOnce](exercises/p0437_run_once_fn_once.md) | Easy | `FnOnce`, closures that move captured values out, `Box<dyn FnOnce>`, `Option::take`, returning `impl FnOnce` | Original |
| 0438 | [Make Adders and Multipliers with move](exercises/p0438_make_adder_move.md) | Easy | returning closures with `impl Fn`, the `move` keyword, closures owning a `String`, precise capturing `use<>` | Original |
| 0439 | [A Table of Boxed Operations](exercises/p0439_boxed_operation_table.md) | Easy | `Box<dyn Fn>`, choosing a closure at run time, `Vec<Box<dyn Fn>>`, `try_fold` with `?`, calling boxed closures | Original |
| 0440 | [Sort Records with Closures](exercises/p0440_sort_records_with_closures.md) | Easy | `sort_by_key`, `sort_by`, `Ordering::then_with`, `cmp::Reverse`, `f64::total_cmp`, stable sorting | Original |
| 0441 | [Matrix Cells in Distance Order](exercises/p0441_matrix_cells_in_distance_order.md) | Easy | nested `flat_map` with an inner `move` closure, `sort_by_key` with a capturing closure, Manhattan distance | [LC 1030](https://leetcode.com/problems/matrix-cells-in-distance-order/) |
| 0442 | [A Running Average Closure](exercises/p0442_running_average_closure.md) | Easy | returning `impl FnMut`, state owned by a closure (`move`), `HashSet` inside a closure, passing an `FnMut` to `map` and `filter` | Original |
| 0443 | [Compose Functions](exercises/p0443_compose_functions.md) | Medium | generic closures `impl Fn(A) -> B`, returning a closure that captures closures, `fold` over `Vec<Box<dyn Fn>>`, collecting into `Option<Vec<_>>` | Original |
| 0444 | [Combine Predicates](exercises/p0444_predicate_combinators.md) | Medium | closures that take and return closures, `impl Fn(&T) -> bool`, `Box<dyn Fn(&T) -> bool>`, `Iterator::all`, generic `filter` | Original |
| 0445 | [Memoize an Expensive Function](exercises/p0445_memoize_closure.md) | Medium | storing a closure in a generic struct, calling a closure field `(self.func)(n)`, `HashMap::entry` + `or_insert_with`, disjoint field capture, returning a memoizing `impl FnMut` | Original |
| 0446 | [Retry an Operation](exercises/p0446_retry_with_closure.md) | Medium | generic `F: FnMut(u32) -> Result<T, E>`, retry loops, collecting failures, predicate closures `P: Fn(&E) -> bool`, `assert!` for preconditions | Original |
| 0447 | [Event Listeners with Boxed FnMut](exercises/p0447_event_listeners.md) | Medium | `Vec<Box<dyn FnMut>>`, `impl FnMut(..) + 'static` parameters, `iter_mut`, `filter_map` with stateful closures, `#[derive(Default)]` | Original |
| 0448 | [Lazy Initialization with FnOnce](exercises/p0448_lazy_value.md) | Medium | struct generic over `F: FnOnce() -> T`, `Option::take`, `Option::get_or_insert_with`, disjoint field capture, consuming `self` | Original |
| 0449 | [A Text Transformation Pipeline](exercises/p0449_text_pipeline_builder.md) | Medium | builder pattern taking `self` by value, `Vec<(String, Box<dyn Fn(String) -> String>)>`, closures owning their configuration, `fold` to run the steps | Original |
| 0450 | [Sort the Jumbled Numbers](exercises/p0450_sort_the_jumbled_numbers.md) | Medium | `sort_by_cached_key` with a closure capturing `mapping`, stable sorting, digits with `to_string().bytes()`, `fold` | [LC 2191](https://leetcode.com/problems/sort-the-jumbled-numbers/) |
| 0451 | [Sum, Product and Count](exercises/p0451_sum_product_count.md) | Easy | `Iterator::sum`, `Iterator::product`, `Iterator::count`, `map`, `filter`, `filter_map`, type annotations for `sum` | Original |
| 0452 | [XOR Operation in an Array](exercises/p0452_xor_operation_in_an_array.md) | Easy | ranges as iterators, `map`, `fold` with `^`, `reduce` | [LC 1486](https://leetcode.com/problems/xor-operation-in-an-array/) |
| 0453 | [Check If Two String Arrays are Equivalent](exercises/p0453_check_if_two_string_arrays_are_equivalent.md) | Easy | `flat_map` over `chars`, `Iterator::eq`, lazy comparison without allocation | [LC 1662](https://leetcode.com/problems/check-if-two-string-arrays-are-equivalent/) |
| 0454 | [Check if a String Is an Acronym of Words](exercises/p0454_check_if_a_string_is_an_acronym_of_words.md) | Easy | `filter_map` with `chars().next()`, `Iterator::eq`, comparing an iterator with a string's characters | [LC 2828](https://leetcode.com/problems/check-if-a-string-is-an-acronym-of-words/) |
| 0455 | [Split Strings by Separator](exercises/p0455_split_strings_by_separator.md) | Easy | `flat_map` with `str::split`, `filter`, `map(String::from)`, collecting into `Vec<String>` | [LC 2788](https://leetcode.com/problems/split-strings-by-separator/) |
| 0456 | [Count Asterisks](exercises/p0456_count_asterisks.md) | Easy | `str::split`, `Iterator::step_by`, `str::matches`, `map` + `sum` | [LC 2315](https://leetcode.com/problems/count-asterisks/) |
| 0457 | [Indexes Where Things Happen](exercises/p0457_indexes_with_enumerate.md) | Easy | `enumerate`, `position`, `rposition`, `max_by_key` tie-breaking, `rev`, `filter_map` | Original |
| 0458 | [Find the Peaks](exercises/p0458_find_the_peaks.md) | Easy | `slice::windows(3)`, `enumerate`, `filter_map`, index arithmetic on windows | [LC 2951](https://leetcode.com/problems/find-the-peaks/) |
| 0459 | [Occurrences After Bigram](exercises/p0459_occurrences_after_bigram.md) | Easy | `split` into `Vec<&str>`, `windows(3)` over words, `filter` + `map`, borrowing vs owning | [LC 1078](https://leetcode.com/problems/occurrences-after-bigram/) |
| 0460 | [Pagination with skip, take and chain](exercises/p0460_pagination_skip_take_chain.md) | Easy | `skip`, `take`, `chain`, `iter::once`, `enumerate` before `skip`, `usize::div_ceil`, `checked_*` arithmetic | Original |
| 0461 | [Read a Header Block with skip_while and take_while](exercises/p0461_header_block_skip_while.md) | Easy | `skip_while`, `take_while`, `map_while`, `skip`, `str::lines`, `split_once` | Original |
| 0462 | [Extremes with min_by_key and max_by](exercises/p0462_extremes_min_max_by.md) | Easy | `min_by_key`, `max_by_key`, `max_by` with `f64::total_cmp`, `cmp::Reverse`, `next_back` vs `last`, `nth`, tie-breaking rules | Original |
| 0463 | [Slowest Key](exercises/p0463_slowest_key.md) | Easy | `iter::once` + `chain` + `windows`, `zip` with `chars`, `max` on tuples for tie-breaking | [LC 1629](https://leetcode.com/problems/slowest-key/) |
| 0464 | [Questions About a List](exercises/p0464_any_all_find.md) | Easy | `any`, `all`, `find`, `find_map`, `count`, `is_sorted`, short-circuiting, vacuous truth | Original |
| 0465 | [Binary Prefix Divisible By 5](exercises/p0465_binary_prefix_divisible_by_5.md) | Easy | `Iterator::scan` with a running state, modular arithmetic, collecting `Vec<bool>` | [LC 1018](https://leetcode.com/problems/binary-prefix-divisible-by-5/) |
| 0466 | [Running Balance with scan](exercises/p0466_running_balance_scan.md) | Easy | `scan` for running totals, stopping `scan` early with `None`, `position`, `iter::once` + `chain` + `max` | Original |
| 0467 | [Best Time to Buy and Sell Stock](exercises/p0467_best_time_to_buy_and_sell_stock.md) | Easy | `fold` with a tuple state `(lowest price so far, best profit)`, one-pass greedy | [LC 121](https://leetcode.com/problems/best-time-to-buy-and-sell-stock/) |
| 0468 | [Partition and Unzip Exam Results](exercises/p0468_partition_and_unzip.md) | Easy | `Iterator::partition`, `Iterator::unzip`, `zip`, `into_iter` vs `iter`, tuple destructuring in closures | Original |
| 0469 | [Reformat The String](exercises/p0469_reformat_the_string.md) | Easy | `partition` on `chars`, `zip` with `chain(repeat(None))`, `flat_map` to interleave, `iter::once` | [LC 1417](https://leetcode.com/problems/reformat-the-string/) |
| 0470 | [Flatten Nested Data](exercises/p0470_flatten_nested_data.md) | Easy | `Iterator::flatten`, `flat_map`, `Option` and `Result` as iterators, `split_whitespace`, `filter_map` | Original |
| 0471 | [Cells in a Range on an Excel Sheet](exercises/p0471_cells_in_a_range_on_an_excel_sheet.md) | Easy | `char` ranges as iterators, nested `flat_map` with an inner `move` closure, `format!` | [LC 2194](https://leetcode.com/problems/cells-in-a-range-on-an-excel-sheet/) |
| 0472 | [Pascal's Triangle II](exercises/p0472_pascals_triangle_ii.md) | Easy | `iter::successors` to generate rows, `windows(2)`, `iter::once` + `chain`, `nth`, `checked_add` collected into `Option<Vec<_>>` | [LC 119](https://leetcode.com/problems/pascals-triangle-ii/) |
| 0473 | [Excel Sheet Column Title](exercises/p0473_excel_sheet_column_title.md) | Easy | `iter::successors` for digit extraction, bijective base 26, `rev`, collecting `char`s into a `String` | [LC 168](https://leetcode.com/problems/excel-sheet-column-title/) |
| 0474 | [License Key Formatting](exercises/p0474_license_key_formatting.md) | Easy | `filter` + `map` on `chars`, `slice::rchunks`, `rev`, `join` | [LC 482](https://leetcode.com/problems/license-key-formatting/) |
| 0475 | [Run-Length Encoding with peekable](exercises/p0475_run_length_encoding_peekable.md) | Medium | `Iterator::peekable`, `Peekable::next_if_eq`, `Peekable::next_if`, `while let`, decoding with errors | Original |
| 0476 | [A Counter Iterator](exercises/p0476_counter_iterator.md) | Easy | `impl Iterator for` a struct, associated type `Item`, `next(&mut self)`, getting every adapter for free | Original |
| 0477 | [A Fibonacci Iterator that Stops on Overflow](exercises/p0477_fibonacci_iterator.md) | Easy | custom `Iterator`, `checked_add` + `Option` state, `?` inside `next`, `take_while`, `last`, `count`, `nth` | Original |
| 0478 | [Sequences with successors and from_fn](exercises/p0478_sequences_successors_from_fn.md) | Easy | `iter::successors`, `iter::from_fn`, `bool::then`, `checked_mul`, `by_ref` | Original |
| 0479 | [Return impl Iterator](exercises/p0479_return_impl_iterator.md) | Medium | returning `impl Iterator<Item = T>`, lazy results, iterators that borrow their input, edition-2024 lifetime capture, infinite iterators with `take` | Original |
| 0480 | [Iterators Are Lazy](exercises/p0480_iterators_are_lazy.md) | Medium | laziness, short-circuiting consumers, counting closure calls with `FnMut`, `take`, `take_while` consuming one extra item, item-by-item processing | Original |
| 0481 | [A Sliding Window Iterator over a Slice](exercises/p0481_slice_windows_iterator.md) | Medium | iterator struct with a lifetime `Windows<'a, T>`, `type Item = &'a [T]`, `slice::get` with a range, `size_hint`, `ExactSizeIterator` | Original |
| 0482 | [IntoIterator for Your Own Collection](exercises/p0482_into_iterator_playlist.md) | Medium | `IntoIterator` for `T`, `&T` and `&mut T`, associated types `Item` / `IntoIter`, `for` loops over a custom type, `FromIterator` for `collect` | Original |
| 0483 | [A Double-Ended Step Range](exercises/p0483_double_ended_step_range.md) | Medium | `DoubleEndedIterator::next_back`, `rev` on a custom iterator, `ExactSizeIterator`, consuming from both ends, `iter::from_fn` | Original |
| 0484 | [Interleave Two Iterators](exercises/p0484_interleave_iterators.md) | Medium | iterator struct generic over `A: Iterator` and `B: Iterator<Item = A::Item>`, `Iterator::fuse` / `Fuse`, `IntoIterator` parameters, `size_hint` arithmetic | Original |
| 0485 | [RLE Iterator](exercises/p0485_rle_iterator.md) | Medium | LeetCode design struct, iterator-like state (`index`, `used`), consuming counts, `i64` to avoid overflow | [LC 900](https://leetcode.com/problems/rle-iterator/) |
| 0486 | [Iterator for Combination](exercises/p0486_iterator_for_combination.md) | Medium | LeetCode design struct, next lexicographic combination of indexes, `rposition` on a range, `Option<Vec<usize>>` as iterator state | [LC 1286](https://leetcode.com/problems/iterator-for-combination/) |
| 0487 | [Maximum Subarray](exercises/p0487_maximum_subarray.md) | Medium | Kadane's algorithm as a `scan` (best sum ending here) followed by `max` | [LC 53](https://leetcode.com/problems/maximum-subarray/) |
| 0488 | [Product of Array Except Self](exercises/p0488_product_of_array_except_self.md) | Medium | exclusive prefix products with `scan`, suffix products with `rev` + `scan`, `zip` to combine them | [LC 238](https://leetcode.com/problems/product-of-array-except-self/) |
| 0489 | [Rearrange Array Elements by Sign](exercises/p0489_rearrange_array_elements_by_sign.md) | Medium | stable `partition`, `zip` of the two halves, `flat_map` into arrays `[p, n]` | [LC 2149](https://leetcode.com/problems/rearrange-array-elements-by-sign/) |
| 0490 | [Number of Laser Beams in a Bank](exercises/p0490_number_of_laser_beams_in_a_bank.md) | Medium | `map` rows to counts, `filter` empty rows, `fold` with `(previous, total)` state | [LC 2125](https://leetcode.com/problems/number-of-laser-beams-in-a-bank/) |
| 0491 | [Simplify Path](exercises/p0491_simplify_path.md) | Medium | `split('/')`, `fold` into a `Vec` used as a stack, `match` on `&str`, `join` | [LC 71](https://leetcode.com/problems/simplify-path/) |
| 0492 | [Longest Absolute File Path](exercises/p0492_longest_absolute_file_path.md) | Medium | `split('\n')`, `trim_start_matches` to measure depth, `Vec::truncate` as a depth stack, `last().copied()` | [LC 388](https://leetcode.com/problems/longest-absolute-file-path/) |
| 0493 | [Letter Combinations of a Phone Number](exercises/p0493_letter_combinations_of_a_phone_number.md) | Medium | `fold` over the digits, `flat_map` to extend every prefix, inner `move` closure, a `const` lookup table | [LC 17](https://leetcode.com/problems/letter-combinations-of-a-phone-number/) |
| 0494 | [Restore IP Addresses](exercises/p0494_restore_ip_addresses.md) | Medium | nested `flat_map` over segment lengths, a validation closure, `filter`, `join`, arrays of slices | [LC 93](https://leetcode.com/problems/restore-ip-addresses/) |
| 0495 | [Different Ways to Add Parentheses](exercises/p0495_different_ways_to_add_parentheses.md) | Medium | divide and conquer with recursion, `char_indices` + `filter`, nested `flat_map` (Cartesian product), a `move` closure choosing the operator | [LC 241](https://leetcode.com/problems/different-ways-to-add-parentheses/) |
| 0496 | [Split Array into Fibonacci Sequence](exercises/p0496_split_array_into_fibonacci_sequence.md) | Medium | trying candidates with `find_map`, `checked_add` + `?` inside an `Option`-returning helper, parsing with overflow detection, `starts_with` | [LC 842](https://leetcode.com/problems/split-array-into-fibonacci-sequence/) |
| 0497 | [Decode String](exercises/p0497_decode_string.md) | Medium | recursive descent over `Peekable<Chars>`, `peek` / `next_if` / `next`, `str::repeat` | [LC 394](https://leetcode.com/problems/decode-string/) |
| 0498 | [Trapping Rain Water](exercises/p0498_trapping_rain_water.md) | Hard | running maximum with `scan` from both ends, `rev`, three-way `zip`, `map` + `sum` | [LC 42](https://leetcode.com/problems/trapping-rain-water/) |
| 0499 | [Basic Calculator](exercises/p0499_basic_calculator.md) | Hard | tokenising with `Peekable` and `next_if`, an explicit stack instead of recursion, unary minus, `i64` accumulation | [LC 224](https://leetcode.com/problems/basic-calculator/) |
| 0500 | [Number of Atoms](exercises/p0500_number_of_atoms.md) | Hard | parsing with `Peekable<Chars>` and `next_if`, a stack of `BTreeMap`s, `mem::take` / `mem::replace`, `entry().or_default()`, building the output with `map` + `collect::<String>()` | [LC 726](https://leetcode.com/problems/number-of-atoms/) |

<!-- PROBLEM_TABLE:END -->
