# Level 1: Basics (problems 0001-0100)

> Focus: variables, scalar types, operators, `if`/`match` on numbers, loops, functions, arrays.

This level covers the first three chapters of The Rust Book. You write small functions on numbers,
booleans, characters, tuples, arrays and vectors, and you get used to the compiler's rules about
types. Overflow is a recurring theme: Rust makes you decide what happens when a number does not fit.
68 problems come from LeetCode (66 Easy, plus 2 simple Medium ones at the very end) and use
LeetCode's exact Rust signatures. The other 32 are original problems, each built around one
language feature.

## Prerequisites

- Rust 1.87 or newer installed, and `cargo test` working (see
  [The Rust Book 1.1](https://doc.rust-lang.org/book/ch01-01-installation.html) and
  [1.3](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)).
- No earlier Rust knowledge is needed. Basic programming ideas (variables, loops, functions) are
  enough. Read chapters 1-3 of the Book before or alongside this level.

## What you will practice

- Variables, `mut`, shadowing and constants:
  [Book 3.1](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html).
- Integer and floating-point types, `bool`, `char`, tuples and arrays:
  [Book 3.2](https://doc.rust-lang.org/book/ch03-02-data-types.html).
- Integer overflow and how to handle it with `checked_*`, `wrapping_*` and `saturating_*` methods:
  [`i32` methods](https://doc.rust-lang.org/std/primitive.i32.html).
- Conversions with `as`, `From` and widening casts:
  [Rust Reference: type cast expressions](https://doc.rust-lang.org/reference/expressions/operator-expr.html#type-cast-expressions).
- Integer division `/` and remainder `%`, including negative operands, `div_euclid` and `rem_euclid`.
- Functions, parameters, return values, and the difference between statements and expressions:
  [Book 3.3](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html).
- `if` as an expression, `loop` with a `break` value, `while`, `for` over ranges with `rev()` and
  `step_by()`, and loop labels:
  [Book 3.5](https://doc.rust-lang.org/book/ch03-05-control-flow.html).
- `match` on integers, ranges and characters, `|` patterns and guards:
  [Book 6.2](https://doc.rust-lang.org/book/ch06-02-match.html) and
  [Book 19.3](https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html).
- Reading a `Vec<i32>` or a slice `&[i32]` by index and by reference:
  [Book 4.3](https://doc.rust-lang.org/book/ch04-03-slices.html).
- Classic number problems: digits, palindromes, GCD and LCM, primes, powers, Fibonacci.

## Roadmap

| Problems | Theme |
| -------- | ----- |
| 0001-0012 | Warm-up: functions, arithmetic, `/` and `%`, constants, shadowing |
| 0013-0022 | Booleans, comparisons, `if` expressions, `match` on integers and ranges |
| 0023-0033 | Integer types, overflow (`checked_*`, `saturating_*`, `wrapping_*`), `as` casts, floats |
| 0034-0038 | The `char` type: classification, letter arithmetic, digits in any base |
| 0039-0054 | Tuples, arrays `[T; N]`, vectors and slices, 2D grids |
| 0055-0072 | Loops: ranges with `rev`/`step_by`, `while`, simulations, `loop` with a value, labels |
| 0073-0087 | Digit manipulation: digit sums, reversing, palindromes, digit counters |
| 0088-0100 | Number theory: GCD, primes, powers, Fibonacci, binary search, and two Medium finales |

## How to work on a problem

```bash
cd learn_11_leetcode_practice/level_01_basics

cargo test practice::p0001      # run the tests for the problem you are solving
cargo test solutions::p0001     # run the tests against the reference solution
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
| 0001 | [Fizz Buzz](exercises/p0001_fizz_buzz.md) | Easy | `for` loops over ranges, `match` on tuples, `String`, `Vec` | [LC 412](https://leetcode.com/problems/fizz-buzz/) |
| 0002 | [Add Two Integers](exercises/p0002_add_two_integers.md) | Easy | functions, parameters, return values, expressions vs statements | [LC 2235](https://leetcode.com/problems/add-two-integers/) |
| 0003 | [Find the Maximum Achievable Number](exercises/p0003_find_the_maximum_achievable_number.md) | Easy | arithmetic expressions, integer multiplication, reasoning before coding | [LC 2769](https://leetcode.com/problems/find-the-maximum-achievable-number/) |
| 0004 | [Calculate Delayed Arrival Time](exercises/p0004_calculate_delayed_arrival_time.md) | Easy | the remainder operator `%`, clock arithmetic | [LC 2651](https://leetcode.com/problems/calculate-delayed-arrival-time/) |
| 0005 | [Smallest Even Multiple](exercises/p0005_smallest_even_multiple.md) | Easy | `if` as an expression, `%` to test divisibility | [LC 2413](https://leetcode.com/problems/smallest-even-multiple/) |
| 0006 | [Convert the Temperature](exercises/p0006_convert_the_temperature.md) | Easy | `f64` arithmetic, float literals, `vec!` | [LC 2469](https://leetcode.com/problems/convert-the-temperature/) |
| 0007 | [Rectangle Area and Perimeter](exercises/p0007_rectangle_area_and_perimeter.md) | Easy | tuples, returning several values, destructuring a tuple parameter | Original |
| 0008 | [Split Seconds into Units](exercises/p0008_split_seconds_into_units.md) | Easy | `const`, integer division `/` and remainder `%`, tuples | Original |
| 0009 | [Division with Negative Numbers](exercises/p0009_division_with_negative_numbers.md) | Easy | integer `/` and `%` with negative operands, `div_euclid`, `rem_euclid`, tuples | Original |
| 0010 | [Count Odd Numbers in an Interval Range](exercises/p0010_count_odd_numbers_in_an_interval_range.md) | Easy | integer division rounding, counting with a formula instead of a loop | [LC 1523](https://leetcode.com/problems/count-odd-numbers-in-an-interval-range/) |
| 0011 | [Normalize an Angle](exercises/p0011_normalize_an_angle.md) | Easy | `let`, `mut`, shadowing (including shadowing that changes the type), `rem_euclid`, `as` | Original |
| 0012 | [Minimum Cuts to Divide a Circle](exercises/p0012_minimum_cuts_to_divide_a_circle.md) | Easy | `if` / `else if` / `else` as an expression, `%`, case analysis | [LC 2481](https://leetcode.com/problems/minimum-cuts-to-divide-a-circle/) |
| 0013 | [Leap Year](exercises/p0013_leap_year.md) | Easy | `bool`, logical operators `&&`, `\|\|`, `!`, `%` | Original |
| 0014 | [Exactly One True](exercises/p0014_exactly_one_true.md) | Easy | `bool`, `&&`, `\|\|`, `!`, `^`, `bool as u8` | Original |
| 0015 | [Letter Grade](exercises/p0015_letter_grade.md) | Easy | `match` on integer ranges, `char`, `Option` | Original |
| 0016 | [Days in a Month](exercises/p0016_days_in_a_month.md) | Easy | `match` with `\|` patterns, `if` inside a `match` arm, `Option` | Original |
| 0017 | [Account Balance After Rounded Purchase](exercises/p0017_account_balance_after_rounded_purchase.md) | Easy | rounding with integer arithmetic, `/` and `*` | [LC 2806](https://leetcode.com/problems/account-balance-after-rounded-purchase/) |
| 0018 | [Nim Game](exercises/p0018_nim_game.md) | Easy | `%`, `bool` results, finding a pattern | [LC 292](https://leetcode.com/problems/nim-game/) |
| 0019 | [Divisor Game](exercises/p0019_divisor_game.md) | Easy | parity, `bool` results, reasoning about games | [LC 1025](https://leetcode.com/problems/divisor-game/) |
| 0020 | [K Items With the Maximum Sum](exercises/p0020_k_items_with_the_maximum_sum.md) | Easy | `if` / `else if` chains as expressions, greedy reasoning | [LC 2600](https://leetcode.com/problems/k-items-with-the-maximum-sum/) |
| 0021 | [Type of Triangle](exercises/p0021_type_of_triangle.md) | Easy | comparison and logical operators, indexing a `Vec`, `String::from` | [LC 3024](https://leetcode.com/problems/type-of-triangle/) |
| 0022 | [Categorize Box According to Criteria](exercises/p0022_categorize_box_according_to_criteria.md) | Easy | integer overflow, widening `i32` to `i64`, `bool` variables, `match` on a tuple of `bool`s | [LC 2525](https://leetcode.com/problems/categorize-box-according-to-criteria/) |
| 0023 | [Smallest Integer Type](exercises/p0023_smallest_integer_type.md) | Easy | integer types and their ranges, `i8::MIN`/`i8::MAX`, `const` in range patterns, `&'static str` | Original |
| 0024 | [Integer Casts with `as`](exercises/p0024_integer_casts_with_as.md) | Easy | `as` casts between integer types, truncation, two's complement, bit shifts `>>` and `<<` | Original |
| 0025 | [Checked Sum](exercises/p0025_checked_sum.md) | Easy | integer overflow, `checked_add`, `Option`, widening to `i64`, iterating a slice by reference | Original |
| 0026 | [Saturating Volume Knob](exercises/p0026_saturating_volume_knob.md) | Easy | `u8`, `saturating_add`, `saturating_sub`, `checked_add_signed`, `unsigned_abs` | Original |
| 0027 | [Wrapping FNV-1a Hash](exercises/p0027_wrapping_fnv_hash.md) | Easy | `wrapping_mul`, `u32`, XOR `^`, `u8` to `u32` conversion, byte string literals | Original |
| 0028 | [Checked Factorial](exercises/p0028_checked_factorial.md) | Easy | `u64`, `checked_mul`, the `?` operator on `Option`, `for` over `1..=n` | Original |
| 0029 | [Overflow-Safe Midpoint](exercises/p0029_overflow_safe_midpoint.md) | Easy | overflow in intermediate results, widening to `i64`, `div_euclid`, `as` casts, `Option` | Original |
| 0030 | [Float Rounding Modes](exercises/p0030_float_rounding_modes.md) | Easy | `f64::floor`, `ceil`, `round`, `trunc`, float-to-integer `as` casts (saturating, NaN), tuples | Original |
| 0031 | [Approximately Equal Floats](exercises/p0031_approximately_equal_floats.md) | Easy | floating-point precision, `f64::abs`, `f64::max`, NaN and infinity, `\|\|` | Original |
| 0032 | [Average Salary Excluding the Minimum and Maximum Salary](exercises/p0032_average_salary_excluding_the_minimum_and_maximum_salary.md) | Easy | `as f64` casts, `min`/`max` tracking, iterating a `Vec` by reference | [LC 1491](https://leetcode.com/problems/average-salary-excluding-the-minimum-and-maximum-salary/) |
| 0033 | [Sign of the Product of an Array](exercises/p0033_sign_of_the_product_of_an_array.md) | Easy | avoiding overflow by reasoning, `match` with guards, early `return` | [LC 1822](https://leetcode.com/problems/sign-of-the-product-of-an-array/) |
| 0034 | [Classify a Character](exercises/p0034_classify_a_character.md) | Easy | the `char` type, `match` on `char` ranges, `\|` patterns, `&'static str` | Original |
| 0035 | [Alphabet Position](exercises/p0035_alphabet_position.md) | Easy | `char as u8`, `u8 as char`, byte literals `b'a'`, `Option`, `match` on ranges | Original |
| 0036 | [Caesar Shift](exercises/p0036_caesar_shift.md) | Easy | `char` arithmetic through `u8`, `rem_euclid`, avoiding overflow, `&[char]` to `Vec<char>` | Original |
| 0037 | [Toggle Letter Case](exercises/p0037_toggle_letter_case.md) | Easy | `char` methods (`is_ascii_lowercase`, `to_ascii_uppercase`, ...), slices `&[char]`, tuples | Original |
| 0038 | [Digits in Any Base](exercises/p0038_digits_in_any_base.md) | Easy | `char::from_digit`, `char::to_digit`, `while` loops, `checked_mul`/`checked_add`, `?` on `Option` | Original |
| 0039 | [Min and Max of a Slice](exercises/p0039_min_and_max_of_a_slice.md) | Easy | slices `&[i32]`, reading by index and by reference, tuples inside `Option`, `abs_diff` | Original |
| 0040 | [Running Sum of 1d Array](exercises/p0040_running_sum_of_1d_array.md) | Easy | indexing a `Vec`, updating elements in place, shadowing a parameter with `let mut` | [LC 1480](https://leetcode.com/problems/running-sum-of-1d-array/) |
| 0041 | [Concatenation of Array](exercises/p0041_concatenation_of_array.md) | Easy | building a `Vec`, `Vec::with_capacity`, `extend_from_slice`, borrowing a `Vec` as a slice | [LC 1929](https://leetcode.com/problems/concatenation-of-array/) |
| 0042 | [Build Array from Permutation](exercises/p0042_build_array_from_permutation.md) | Easy | using values as indices, `as usize`, iterating a `Vec` by reference, `push` | [LC 1920](https://leetcode.com/problems/build-array-from-permutation/) |
| 0043 | [Shuffle the Array](exercises/p0043_shuffle_the_array.md) | Easy | index arithmetic, `i32` to `usize`, `for` over `0..n`, `push` | [LC 1470](https://leetcode.com/problems/shuffle-the-array/) |
| 0044 | [Find the Highest Altitude](exercises/p0044_find_the_highest_altitude.md) | Easy | running totals with `mut`, `max`, iterating a `Vec` by reference | [LC 1732](https://leetcode.com/problems/find-the-highest-altitude/) |
| 0045 | [Kids With the Greatest Number of Candies](exercises/p0045_kids_with_the_greatest_number_of_candies.md) | Easy | two passes over a `Vec`, `Vec<bool>`, comparison operators | [LC 1431](https://leetcode.com/problems/kids-with-the-greatest-number-of-candies/) |
| 0046 | [Richest Customer Wealth](exercises/p0046_richest_customer_wealth.md) | Easy | `Vec<Vec<i32>>`, nested `for` loops over references, accumulators | [LC 1672](https://leetcode.com/problems/richest-customer-wealth/) |
| 0047 | [Maximum Product of Two Elements in an Array](exercises/p0047_maximum_product_of_two_elements_in_an_array.md) | Easy | tracking the two largest values in one pass, `if` / `else if` | [LC 1464](https://leetcode.com/problems/maximum-product-of-two-elements-in-an-array/) |
| 0048 | [Max Consecutive Ones](exercises/p0048_max_consecutive_ones.md) | Easy | counters that reset, `if` / `else`, `max` | [LC 485](https://leetcode.com/problems/max-consecutive-ones/) |
| 0049 | [Three Consecutive Odds](exercises/p0049_three_consecutive_odds.md) | Easy | index windows `arr[i - 2]`, `arr[i - 1]`, `arr[i]`, ranges that avoid underflow, `&&` | [LC 1550](https://leetcode.com/problems/three-consecutive-odds/) |
| 0050 | [Matrix Diagonal Sum](exercises/p0050_matrix_diagonal_sum.md) | Easy | 2D indexing, `enumerate`, the center cell of an odd-sized square | [LC 1572](https://leetcode.com/problems/matrix-diagonal-sum/) |
| 0051 | [Number of Good Pairs](exercises/p0051_number_of_good_pairs.md) | Easy | fixed-size array as a counter `[i32; 101]`, nested index loops, `as usize` | [LC 1512](https://leetcode.com/problems/number-of-good-pairs/) |
| 0052 | [Left and Right Sum Differences](exercises/p0052_left_and_right_sum_differences.md) | Easy | prefix sums, total minus prefix, `abs`, building a `Vec` | [LC 2574](https://leetcode.com/problems/left-and-right-sum-differences/) |
| 0053 | [Transpose a 3x3 Matrix](exercises/p0053_transpose_a_3x3_matrix.md) | Easy | arrays `[[i32; 3]; 3]`, arrays are `Copy`, passing arrays by value and by reference, nested loops | Original |
| 0054 | [Tic-Tac-Toe Winner](exercises/p0054_tic_tac_toe_winner.md) | Easy | `[[char; 3]; 3]`, a `const` array of tuples, destructuring tuples in a `for` loop, `Option<char>` | Original |
| 0055 | [Countdown by Step](exercises/p0055_countdown_by_step.md) | Easy | `for` over ranges, `rev()`, `step_by()`, inclusive ranges `..=`, `div_ceil` | Original |
| 0056 | [Sum Multiples](exercises/p0056_sum_multiples.md) | Easy | `for` over `1..=n`, `%`, `\|\|`, accumulators | [LC 2652](https://leetcode.com/problems/sum-multiples/) |
| 0057 | [Divisible and Non-divisible Sums Difference](exercises/p0057_divisible_and_non_divisible_sums_difference.md) | Easy | `for` loops with `if` / `else`, two accumulators, integer formulas | [LC 2894](https://leetcode.com/problems/divisible-and-non-divisible-sums-difference/) |
| 0058 | [Number of Steps to Reduce a Number to Zero](exercises/p0058_number_of_steps_to_reduce_a_number_to_zero.md) | Easy | `while` loops, `if` / `else`, compound assignment `/=` and `-=` | [LC 1342](https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/) |
| 0059 | [Count of Matches in Tournament](exercises/p0059_count_of_matches_in_tournament.md) | Easy | `while` simulation, integer division, odd/even cases | [LC 1688](https://leetcode.com/problems/count-of-matches-in-tournament/) |
| 0060 | [Water Bottles](exercises/p0060_water_bottles.md) | Easy | `while` simulation, `/` and `%` together, several `mut` state variables | [LC 1518](https://leetcode.com/problems/water-bottles/) |
| 0061 | [Total Distance Traveled](exercises/p0061_total_distance_traveled.md) | Easy | `while` simulation with a compound condition `&&`, several `mut` variables | [LC 2739](https://leetcode.com/problems/total-distance-traveled/) |
| 0062 | [Pass the Pillow](exercises/p0062_pass_the_pillow.md) | Easy | `%` on a cycle, `if` as an expression, checking a formula against a simulation | [LC 2582](https://leetcode.com/problems/pass-the-pillow/) |
| 0063 | [Calculate Money in Leetcode Bank](exercises/p0063_calculate_money_in_leetcode_bank.md) | Easy | `for` over a range, `/` and `%` to split days into weeks and weekdays | [LC 1716](https://leetcode.com/problems/calculate-money-in-leetcode-bank/) |
| 0064 | [Distribute Candies to People](exercises/p0064_distribute_candies_to_people.md) | Easy | `loop` with `break`, `min`, indexing with `%`, `vec![0; n]` | [LC 1103](https://leetcode.com/problems/distribute-candies-to-people/) |
| 0065 | [Next Power of Two](exercises/p0065_next_power_of_two.md) | Easy | `loop` with a `break` value, `checked_mul`, `Option`, halving with `/=` | Original |
| 0066 | [Collatz Steps](exercises/p0066_collatz_steps.md) | Easy | `loop` with a `break` value, `checked_mul`/`checked_add` with `?`, `u64`, tuples | Original |
| 0067 | [Find the Pivot Integer](exercises/p0067_find_the_pivot_integer.md) | Easy | `for` with an early `return`, running sums, sum formula `n * (n + 1) / 2` | [LC 2485](https://leetcode.com/problems/find-the-pivot-integer/) |
| 0068 | [Maximum Height of a Triangle](exercises/p0068_maximum_height_of_a_triangle.md) | Easy | helper functions, simulation with `loop`, tuples for alternating state, `max` | [LC 3200](https://leetcode.com/problems/maximum-height-of-a-triangle/) |
| 0069 | [Distribute Candies Among Children I](exercises/p0069_distribute_candies_among_children_i.md) | Easy | nested `for` loops, deriving the last variable instead of looping over it, ranges | [LC 2928](https://leetcode.com/problems/distribute-candies-among-children-i/) |
| 0070 | [Count Square Sum Triples](exercises/p0070_count_square_sum_triples.md) | Easy | nested `for` loops, float square root with `as` casts, verifying a float result with integers | [LC 1925](https://leetcode.com/problems/count-square-sum-triples/) |
| 0071 | [Find a Value in a Grid](exercises/p0071_find_a_value_in_a_grid.md) | Easy | loop labels, `break 'outer`, `continue 'outer`, `[[i32; 4]; 3]`, `enumerate` | Original |
| 0072 | [Pythagorean Triplet with a Given Sum](exercises/p0072_pythagorean_triplet_with_a_given_sum.md) | Easy | labeled blocks with `break 'label value`, nested `for` loops, choosing loop bounds, tuples | Original |
| 0073 | [Subtract the Product and Sum of Digits of an Integer](exercises/p0073_subtract_the_product_and_sum_of_digits_of_an_integer.md) | Easy | extracting digits with `% 10` and `/ 10`, `while` loops, two accumulators | [LC 1281](https://leetcode.com/problems/subtract-the-product-and-sum-of-digits-of-an-integer/) |
| 0074 | [Count the Digits That Divide a Number](exercises/p0074_count_the_digits_that_divide_a_number.md) | Easy | digit loop on a **copy** of the input, `%` for divisibility, counting | [LC 2520](https://leetcode.com/problems/count-the-digits-that-divide-a-number/) |
| 0075 | [Alternating Digit Sum](exercises/p0075_alternating_digit_sum.md) | Easy | digit loop, alternating signs, fixing the sign at the end | [LC 2544](https://leetcode.com/problems/alternating-digit-sum/) |
| 0076 | [Add Digits](exercises/p0076_add_digits.md) | Easy | nested `while` loops, the digital root, `%` by 9 | [LC 258](https://leetcode.com/problems/add-digits/) |
| 0077 | [Harshad Number](exercises/p0077_harshad_number.md) | Easy | helper functions, digit sums, `if` as the return expression | [LC 3099](https://leetcode.com/problems/harshad-number/) |
| 0078 | [Find Numbers with Even Number of Digits](exercises/p0078_find_numbers_with_even_number_of_digits.md) | Easy | counting digits, nested functions, iterating a `Vec` by reference | [LC 1295](https://leetcode.com/problems/find-numbers-with-even-number-of-digits/) |
| 0079 | [Palindrome Number](exercises/p0079_palindrome_number.md) | Easy | reversing digits, avoiding overflow by reversing only half, early `return` | [LC 9](https://leetcode.com/problems/palindrome-number/) |
| 0080 | [A Number After a Double Reversal](exercises/p0080_a_number_after_a_double_reversal.md) | Easy | reasoning about trailing zeros, `\|\|`, checking a shortcut against a simulation | [LC 2119](https://leetcode.com/problems/a-number-after-a-double-reversal/) |
| 0081 | [Maximum 69 Number](exercises/p0081_maximum_69_number.md) | Easy | place values (`1, 10, 100, ...`), digit loop that remembers a position, greedy choice | [LC 1323](https://leetcode.com/problems/maximum-69-number/) |
| 0082 | [Self Dividing Numbers](exercises/p0082_self_dividing_numbers.md) | Easy | helper function returning `bool`, early `return false`, `for` over an inclusive range | [LC 728](https://leetcode.com/problems/self-dividing-numbers/) |
| 0083 | [Count Integers With Even Digit Sum](exercises/p0083_count_integers_with_even_digit_sum.md) | Easy | `for` over `1..=num`, a nested helper function, digit sums | [LC 2180](https://leetcode.com/problems/count-integers-with-even-digit-sum/) |
| 0084 | [Minimum Sum of Four Digit Number After Splitting Digits](exercises/p0084_minimum_sum_of_four_digit_number_after_splitting_digits.md) | Easy | fixed-size array `[i32; 4]`, sorting an array, place values, greedy reasoning | [LC 2160](https://leetcode.com/problems/minimum-sum-of-four-digit-number-after-splitting-digits/) |
| 0085 | [Separate the Digits in an Array](exercises/p0085_separate_the_digits_in_an_array.md) | Easy | collecting digits in reverse, `Vec::reverse`, slices of a `Vec`, `extend_from_slice` | [LC 2553](https://leetcode.com/problems/separate-the-digits-in-an-array/) |
| 0086 | [Happy Number](exercises/p0086_happy_number.md) | Easy | digit squares, cycle detection with two pointers (Floyd), `loop` with `break` value | [LC 202](https://leetcode.com/problems/happy-number/) |
| 0087 | [Maximum Number of Balls in a Box](exercises/p0087_maximum_number_of_balls_in_a_box.md) | Easy | fixed-size array as a counter `[i32; 46]`, digit sums, `as usize`, finding a maximum | [LC 1742](https://leetcode.com/problems/maximum-number-of-balls-in-a-box/) |
| 0088 | [GCD and LCM](exercises/p0088_gcd_and_lcm.md) | Easy | Euclid's algorithm with `while`, tuple assignment `(a, b) = (b, a % b)`, `checked_mul`, `Option` | Original |
| 0089 | [Find Greatest Common Divisor of Array](exercises/p0089_find_greatest_common_divisor_of_array.md) | Easy | min and max in one pass, Euclid's algorithm in a helper function | [LC 1979](https://leetcode.com/problems/find-greatest-common-divisor-of-array/) |
| 0090 | [Number of Common Factors](exercises/p0090_number_of_common_factors.md) | Easy | `for` over `1..=min(a, b)`, `%` with `&&`, counting | [LC 2427](https://leetcode.com/problems/number-of-common-factors/) |
| 0091 | [Prime Factorization](exercises/p0091_prime_factorization.md) | Easy | trial division, `while` with an overflow-safe condition, `u64`, building a `Vec` | Original |
| 0092 | [Three Divisors](exercises/p0092_three_divisors.md) | Easy | counting divisors in pairs up to `sqrt(n)`, `while`, early exit | [LC 1952](https://leetcode.com/problems/three-divisors/) |
| 0093 | [Ugly Number](exercises/p0093_ugly_number.md) | Easy | `for` over an array literal, nested `while`, dividing out factors, early `return` | [LC 263](https://leetcode.com/problems/ugly-number/) |
| 0094 | [Power of Two](exercises/p0094_power_of_two.md) | Easy | bitwise `&`, `count_ones`, binary representation, guarding against non-positive input | [LC 231](https://leetcode.com/problems/power-of-two/) |
| 0095 | [Power of Three](exercises/p0095_power_of_three.md) | Easy | `while` with `%` and `/=`, guarding non-positive input, a divisibility trick | [LC 326](https://leetcode.com/problems/power-of-three/) |
| 0096 | [Fibonacci Number](exercises/p0096_fibonacci_number.md) | Easy | iteration instead of recursion, tuple assignment `(a, b) = (b, a + b)`, `for _ in` | [LC 509](https://leetcode.com/problems/fibonacci-number/) |
| 0097 | [Climbing Stairs](exercises/p0097_climbing_stairs.md) | Easy | recognizing a recurrence, iterative Fibonacci, tuple assignment | [LC 70](https://leetcode.com/problems/climbing-stairs/) |
| 0098 | [Sqrt(x)](exercises/p0098_sqrtx.md) | Easy | binary search with `while`, widening to `i64` to avoid overflow, loop invariants | [LC 69](https://leetcode.com/problems/sqrtx/) |
| 0099 | [Reverse Integer](exercises/p0099_reverse_integer.md) | Medium | `checked_mul` and `checked_add` on `i32`, `%` with negative numbers, `match` on `Option` | [LC 7](https://leetcode.com/problems/reverse-integer/) |
| 0100 | [Count Primes](exercises/p0100_count_primes.md) | Medium | sieve of Eratosthenes, `vec![true; n]`, `step_by`, `usize` indices, `as` casts | [LC 204](https://leetcode.com/problems/count-primes/) |

<!-- PROBLEM_TABLE:END -->
