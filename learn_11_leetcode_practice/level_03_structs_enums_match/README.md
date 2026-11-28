# Level 3: Structs, Enums & Pattern Matching (problems 0201-0300)

> Focus: structs, `impl` blocks, enums with data, `match`, `Option`, `if let`, `let else`.

This level covers chapters 5 (structs), 6 (enums and `match`) and 19 (patterns) of The Rust Book.
You will model data with structs and enums, give them methods, and let exhaustive `match`
expressions, guards, `@` bindings and slice patterns do the work that `if` chains would do in
other languages. 50 problems come from LeetCode (25 Easy, 22 Medium and 3 Hard, using LeetCode's
exact Rust signatures, including 12 "design" problems where you implement a struct with `new` and
methods); the other 50 are original problems that model small domains: shapes, traffic lights,
coins, a vending machine, a library, an RPG battle, parsers and state machines.

## Prerequisites

- Levels 1 and 2, or the same knowledge: functions, loops, `Vec`, slices, ownership and borrowing,
  `String` vs `&str`.
- Read [The Rust Book 5](https://doc.rust-lang.org/book/ch05-00-structs.html),
  [6](https://doc.rust-lang.org/book/ch06-00-enums.html) and
  [19](https://doc.rust-lang.org/book/ch19-00-patterns.html) before or alongside this level.
- A few problems use `HashSet` or iterator adapters such as `filter_map` and `fold`; only the basic
  calls are needed (collections and iterators are covered in depth in levels 4 and 5).

## What you will practice

- Defining structs, field init shorthand, struct update syntax `..base`, tuple structs and unit
  structs: [Book 5.1](https://doc.rust-lang.org/book/ch05-01-defining-structs.html),
  [Book 5.2](https://doc.rust-lang.org/book/ch05-02-example-structs.html).
- Methods and associated functions, `self` / `&self` / `&mut self`, several `impl` blocks,
  associated constants, private fields that protect an invariant:
  [Book 5.3](https://doc.rust-lang.org/book/ch05-03-method-syntax.html).
- `#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]` and what each derived
  trait means: [Book Appendix C](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html).
- Enums with unit, tuple and struct-like variants, explicit discriminants, enums inside enums:
  [Book 6.1](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html),
  [Reference: enumerations](https://doc.rust-lang.org/reference/items/enumerations.html).
- `Option<T>` in depth: `match`, `?`, `map`, `and_then`, `or_else`, `unwrap_or*`, `is_some_and`,
  `as_ref`, `as_deref`, `as_mut`, `take`, `replace`, `get_or_insert_with`:
  [`Option`](https://doc.rust-lang.org/std/option/enum.Option.html).
- Exhaustive `match`, `if let`, `let ... else` and `while let`:
  [Book 6.2](https://doc.rust-lang.org/book/ch06-02-match.html),
  [Book 6.3](https://doc.rust-lang.org/book/ch06-03-if-let.html),
  [Book 19.1](https://doc.rust-lang.org/book/ch19-01-all-the-places-for-patterns.html).
- Pattern syntax: literals, `|`, ranges (`a..=b`, `a..`, `a..b`), `@` bindings, match guards,
  `_` and `..`, nested struct and enum patterns, slice patterns with `rest @ ..`, binding modes and
  `ref`/`ref mut`, and the [`matches!`](https://doc.rust-lang.org/std/macro.matches.html) macro:
  [Book 19.3](https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html),
  [Reference: patterns](https://doc.rust-lang.org/reference/patterns.html).
- Parsing text into structs and enums (`split_once`, `strip_prefix`, `parse().ok()`), then working
  with typed values instead of strings.
- State machines (`match (state, event)`), step-by-step simulations, and LeetCode design problems.

## Roadmap

| Problems | Theme |
| -------- | ----- |
| 0201-0215 | Structs: definitions, shorthand, update syntax, tuple and unit structs, methods, `self` kinds, derives, `impl` blocks, first design problems, `Copy` structs |
| 0216-0230 | Enums: fieldless and data-carrying variants, a state cycle, parsing characters and words into enums, discriminants, derived ordering |
| 0231-0240 | `Option<T>`: returning and matching it, defaults, chaining, `Option<&T>`, `take`/`replace`, `if let`, `let else`, `while let` |
| 0241-0244 | Parsing dates and fixed-width records into structs |
| 0245-0257 | Pattern syntax: ranges, char classes, `matches!`, `@`, guards, tuple, slice and nested patterns, binding modes, ignoring values |
| 0258-0262 | Classic matching problems: Roman numerals, brackets, UTF-8 and IP address validation |
| 0263-0273 | State machines and simulations: door lock, vending machine, change, order workflow, robots, elevator, board games |
| 0274-0278 | Grid simulations: spiral matrices, Game of Life, Sudoku, guarded cells |
| 0279-0290 | Domain models and parsers: library, RPG battle, command and INI parsers, versions, money, tokens, calculators, a recursive JSON enum |
| 0291-0300 | LeetCode design problems and a Hard finale: browser history, bank, stacks, queues, robot, ATM, text editor, valid number, boolean expressions |

## How to work on a problem

```bash
cd learn_11_leetcode_practice/level_03_structs_enums_match

cargo test practice::p0201      # run the tests for the problem you are solving
cargo test solutions::p0201     # run the tests against the reference solution
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
| 0201 | [Rectangle Area with a Struct](exercises/p0201_rectangle_area.md) | Easy | defining a `struct`, named fields, field access, borrowing a struct with `&` | Original |
| 0202 | [Build a User with Field Init Shorthand](exercises/p0202_build_user.md) | Easy | struct literals, field init shorthand, mutable struct instances, `String` fields | Original |
| 0203 | [Copy a Profile with Struct Update Syntax](exercises/p0203_struct_update_syntax.md) | Easy | struct update syntax `..base`, moves of non-`Copy` fields, `clone` | Original |
| 0204 | [Tuple Structs: Colors and Units](exercises/p0204_tuple_struct_newtypes.md) | Easy | tuple structs, `.0` / `.1` field access, destructuring with `let`, newtypes | Original |
| 0205 | [Unit Structs as Zero-Sized Tools](exercises/p0205_unit_struct_markers.md) | Easy | unit-like structs, methods on a struct without fields, `std::mem::size_of` | Original |
| 0206 | [Rectangle Methods and a Square Constructor](exercises/p0206_rectangle_methods.md) | Easy | `impl` blocks, methods with `&self`, associated functions, `Self` | Original |
| 0207 | [A Bank Account with `&mut self`](exercises/p0207_bank_account_mut_self.md) | Easy | methods with `&mut self`, private fields, getters, keeping an invariant | Original |
| 0208 | [Redeem a Ticket by Value](exercises/p0208_consuming_self_ticket.md) | Easy | methods taking `self` by value, moving fields out of a struct, consuming APIs | Original |
| 0209 | [Derive Debug, Clone and PartialEq](exercises/p0209_derive_debug_clone_partialeq.md) | Easy | `#[derive(Debug, Clone, PartialEq)]`, `{:?}` and `{:#?}`, `clone`, `==` on structs | Original |
| 0210 | [Temperature with Multiple impl Blocks](exercises/p0210_multiple_impl_blocks_temperature.md) | Easy | several `impl` blocks for one type, associated constants, private fields, `Self` | Original |
| 0211 | [Design Parking System](exercises/p0211_design_parking_system.md) | Easy | LeetCode "design" problems, `new` constructor, `&mut self`, arrays as fields | [LC 1603](https://leetcode.com/problems/design-parking-system/) |
| 0212 | [Range Sum Query - Immutable](exercises/p0212_range_sum_query_immutable.md) | Easy | design struct, precomputing in `new`, `&self` queries, prefix sums | [LC 303](https://leetcode.com/problems/range-sum-query-immutable/) |
| 0213 | [Design Neighbor Sum Service](exercises/p0213_design_neighbor_sum_service.md) | Easy | design struct, private helper methods, lookup table built in `new`, `checked_add_signed` | [LC 3242](https://leetcode.com/problems/design-neighbor-sum-service/) |
| 0214 | [A 24-Hour Clock Struct](exercises/p0214_clock_time_arithmetic.md) | Easy | normalising constructor, private fields, `rem_euclid`, `#[derive(Clone, Copy, PartialEq, Eq)]` | Original |
| 0215 | [2D Vectors as Copy Structs](exercises/p0215_vector2_copy_struct.md) | Easy | `#[derive(Clone, Copy)]`, methods taking `self` by value, methods returning `Self`, associated constants | Original |
| 0216 | [Compass Directions](exercises/p0216_compass_direction.md) | Easy | fieldless `enum`, exhaustive `match`, methods on enums, `Option<Self>` constructor | Original |
| 0217 | [Coins and State Quarters](exercises/p0217_coin_values.md) | Easy | enum variant holding another enum, binding a value inside a pattern, `{:?}` on an enum | Original |
| 0218 | [Four Kinds of Messages](exercises/p0218_message_enum.md) | Easy | unit, tuple and struct-like enum variants, destructuring variants in `match`, `&mut self` | Original |
| 0219 | [Shapes: Area and Perimeter](exercises/p0219_shape_area_perimeter.md) | Easy | enum variants with named fields, methods on enums, `match` with destructuring, `Option<usize>` | Original |
| 0220 | [Traffic Light Cycle](exercises/p0220_traffic_light_cycle.md) | Easy | enum as a state machine, `next(self) -> Self`, methods returning data per variant, simulation loop | Original |
| 0221 | [Robot Return to Origin](exercises/p0221_robot_return_to_origin.md) | Easy | parsing a `char` into an `enum`, `Option` from a parser, `match`, simulation | [LC 657](https://leetcode.com/problems/robot-return-to-origin/) |
| 0222 | [Final Value of Variable After Performing Operations](exercises/p0222_final_value_of_variable_after_performing_operations.md) | Easy | parsing strings into an `enum`, `match` on `&str` with `\|` alternatives, `fold` | [LC 2011](https://leetcode.com/problems/final-value-of-variable-after-performing-operations/) |
| 0223 | [Furthest Point From Origin](exercises/p0223_furthest_point_from_origin.md) | Easy | enum with a "wildcard" variant, counting with a struct, `match` on `char` | [LC 2833](https://leetcode.com/problems/furthest-point-from-origin/) |
| 0224 | [Crawler Log Folder](exercises/p0224_crawler_log_folder.md) | Easy | parsing strings into an `enum`, `match` on `&str`, `saturating_sub`, `fold` | [LC 1598](https://leetcode.com/problems/crawler-log-folder/) |
| 0225 | [Snake in Matrix](exercises/p0225_snake_in_matrix.md) | Easy | command `enum`, position `struct`, methods with `&mut self`, `match` on `&str` | [LC 3248](https://leetcode.com/problems/snake-in-matrix/) |
| 0226 | [Determine Color of a Chessboard Square](exercises/p0226_determine_color_of_a_chessboard_square.md) | Easy | returning an `enum` from a helper, slice pattern with `let ... else`, byte arithmetic | [LC 1812](https://leetcode.com/problems/determine-color-of-a-chessboard-square/) |
| 0227 | [Weekdays with Explicit Discriminants](exercises/p0227_weekday_discriminants.md) | Easy | C-like enums with explicit discriminants, `as` casts from an enum, number to `Option<Enum>`, `rem_euclid` | Original |
| 0228 | [Ordering Playing Cards](exercises/p0228_playing_card_ordering.md) | Easy | `#[derive(PartialOrd, Ord)]` on enums and structs, field order matters, `max`, `sort`, `match` on a tuple of `bool`s | Original |
| 0229 | [Count Items Matching a Rule](exercises/p0229_count_items_matching_a_rule.md) | Easy | `enum` as a field selector, parsing `&str` into `Option<Enum>`, slice indexing, `filter` + `count` | [LC 1773](https://leetcode.com/problems/count-items-matching-a-rule/) |
| 0230 | [Length Units with Enum Data](exercises/p0230_unit_conversion_enum.md) | Easy | enum variants with different data shapes, converting between variants, `round`, `format!` precision | Original |
| 0231 | [Checked Division with Option](exercises/p0231_checked_division_option.md) | Easy | returning `Option<T>`, `Some` / `None` as success / failure, overflow edge cases, early `return None` | Original |
| 0232 | [Matching on Option](exercises/p0232_option_match_plus_one.md) | Easy | `match` on `Option<T>`, `Some(x)` / `None` patterns, matching a tuple of two `Option`s, `Option<&str>` | Original |
| 0233 | [Defaults for Missing Values](exercises/p0233_option_defaults.md) | Easy | `unwrap_or`, `unwrap_or_else`, `unwrap_or_default`, `map_or`, `is_some_and`, `Option` fields in a struct | Original |
| 0234 | [Option Chaining Through Nested Fields](exercises/p0234_option_chaining_address.md) | Easy | `Option::as_ref`, `as_deref`, `and_then`, `map`, `or_else`, the `?` operator on `Option`, nested structs | Original |
| 0235 | [Inventory Lookup](exercises/p0235_inventory_lookup.md) | Easy | returning `Option<&T>` and `Option<&mut T>`, `iter().find`, `iter_mut().find`, `min_by_key`, private `Vec` field | Original |
| 0236 | [Parking Slots with take and replace](exercises/p0236_option_take_replace_slot.md) | Easy | `Option::take`, `Option::replace`, `get_or_insert_with`, `as_mut`, `as_deref`, `Vec<Option<T>>` | Original |
| 0237 | [Design an Ordered Stream](exercises/p0237_design_an_ordered_stream.md) | Easy | design struct, `Vec<Option<String>>`, `while let`, `Option::take`, `and_then` | [LC 1656](https://leetcode.com/problems/design-an-ordered-stream/) |
| 0238 | [Settings with if let](exercises/p0238_if_let_settings.md) | Easy | `if let`, `if let ... else`, matching one variant out of many, `if let` with `&mut`, nested patterns | Original |
| 0239 | [Parse Records with let else](exercises/p0239_let_else_parse_record.md) | Easy | `let ... else` with early return, refutable patterns, `split_once`, `parse::<u8>().ok()`, building a struct | Original |
| 0240 | [Undo Stack with while let](exercises/p0240_while_let_undo_stack.md) | Easy | `while let Some(x) = stack.pop()`, an enum recording changes, `String::truncate`, `std::mem::take` | Original |
| 0241 | [Day of the Year](exercises/p0241_day_of_the_year.md) | Easy | parsing into a `Date` struct, `let ... else`, methods on a struct, leap-year rule, `match` on month numbers | [LC 1154](https://leetcode.com/problems/day-of-the-year/) |
| 0242 | [Number of Senior Citizens](exercises/p0242_number_of_senior_citizens.md) | Easy | parsing fixed-width records into a struct, an `enum` field, string slicing with `get`, `filter_map` | [LC 2678](https://leetcode.com/problems/number-of-senior-citizens/) |
| 0243 | [Determine if Two Events Have Conflict](exercises/p0243_determine_if_two_events_have_conflict.md) | Easy | a `Time` struct with derived `PartialOrd`/`Ord`, an `Event` struct of two times, parsing with `split_once` | [LC 2446](https://leetcode.com/problems/determine-if-two-events-have-conflict/) |
| 0244 | [Number of Days Between Two Dates](exercises/p0244_number_of_days_between_two_dates.md) | Easy | a `Date` struct with a "days since epoch" method, parsing with `?`, `abs_diff`, `match` with a guard | [LC 1360](https://leetcode.com/problems/number-of-days-between-two-dates/) |
| 0245 | [Letter Grades with Range Patterns](exercises/p0245_letter_grade_range_patterns.md) | Easy | inclusive range patterns `a..=b`, half-open range patterns `a..`, exclusive range patterns `a..b`, enums as results | Original |
| 0246 | [Classify Characters](exercises/p0246_char_classifier.md) | Easy | `char` range patterns, `\|` alternatives, arm order, counting into a struct with `#[derive(Default)]` | Original |
| 0247 | [Strong Password Checker II](exercises/p0247_strong_password_checker_ii.md) | Easy | the `matches!` macro, `char` range patterns, `any`, `windows`, a small rules struct | [LC 2299](https://leetcode.com/problems/strong-password-checker-ii/) |
| 0248 | [HTTP Status Classes with @ Bindings](exercises/p0248_http_status_at_bindings.md) | Easy | `name @ pattern` bindings, `@` with ranges and with `\|` sub-patterns, binding inside struct patterns | Original |
| 0249 | [Describe Pairs with Match Guards](exercises/p0249_match_guards_pairs.md) | Easy | match guards `if ...`, guards that use outer variables, guards with `\|` alternatives, arm order | Original |
| 0250 | [Rock, Paper, Scissors](exercises/p0250_rock_paper_scissors.md) | Easy | `match` on a tuple of two enums, exhaustive tuple patterns, enum methods, tallying into a struct | Original |
| 0251 | [Describe a List with Slice Patterns](exercises/p0251_slice_patterns_describe.md) | Easy | slice patterns `[]`, `[x]`, `[first, .., last]`, rest bindings `rest @ ..`, recursion on slices | Original |
| 0252 | [Nested Destructuring of UI Events](exercises/p0252_nested_destructuring_events.md) | Medium | nested enum and struct patterns, literals inside nested patterns, `..` in struct patterns, `if let` with nested patterns | Original |
| 0253 | [Matching Through References](exercises/p0253_binding_modes_borrowed_match.md) | Medium | default binding modes, `ref` and `ref mut`, matching on `&T` and `&mut T`, destructuring in `for` loops, partial moves | Original |
| 0254 | [Ignoring Parts of a Value](exercises/p0254_ignore_with_rest_patterns.md) | Easy | `_` wildcard, `..` rest pattern in tuples, structs and slices, `_name` vs `_`, patterns in `let`, `for` and parameters | Original |
| 0255 | [Latest Time by Replacing Hidden Digits](exercises/p0255_latest_time_by_replacing_hidden_digits.md) | Easy | `match` on tuples of `char`s, `char` range patterns, slice patterns on bytes, greedy choice | [LC 1736](https://leetcode.com/problems/latest-time-by-replacing-hidden-digits/) |
| 0256 | [Day of the Week](exercises/p0256_day_of_the_week.md) | Easy | a `Weekday` enum with `name()`, converting a count into an enum, `rem_euclid`, leap years | [LC 1185](https://leetcode.com/problems/day-of-the-week/) |
| 0257 | [Reformat Date](exercises/p0257_reformat_date.md) | Easy | `match` on string slices, slice patterns on `split_whitespace` results, `trim_end_matches`, zero-padded `format!` | [LC 1507](https://leetcode.com/problems/reformat-date/) |
| 0258 | [Roman to Integer](exercises/p0258_roman_to_integer.md) | Easy | `match` on `char` returning `Option`, look-ahead with `peekable`, `while let`, subtractive notation | [LC 13](https://leetcode.com/problems/roman-to-integer/) |
| 0259 | [Integer to Roman](exercises/p0259_integer_to_roman.md) | Medium | a `const` table of tuples, destructuring tuples in a `for` loop, greedy subtraction, `String::push_str` | [LC 12](https://leetcode.com/problems/integer-to-roman/) |
| 0260 | [Valid Parentheses](exercises/p0260_valid_parentheses.md) | Easy | `match` on a tuple `(Option<char>, char)`, `\|` alternatives across tuple patterns, a `Vec` as a stack | [LC 20](https://leetcode.com/problems/valid-parentheses/) |
| 0261 | [UTF-8 Validation](exercises/p0261_utf_8_validation.md) | Medium | binary literal range patterns, an `enum` state machine, `match` on `(state, byte)`, `fold`-like loop | [LC 393](https://leetcode.com/problems/utf-8-validation/) |
| 0262 | [Validate IP Address](exercises/p0262_validate_ip_address.md) | Medium | an `enum` result with a `name()` method, slice patterns on bytes, `all`, `is_ascii_hexdigit` | [LC 468](https://leetcode.com/problems/validate-ip-address/) |
| 0263 | [Door Lock State Machine](exercises/p0263_door_lock_state_machine.md) | Medium | state machine with `match (state, event)`, enum states carrying data, guards in transitions, `Copy` enums | Original |
| 0264 | [Vending Machine](exercises/p0264_vending_machine.md) | Medium | input and output enums carrying data, a struct holding machine state, `&mut self` event handler, `get_mut` + `let else` | Original |
| 0265 | [Lemonade Change](exercises/p0265_lemonade_change.md) | Easy | a `Bill` enum parsed from `i32`, a `Till` struct with a `&mut self` method, greedy choice in `match` arms | [LC 860](https://leetcode.com/problems/lemonade-change/) |
| 0266 | [Baseball Game](exercises/p0266_baseball_game.md) | Easy | parsing strings into an operation `enum` with data, `match` on `&str` with a binding fallback, slice patterns on a stack | [LC 682](https://leetcode.com/problems/baseball-game/) |
| 0267 | [Order Status Workflow](exercises/p0267_order_status_workflow.md) | Medium | enum states with data, transition methods returning `Option<Self>`, `matches!` for state groups, replaying actions | Original |
| 0268 | [Robot Bounded In Circle](exercises/p0268_robot_bounded_in_circle.md) | Medium | a heading `enum` with turn methods, a robot `struct` with `&mut self` methods, simulation, reasoning about periodicity | [LC 1041](https://leetcode.com/problems/robot-bounded-in-circle/) |
| 0269 | [Walking Robot Simulation](exercises/p0269_walking_robot_simulation.md) | Medium | command `enum` parsed from integers, robot `struct` with heading `enum`, `HashSet` of obstacles, step-by-step simulation | [LC 874](https://leetcode.com/problems/walking-robot-simulation/) |
| 0270 | [Elevator Simulation](exercises/p0270_elevator_simulation.md) | Medium | struct + direction `enum` state, tick-based simulation, `match` with guards to choose a direction, `Vec<bool>` request table | Original |
| 0271 | [Find Winner on a Tic Tac Toe Game](exercises/p0271_find_winner_on_a_tic_tac_toe_game.md) | Easy | a `Cell` enum grid, a `GameResult` enum, `Option<Player>` cells, checking lines with a `const` table | [LC 1275](https://leetcode.com/problems/find-winner-on-a-tic-tac-toe-game/) |
| 0272 | [Valid Tic-Tac-Toe State](exercises/p0272_valid_tic_tac_toe_state.md) | Medium | summarising a board into a struct, `match` on a tuple of counts and `bool`s, guards, `matches!` | [LC 794](https://leetcode.com/problems/valid-tic-tac-toe-state/) |
| 0273 | [Available Captures for Rook](exercises/p0273_available_captures_for_rook.md) | Easy | a `Piece` enum parsed from `char`, walking a grid in four directions, `match` deciding when to stop, `find_map` | [LC 999](https://leetcode.com/problems/available-captures-for-rook/) |
| 0274 | [Spiral Matrix](exercises/p0274_spiral_matrix.md) | Medium | a `Bounds` struct that shrinks, a direction `enum` with `turn`, `match` on the direction, simulation | [LC 54](https://leetcode.com/problems/spiral-matrix/) |
| 0275 | [Spiral Matrix II](exercises/p0275_spiral_matrix_ii.md) | Medium | a direction `enum` with `delta` and `turn`, a cursor `struct`, "turn when blocked" simulation | [LC 59](https://leetcode.com/problems/spiral-matrix-ii/) |
| 0276 | [Game of Life](exercises/p0276_game_of_life.md) | Medium | a cell-state `enum` with four variants for in-place updates, `match (alive, neighbours)`, `&mut Vec<Vec<i32>>` | [LC 289](https://leetcode.com/problems/game-of-life/) |
| 0277 | [Valid Sudoku](exercises/p0277_valid_sudoku.md) | Medium | `match` on `char` with a range pattern and a binding, a `Seen` struct of bit masks, box index arithmetic | [LC 36](https://leetcode.com/problems/valid-sudoku/) |
| 0278 | [Count Unguarded Cells in the Grid](exercises/p0278_count_unguarded_cells_in_the_grid.md) | Medium | a `Cell` enum grid, `matches!` to stop a ray, `while let` over an `Option` cursor, counting with `filter` | [LC 2257](https://leetcode.com/problems/count-unguarded-cells-in-the-grid/) |
| 0279 | [Library Checkout](exercises/p0279_library_checkout.md) | Medium | enum status with data inside a struct, `&mut` access to a variant's fields, `match` on `&mut self.status`, late fees | Original |
| 0280 | [RPG Battle Simulation](exercises/p0280_rpg_battle_simulation.md) | Medium | structs and enums working together, `match` on `(class, action)`, borrowing two elements of a `Vec` one after the other, outcome enum | Original |
| 0281 | [Parse Robot Commands](exercises/p0281_parse_robot_commands.md) | Medium | parsing text into `Option<Command>`, slice patterns over words (`["say", rest @ ..]`), guards on slices, `enumerate` for line numbers | Original |
| 0282 | [INI Line Parser](exercises/p0282_ini_line_parser.md) | Medium | an enum of line kinds with data, `strip_prefix` / `strip_suffix` in `if let`, `split_once`, tracking state while scanning | Original |
| 0283 | [Semantic Versions](exercises/p0283_semantic_version.md) | Medium | parsing into a struct, derived `PartialOrd`/`Ord` on a struct, an enum selecting what to change, struct update syntax, `filter_map` + `max` | Original |
| 0284 | [Money with Currencies](exercises/p0284_money_with_currency.md) | Medium | a struct with an enum field, `Option` for currency mismatches, `checked_add`/`checked_sub`, `?` chains, associated functions | Original |
| 0285 | [Expression Tokenizer](exercises/p0285_expression_tokenizer.md) | Medium | a `Token` enum with data, scanning `char`s with `peekable` and `while let`, `match` with ranges and guards, `Option<Vec<Token>>` | Original |
| 0286 | [Evaluate Reverse Polish Notation](exercises/p0286_evaluate_reverse_polish_notation.md) | Medium | parsing strings into a `Token` enum, an `Op` enum with an `apply` method, a stack, slice patterns on the stack | [LC 150](https://leetcode.com/problems/evaluate-reverse-polish-notation/) |
| 0287 | [Complex Number Multiplication](exercises/p0287_complex_number_multiplication.md) | Medium | a `Complex` struct with `parse`, `times` and `text` methods, `split_once`, `strip_suffix`, `?` on `Option` | [LC 537](https://leetcode.com/problems/complex-number-multiplication/) |
| 0288 | [Fraction Addition and Subtraction](exercises/p0288_fraction_addition_and_subtraction.md) | Medium | a `Fraction` struct that keeps itself normalised, a constructor enforcing invariants, scanning signed numbers, `fold` | [LC 592](https://leetcode.com/problems/fraction-addition-and-subtraction/) |
| 0289 | [Basic Calculator II](exercises/p0289_basic_calculator_ii.md) | Medium | tokenizing into an enum, operator precedence with a "pending operator" state, `match` on `(Op, value)`, a stack of terms | [LC 227](https://leetcode.com/problems/basic-calculator-ii/) |
| 0290 | [A JSON-like Value Enum](exercises/p0290_json_value_enum.md) | Medium | a recursive `enum` (through `Vec`), recursion with `match`, returning `Option<&T>` into a tree, escaping strings | Original |
| 0291 | [Design Browser History](exercises/p0291_design_browser_history.md) | Medium | design struct with a `Vec` and a cursor, `Vec::truncate`, clamping with `min` / `saturating_sub`, `&mut self` methods | [LC 1472](https://leetcode.com/problems/design-browser-history/) |
| 0292 | [Simple Bank System](exercises/p0292_simple_bank_system.md) | Medium | design struct, validating 1-based account numbers into `Option<usize>`, `get_mut`, checking before mutating | [LC 2043](https://leetcode.com/problems/simple-bank-system/) |
| 0293 | [Design a Stack With Increment Operation](exercises/p0293_design_a_stack_with_increment_operation.md) | Medium | design struct with two parallel `Vec`s, lazy updates, `Option` from `pop`, keeping an invariant across methods | [LC 1381](https://leetcode.com/problems/design-a-stack-with-increment-operation/) |
| 0294 | [Design Circular Queue](exercises/p0294_design_circular_queue.md) | Medium | ring-buffer struct, index arithmetic with `%`, methods with `&self` and `&mut self`, sentinel values | [LC 622](https://leetcode.com/problems/design-circular-queue/) |
| 0295 | [Find Consecutive Integers from a Data Stream](exercises/p0295_find_consecutive_integers_from_a_data_stream.md) | Medium | design struct holding streaming state, `&mut self` update method, constant memory instead of storing the stream | [LC 2526](https://leetcode.com/problems/find-consecutive-integers-from-a-data-stream/) |
| 0296 | [Walking Robot Simulation II](exercises/p0296_walking_robot_simulation_ii.md) | Medium | design struct, a `Facing` enum with `name()`, reducing a long walk with `%`, mapping a perimeter index to coordinates with `match` guards | [LC 2069](https://leetcode.com/problems/walking-robot-simulation-ii/) |
| 0297 | [Design an ATM Machine](exercises/p0297_design_an_atm_machine.md) | Medium | design struct with fixed-size arrays, greedy plan computed before mutating, `zip` over parallel arrays, `[-1]` sentinel | [LC 2241](https://leetcode.com/problems/design-an-atm-machine/) |
| 0298 | [Design a Text Editor](exercises/p0298_design_a_text_editor.md) | Hard | design struct with two stacks around a cursor, moving characters between `Vec`s, `split_off`, returning a window of text | [LC 2296](https://leetcode.com/problems/design-a-text-editor/) |
| 0299 | [Valid Number](exercises/p0299_valid_number.md) | Hard | finite state machine with an `enum`, classifying characters into an `enum`, `match (state, class)` transition table, accepting states with `matches!` | [LC 65](https://leetcode.com/problems/valid-number/) |
| 0300 | [Parsing A Boolean Expression](exercises/p0300_parsing_a_boolean_expression.md) | Hard | an operator `enum` with an `eval` method, a stack of `Frame` structs, `match` on `char`, `Option` for the pending operator | [LC 1106](https://leetcode.com/problems/parsing-a-boolean-expression/) |

<!-- PROBLEM_TABLE:END -->
