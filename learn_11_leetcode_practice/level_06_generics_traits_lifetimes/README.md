# Level 6: Generics, Traits & Lifetimes (problems 0501-0600)

> Focus: generics, traits, trait objects, standard traits, operator overloading, lifetimes.

This level covers chapter 10 (generics, traits, lifetimes), chapter 18 (trait objects and the state
pattern) and sections 20.2-20.3 (advanced traits and types) of The Rust Book. You write generic
functions and data structures, define traits with default methods, implement the standard traits
(`Display`, `Debug`, `Ord`, `Hash`, `From`, `FromStr`, `Iterator`, `Index`, `Deref`, the operators...),
build plugin-style designs with `Box<dyn Trait>`, and write structs and functions whose signatures
need explicit lifetimes. 26 problems come from LeetCode (6 Easy and 20 Medium, using LeetCode's exact
Rust signatures, mostly *design* problems such as Min Stack or Snapshot Array); the other 74 are
original problems, each built around one generics, trait or lifetime feature.

## Prerequisites

- Levels 1-5, or the same knowledge: structs, enums and `match`, `Vec`/`HashMap`/`BTreeMap`,
  `Option`/`Result` with `?`, closures and iterator adapters.
- Read [The Rust Book 10](https://doc.rust-lang.org/book/ch10-00-generics.html) before starting, then
  [18.2-18.3](https://doc.rust-lang.org/book/ch18-02-trait-objects.html) before 0581 and
  [20.2](https://doc.rust-lang.org/book/ch20-02-advanced-traits.html) /
  [20.3](https://doc.rust-lang.org/book/ch20-03-advanced-types.html) before 0590.
- Design problems (a struct with `new` and methods instead of `impl Solution`) were introduced in
  level 3; here they are paired with generic versions of the same structures.

## What you will practice

- Generic functions with trait bounds, `+` and `where` clauses, operator traits as bounds
  (`Add<Output = T>`), closure bounds (`F: Fn(&T) -> K`):
  [Book 10.1](https://doc.rust-lang.org/book/ch10-01-syntax.html),
  [Book 10.2](https://doc.rust-lang.org/book/ch10-02-traits.html#trait-bound-syntax).
- Generic structs and enums (`Point<T>`, `Either<L, R>`, `Stack<T>`, `RingBuffer<T>`, `Grid<T>`),
  impl blocks for one concrete type (`impl Point<f64>`) and conditional methods
  (`impl<T: PartialOrd + Display> Pair<T>`).
- Defining traits, default methods, `impl Trait` in argument and return position, extension traits
  and the orphan rule: [Book 10.2](https://doc.rust-lang.org/book/ch10-02-traits.html).
- Standard traits: [`Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html),
  [`Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html) by hand,
  [`Default`](https://doc.rust-lang.org/std/default/trait.Default.html),
  `Clone`/`Copy`, `PartialEq`/`Eq`/[`Hash`](https://doc.rust-lang.org/std/hash/trait.Hash.html)
  consistency, [`Ord`](https://doc.rust-lang.org/std/cmp/trait.Ord.html),
  [`From`](https://doc.rust-lang.org/std/convert/trait.From.html)/`Into`,
  [`TryFrom`](https://doc.rust-lang.org/std/convert/trait.TryFrom.html),
  [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html),
  [`AsRef`](https://doc.rust-lang.org/std/convert/trait.AsRef.html) and
  [`PhantomData`](https://doc.rust-lang.org/std/marker/struct.PhantomData.html).
- Operator overloading (`Add`, `Sub`, `Mul`, `Neg`, `AddAssign`, `Sum`), `Index`/`IndexMut` and
  `Deref` on newtypes such as `Money`, `Fraction`, `Polynomial` and `Matrix<T>`:
  [`std::ops`](https://doc.rust-lang.org/std/ops/index.html).
- Lifetimes in functions, structs and impl blocks, the elision rules, `'static`, `T: 'a` and
  structs that hold `&str`: [Book 10.3](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html),
  [Reference: lifetime elision](https://doc.rust-lang.org/reference/lifetime-elision.html).
- Implementing [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) and
  [`IntoIterator`](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html), overriding `nth` and
  `size_hint`, writing adapters and blanket extension traits.
- Trait objects: `Box<dyn Trait>`, `&dyn Trait`, `self: Box<Self>`, dyn compatibility, cloning
  boxed objects and downcasting with [`Any`](https://doc.rust-lang.org/std/any/index.html):
  [Book 18.2](https://doc.rust-lang.org/book/ch18-02-trait-objects.html),
  [Book 18.3](https://doc.rust-lang.org/book/ch18-03-oo-design-patterns.html),
  [Reference: dyn compatibility](https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility).
- Advanced traits and types: fully qualified syntax, supertraits, the newtype pattern, associated
  constants and types, generic traits vs associated types, blanket impls, type aliases, `!`,
  `?Sized`, phantom unit types and const generics:
  [Book 20.2](https://doc.rust-lang.org/book/ch20-02-advanced-traits.html),
  [Book 20.3](https://doc.rust-lang.org/book/ch20-03-advanced-types.html),
  [Reference: generics](https://doc.rust-lang.org/reference/items/generics.html).

## Roadmap

| Problems | Theme |
| -------- | ----- |
| 0501-0510 | Generic functions: trait bounds, `where` clauses, operator bounds, generic helpers for LeetCode matrices |
| 0511-0515 | Generic structs and enums: `Point<T>`, concrete impls, conditional methods, `Either<L, R>`, `Stack<T>` |
| 0516-0526 | Generic and LeetCode design structures: queues and stacks, Min Stack, ring buffer, `Grid<T>`, prefix sums, snapshots |
| 0527-0532 | Defining traits: default methods, `impl Trait` arguments and returns, extension traits, trait-provided constructors |
| 0533-0541 | Formatting, equality, hashing and ordering: `Display`, manual `Debug`, `Default`, `Hash`, `Ord`, custom sort keys |
| 0542-0549 | Conversions: `From`/`Into`, `TryFrom`, `FromStr`, `AsRef`, typed ids with `PhantomData` |
| 0550-0560 | Operator overloading, `Index`/`IndexMut`, `Deref`: vectors, units, money, fractions, polynomials, matrices, Bitset |
| 0561-0573 | Lifetimes: function signatures, elision, borrowed `HashMap` keys, structs holding `&str`, `'static`, `T: 'a`, a tokenizer |
| 0574-0580 | Implementing `Iterator` and `IntoIterator`: custom iterators, run-length decoding, nested lists, combinations, adapters |
| 0581-0589 | Trait objects: shapes, GUI components, plugins, pipelines, state pattern vs type-state, commands, dyn compatibility, `Any` |
| 0590-0600 | Advanced traits and types: qualified paths, supertraits, newtypes, associated consts and types, blanket impls, `!`, `?Sized`, units of measure, const generics |

## How to work on a problem

```bash
cd learn_11_leetcode_practice/level_06_generics_traits_lifetimes

cargo test practice::p0501      # run the tests for the problem you are solving
cargo test solutions::p0501     # run the tests against the reference solution
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
| 0501 | [Largest Element of a Generic Slice](exercises/p0501_largest_in_slice.md) | Easy | generic functions, trait bounds `PartialOrd + Copy`, `Option<T>`, `Option<&T>` | Original |
| 0502 | [Remove Consecutive Duplicates Generically](exercises/p0502_dedup_consecutive.md) | Easy | generic functions, `T: PartialEq + Clone`, `Vec<T>`, `&mut Vec<T>` | Original |
| 0503 | [Join Anything Displayable](exercises/p0503_join_display.md) | Easy | `where` clauses, `T: Display`, `std::fmt::Write`, generic functions over slices | Original |
| 0504 | [Generic Sum and Product](exercises/p0504_generic_sum_and_product.md) | Easy | operator traits as bounds, `Add<Output = T>`, `Mul<Output = T>`, `Default`, `From<u8>` | Original |
| 0505 | [Shift 2D Grid](exercises/p0505_shift_2d_grid.md) | Easy | generic helper `shift_grid_generic<T: Clone>`, flattening, `rotate_right`, `chunks` | [LC 1260](https://leetcode.com/problems/shift-2d-grid/) |
| 0506 | [Determine Whether Matrix Can Be Obtained By Rotation](exercises/p0506_determine_whether_matrix_can_be_obtained_by_rotation.md) | Easy | generic helper `rotate_clockwise<T: Clone>`, `PartialEq` on nested `Vec`, simulation | [LC 1886](https://leetcode.com/problems/determine-whether-matrix-can-be-obtained-by-rotation/) |
| 0507 | [Merge Two Sorted Slices](exercises/p0507_merge_sorted_slices.md) | Easy | `T: Ord + Clone`, `Ord::cmp`, `std::cmp::Ordering`, `T: PartialOrd` | Original |
| 0508 | [Generic Lower and Upper Bound](exercises/p0508_generic_binary_search.md) | Medium | generic binary search, `T: Ord`, `Result<usize, usize>`, half-open ranges | Original |
| 0509 | [Most Frequent Item](exercises/p0509_most_frequent_item.md) | Medium | multiple trait bounds, `where` clauses, `T: Hash + Eq + Ord + Clone`, `HashMap<&T, usize>` | Original |
| 0510 | [Group Items by a Key](exercises/p0510_group_by_key.md) | Medium | several type parameters, closure bounds `F: Fn(&T) -> K`, `K: Ord`, `BTreeMap` | Original |
| 0511 | [Generic Point with Concrete Impls](exercises/p0511_generic_point.md) | Easy | generic structs, `impl<T> Point<T>`, `impl Point<f64>`, `impl Point<i32>` | Original |
| 0512 | [Mixing Two Generic Points](exercises/p0512_point_mixup.md) | Easy | structs with several type parameters, generic methods with their own parameters, `FnOnce` | Original |
| 0513 | [Pair With Conditional Methods](exercises/p0513_pair_conditional_methods.md) | Easy | trait bounds on impl blocks, conditional methods, `PartialOrd`, `Display`, `Add` | Original |
| 0514 | [A Generic Either Enum](exercises/p0514_generic_either.md) | Easy | generic enums, `impl<L, R>`, consuming methods, `Option`, mapping with closures | Original |
| 0515 | [A Generic Stack](exercises/p0515_generic_stack.md) | Easy | generic data structure `Stack<T>`, `Option<&T>`, `Option<&mut T>`, `Default` | Original |
| 0516 | [Implement Queue using Stacks](exercises/p0516_implement_queue_using_stacks.md) | Easy | design struct, two stacks, amortized analysis, `&self` vs `&mut self`, `Default` | [LC 232](https://leetcode.com/problems/implement-queue-using-stacks/) |
| 0517 | [Min Stack](exercises/p0517_min_stack.md) | Medium | design struct, storing derived data with each element, tuples in a `Vec`, `Default` | [LC 155](https://leetcode.com/problems/min-stack/) |
| 0518 | [Online Stock Span](exercises/p0518_online_stock_span.md) | Medium | design struct, monotonic stack of `(price, span)` pairs, amortized analysis, `Default` | [LC 901](https://leetcode.com/problems/online-stock-span/) |
| 0519 | [Product of the Last K Numbers](exercises/p0519_product_of_the_last_k_numbers.md) | Medium | design struct, prefix products, handling zeros by resetting, `i64` intermediates, `Default` | [LC 1352](https://leetcode.com/problems/product-of-the-last-k-numbers/) |
| 0520 | [A Generic Ring Buffer](exercises/p0520_generic_ring_buffer.md) | Medium | generic struct `RingBuffer<T>`, `Vec<Option<T>>`, `Option::take`/`replace`, method-level `where T: Clone` | Original |
| 0521 | [Design Memory Allocator](exercises/p0521_design_memory_allocator.md) | Medium | design struct, `Vec<i32>` as memory, scanning for runs, `iter_mut`, `retain`-style counting | [LC 2502](https://leetcode.com/problems/design-memory-allocator/) |
| 0522 | [Design Front Middle Back Queue](exercises/p0522_design_front_middle_back_queue.md) | Medium | design struct, two balanced `VecDeque`s, private helper methods, invariants, `Default` | [LC 1670](https://leetcode.com/problems/design-front-middle-back-queue/) |
| 0523 | [Image Smoother](exercises/p0523_image_smoother.md) | Easy | generic struct `Grid<T>`, `impl<T> Grid<T>`, generic `map<U, F>`, neighbourhood queries | [LC 661](https://leetcode.com/problems/image-smoother/) |
| 0524 | [Range Sum Query 2D - Immutable](exercises/p0524_range_sum_query_2d_immutable.md) | Medium | design struct wrapping a generic `PrefixSum2D<T>`, bounds `Copy + Default + Add + Sub`, 2D prefix sums | [LC 304](https://leetcode.com/problems/range-sum-query-2d-immutable/) |
| 0525 | [Subrectangle Queries](exercises/p0525_subrectangle_queries.md) | Medium | design struct, small helper struct with a `contains` method, update log vs eager updates, `iter().rev().find()` | [LC 1476](https://leetcode.com/problems/subrectangle-queries/) |
| 0526 | [Snapshot Array](exercises/p0526_snapshot_array.md) | Medium | design struct, per-index history `Vec<Vec<(i32, i32)>>`, `partition_point`, `&self` queries | [LC 1146](https://leetcode.com/problems/snapshot-array/) |
| 0527 | [The Summary Trait](exercises/p0527_summary_trait.md) | Easy | defining a trait, required vs default methods, overriding a default, `impl Trait for Type` | Original |
| 0528 | [`impl Trait` Parameters](exercises/p0528_impl_trait_parameters.md) | Easy | `&impl Trait` parameters, trait bound syntax `<T: Trait>`, multiple bounds `impl A + B`, same type vs different types | Original |
| 0529 | [Returning `impl Trait`](exercises/p0529_impl_trait_return.md) | Easy | `-> impl Iterator<Item = T>`, `-> impl Fn`, `-> impl FnMut`, `move` closures, lifetime capture in edition 2024 | Original |
| 0530 | [An Extension Trait for `str`](exercises/p0530_extension_trait_for_str.md) | Easy | implementing your own trait for std types (`str`, `[i32]`), the orphan rule, method calls through `Deref` | Original |
| 0531 | [A Trait-Bounded Leaderboard](exercises/p0531_trait_bounded_leaderboard.md) | Medium | traits with `&str`-returning methods, generic functions over `&[T]` with `T: Scored`, `where` clauses, sorting references | Original |
| 0532 | [The Monoid Trait](exercises/p0532_monoid_trait.md) | Medium | traits with associated functions (`fn empty() -> Self`), `Self` in trait signatures, generic impls with bounds (`impl<A: Monoid, B: Monoid> Monoid for (A, B)`) | Original |
| 0533 | [Implementing `Display`](exercises/p0533_display_for_types.md) | Easy | `std::fmt::Display`, `fmt::Formatter`, `write!`, format specs (`{:02}`, `{:02X}`), `to_string` for free | Original |
| 0534 | [A Manual `Debug` That Redacts Secrets](exercises/p0534_manual_debug_redaction.md) | Easy | implementing `fmt::Debug` by hand, `debug_tuple`, `debug_map`, generic impls without bounds, `{:?}` vs `{:#?}` | Original |
| 0535 | [Sensible Defaults](exercises/p0535_default_trait.md) | Easy | implementing `Default`, `#[derive(Default)]` with a `#[default]` enum variant, struct update syntax `..Default::default()`, `T: Default` bounds | Original |
| 0536 | [A Case-Insensitive Key](exercises/p0536_case_insensitive_key.md) | Medium | manual `PartialEq`, `Eq` and `Hash`, the `Hash`/`Eq` consistency rule, `PartialEq<&str>`, `HashSet`/`HashMap` with custom keys | Original |
| 0537 | [Design Underground System](exercises/p0537_design_underground_system.md) | Medium | `#[derive(Hash, PartialEq, Eq)]` on a struct used as a `HashMap` key, design struct, running averages, `Default` | [LC 1396](https://leetcode.com/problems/design-underground-system/) |
| 0538 | [Ordering Tasks by Priority](exercises/p0538_ordering_for_tasks.md) | Medium | manual `Ord` and `PartialOrd`, `Ordering::then_with`, consistency with `Eq`, `BinaryHeap` as a max-heap, `std::cmp::Reverse` | Original |
| 0539 | [Rank Teams by Votes](exercises/p0539_rank_teams_by_votes.md) | Medium | manual `Ord` on a struct holding a `Vec`, lexicographic order of `Vec<u32>`, sorting with a custom order | [LC 1366](https://leetcode.com/problems/rank-teams-by-votes/) |
| 0540 | [Reward Top K Students](exercises/p0540_reward_top_k_students.md) | Medium | `HashSet<&str>` borrowing from the input, tuple sort keys `(Reverse(points), id)`, `sort_unstable_by_key` | [LC 2512](https://leetcode.com/problems/reward-top-k-students/) |
| 0541 | [Find the Kth Largest Integer in the Array](exercises/p0541_find_the_kth_largest_integer_in_the_array.md) | Medium | newtype `NumStr<'a>(&'a str)` with a manual `Ord`, numeric order vs string order, `select_nth_unstable` | [LC 1985](https://leetcode.com/problems/find-the-kth-largest-integer-in-the-array/) |
| 0542 | [Temperatures with `From` and `Into`](exercises/p0542_from_into_temperature.md) | Easy | newtypes, implementing `From`, getting `Into` for free, `impl Into<T>` parameters, generic `T: Into<U> + Copy` | Original |
| 0543 | [Validated Values with `TryFrom`](exercises/p0543_try_from_validation.md) | Easy | `TryFrom` with an associated `Error` type, `try_into()`, private fields to protect invariants, collecting `Result`s | Original |
| 0544 | [Parsing a Point with `FromStr`](exercises/p0544_from_str_point.md) | Easy | implementing `FromStr` with a custom error enum, `str::parse::<T>()`, `strip_prefix`/`strip_suffix`, `Display` round trip | Original |
| 0545 | [Minimum Time Difference](exercises/p0545_minimum_time_difference.md) | Medium | `FromStr` for a clock-time newtype, derived `Ord` on a newtype, sorting, circular differences | [LC 539](https://leetcode.com/problems/minimum-time-difference/) |
| 0546 | [Alert Using Same Key-Card Three or More Times in a One Hour Period](exercises/p0546_alert_using_same_key_card_three_or_more_times_in_a_one_hour_period.md) | Medium | `HashMap<&str, Vec<u16>>` borrowing names from the input, parsing `"HH:MM"`, sorting, `windows(3)` | [LC 1604](https://leetcode.com/problems/alert-using-same-key-card-three-or-more-times-in-a-one-hour-period/) |
| 0547 | [Semantic Versions](exercises/p0547_semantic_version.md) | Medium | `FromStr` + `Display` round trip, derived `PartialOrd`/`Ord` (field order matters), `Hash`, `filter_map` over parse results | Original |
| 0548 | [Flexible Inputs with `AsRef`](exercises/p0548_as_ref_generic_inputs.md) | Easy | `AsRef<str>`, `AsRef<[T]>`, `IntoIterator<Item = S>` bounds, accepting owned and borrowed inputs with one function | Original |
| 0549 | [Typed IDs with `PhantomData`](exercises/p0549_typed_ids_phantom_data.md) | Medium | `PhantomData<T>`, manual `Clone`/`Copy`/`PartialEq`/`Eq`/`Hash`/`Debug` impls without bounds on `T`, why `#[derive]` adds bounds | Original |
| 0550 | [2D Vector Operators](exercises/p0550_vector2_operators.md) | Easy | operator overloading with `Add`, `Sub`, `Neg`, `Mul<f64>`, `Mul<Vec2> for f64`, `AddAssign`, `SubAssign`, `iter::Sum` | Original |
| 0551 | [Adding Different Units](exercises/p0551_add_different_units.md) | Easy | default generic type parameters (`Add<Rhs = Self>`), `impl Add<Meters> for Millimeters`, choosing `Output`, `Mul<u32>` | Original |
| 0552 | [Indexing a Schedule by Weekday](exercises/p0552_index_by_enum.md) | Easy | `Index` and `IndexMut` with a non-integer index, generic struct `WeeklySchedule<T>`, `std::array::from_fn`, `impl<T: Default> Default` | Original |
| 0553 | [A Money Newtype](exercises/p0553_money_newtype.md) | Medium | newtype pattern with a private field, arithmetic operator traits, `Mul<i64>`, `Sum`, `Display` with signs and padding | Original |
| 0554 | [A Fraction Type](exercises/p0554_fraction_type.md) | Medium | a normalised value type, `Add`/`Sub`/`Mul`/`Div`/`Neg`, manual `Ord` by cross-multiplication, `From<i64>`, `Display` | Original |
| 0555 | [Solve the Equation](exercises/p0555_solve_the_equation.md) | Medium | a small value type `Linear` with `Add`/`Sub`/`Neg`/`Sum`, parsing terms with signs, `FromStr` | [LC 640](https://leetcode.com/problems/solve-the-equation/) |
| 0556 | [Polynomial Arithmetic](exercises/p0556_polynomial_arithmetic.md) | Medium | operator traits implemented for references (`impl Add for &Polynomial`), normalised representation, Horner's method, `Display` with signs | Original |
| 0557 | [A Generic Matrix](exercises/p0557_generic_matrix.md) | Medium | generic struct `Matrix<T>`, `Index<(usize, usize)>`/`IndexMut`, `Add`/`Mul` for `&Matrix<T>`, bounds on impl blocks | Original |
| 0558 | [A Sparse Vector with `Index`](exercises/p0558_sparse_vector_index.md) | Medium | `Index` returning a reference to a `static`, `IndexMut` that inserts, `BTreeMap<usize, f64>`, `'static` references | Original |
| 0559 | [A Sorted Vec with `Deref`](exercises/p0559_sorted_vec_deref.md) | Medium | `Deref<Target = [T]>` on a newtype, deref coercion, why not `DerefMut`, inherent methods shadowing slice methods, `FromIterator` | Original |
| 0560 | [Design Bitset](exercises/p0560_design_bitset.md) | Medium | design struct, lazy flip with an XOR flag, cached counts, an inherent `to_string` vs the `Display` trait | [LC 2166](https://leetcode.com/problems/design-bitset/) |
| 0561 | [The Longer of Two Strings](exercises/p0561_longest_of_two.md) | Easy | lifetime parameters in function signatures, `'a` relating inputs and outputs, outputs tied to only one input | Original |
| 0562 | [Lifetime Elision Rules](exercises/p0562_elision_rules.md) | Easy | the three lifetime elision rules, when explicit lifetimes are required, returning sub-slices, methods returning references tied to `&self` | Original |
| 0563 | [Uncommon Words from Two Sentences](exercises/p0563_uncommon_words_from_two_sentences.md) | Easy | `HashMap<&str, usize>` whose keys borrow from local `String`s, `chain` of two `split_whitespace` iterators | [LC 884](https://leetcode.com/problems/uncommon-words-from-two-sentences/) |
| 0564 | [Count Common Words With One Occurrence](exercises/p0564_count_common_words_with_one_occurrence.md) | Easy | `HashMap<&str, (u32, u32)>` with keys borrowed from two vectors, `String::as_str`, counting with tuples | [LC 2085](https://leetcode.com/problems/count-common-words-with-one-occurrence/) |
| 0565 | [A Struct Holding `&str`](exercises/p0565_struct_holding_str.md) | Easy | structs with a lifetime parameter, `impl<'a> Excerpt<'a>`, returning `&'a str` vs elided `&str`, constructors that borrow | Original |
| 0566 | [The `'static` Lifetime](exercises/p0566_static_lifetime.md) | Easy | `&'static str` from literals and `static` tables, structs storing `&'static str`, the `T: 'static` bound with `std::thread::spawn` | Original |
| 0567 | [Rearrange Words in a Sentence](exercises/p0567_rearrange_words_in_a_sentence.md) | Medium | `Vec<&str>` borrowing from a local `String`, stable `sort_by_key`, capitalising the first character | [LC 1451](https://leetcode.com/problems/rearrange-words-in-a-sentence/) |
| 0568 | [Subdomain Visit Count](exercises/p0568_subdomain_visit_count.md) | Medium | `HashMap<&str, i32>` keyed by sub-slices of the input, `split_once`, `match_indices`, borrowing for the whole function | [LC 811](https://leetcode.com/problems/subdomain-visit-count/) |
| 0569 | [Sentence Similarity III](exercises/p0569_sentence_similarity_iii.md) | Medium | `Vec<&str>` word slices, comparing slices with `==`, common prefix/suffix with `zip` and `take_while` | [LC 1813](https://leetcode.com/problems/sentence-similarity-iii/) |
| 0570 | [Config Lookups That Outlive the Borrow](exercises/p0570_config_lookup_lifetimes.md) | Medium | a struct holding `&'a str` pairs, methods returning `&'a str` instead of `&self`-bound references, lifetime bounds `'a: 'b` | Original |
| 0571 | [A Matcher with Two Lifetimes](exercises/p0571_two_lifetime_matcher.md) | Medium | a struct with lifetime `'p` and methods with their own lifetime `'t`, results tied to an argument instead of `self` | Original |
| 0572 | [Slice Views and `T: 'a`](exercises/p0572_slice_view_outlives.md) | Medium | `struct View<'a, T: 'a>`, outlives bounds, returning `View<'a, T>` and `&'a T` from methods, manual `Clone`/`Copy` without `T: Clone` | Original |
| 0573 | [A Borrowing Tokenizer](exercises/p0573_borrowed_tokenizer.md) | Medium | `enum Token<'a>` holding `&'a str`, `struct Tokenizer<'a>`, `impl<'a> Iterator for Tokenizer<'a>` with `type Item = Token<'a>`, byte positions and `char` boundaries | Original |
| 0574 | [A Fibonacci Iterator](exercises/p0574_fibonacci_iterator.md) | Easy | implementing `Iterator` (`type Item`, `fn next`), ending an infinite sequence on overflow with `checked_add`, `ExactSizeIterator`, adapters for free | Original |
| 0575 | [A Generic Step Range](exercises/p0575_step_range_iterator.md) | Easy | a generic iterator `StepRange<T>`, `impl<T> Iterator for StepRange<T> where T: Copy + PartialOrd + Default + Add + Sub`, avoiding overflow generically | Original |
| 0576 | [`IntoIterator` for a Collection](exercises/p0576_into_iterator_for_bag.md) | Medium | `IntoIterator` for `Bag<T>`, `&Bag<T>` and `&mut Bag<T>`, associated types `Item`/`IntoIter`, `FromIterator`, `Extend` | Original |
| 0577 | [A Run-Length Decoding Iterator](exercises/p0577_run_length_iterator.md) | Medium | an iterator borrowing a slice (`RunLengthDecode<'a, T>` yielding `&'a T`), overriding provided methods (`nth`, `size_hint`), generic encoding with `T: PartialEq + Clone` | Original |
| 0578 | [Flatten Nested List Iterator](exercises/p0578_flatten_nested_list_iterator.md) | Medium | recursive enum `NestedInteger`, a stack of `std::vec::IntoIter`, lazy iteration with a peeked value, `has_next(&self)` | [LC 341](https://leetcode.com/problems/flatten-nested-list-iterator/) |
| 0579 | [A Generic Combinations Iterator](exercises/p0579_combinations_iterator.md) | Medium | an iterator borrowing `&'a [T]` and yielding `Vec<&'a T>`, index-vector state machine, no `Clone` bound on `T` | Original |
| 0580 | [An Iterator Adapter and Extension Trait](exercises/p0580_iterator_adapter_extension.md) | Medium | adapter structs generic over an inner iterator, `I::Item` in bounds, an extension trait with a blanket `impl<I: Iterator> IterExt for I {}` | Original |
| 0581 | [Shapes as Trait Objects](exercises/p0581_shapes_trait_objects.md) | Easy | `dyn Trait`, `Vec<Box<dyn Shape>>` for heterogeneous collections, `&dyn Shape`, static vs dynamic dispatch | Original |
| 0582 | [Drawing a Screen of Components](exercises/p0582_gui_screen_draw.md) | Easy | `Vec<Box<dyn Draw>>`, open extension by library users, generic `add<C: Draw + 'static>` that boxes, `Box<dyn Draw>` means `Box<dyn Draw + 'static>` | Original |
| 0583 | [A Plugin Registry](exercises/p0583_plugin_registry.md) | Medium | `Box<dyn Plugin>` with `&mut self` methods, stateful trait objects, returning `Option<&mut dyn Plugin>`, removing boxed objects | Original |
| 0584 | [A Text Pipeline](exercises/p0584_text_pipeline_closures.md) | Medium | `Vec<Box<dyn Transform>>`, a blanket impl `impl<F: Fn(&str) -> String> Transform for F`, default trait methods, builder methods taking `self` | Original |
| 0585 | [Blog Post State Pattern](exercises/p0585_blog_post_state_pattern.md) | Medium | the state pattern with `Option<Box<dyn State>>`, methods taking `self: Box<Self>`, `Option::take`, default trait methods returning references | Original |
| 0586 | [Blog Post as Types](exercises/p0586_blog_post_type_state.md) | Medium | encoding states as distinct types, consuming transitions `fn request_review(self) -> PendingReviewPost`, enums for branching results, compile-time guarantees | Original |
| 0587 | [Undo and Redo with Commands](exercises/p0587_command_pattern_undo.md) | Medium | the command pattern with `Box<dyn Command>`, trait methods taking `&mut self` and another `&mut` argument, undo/redo stacks, moving boxes between vectors | Original |
| 0588 | [Dyn Compatibility and Cloning Boxes](exercises/p0588_dyn_compatibility_clone_box.md) | Medium | dyn-compatible traits (formerly "object safety"), `where Self: Sized` methods, cloning `Box<dyn Trait>` via a helper supertrait and a blanket impl | Original |
| 0589 | [Downcasting with `Any`](exercises/p0589_downcast_with_any.md) | Medium | `std::any::Any`, `downcast_ref`/`downcast_mut`/`Box::downcast`, `TypeId`, trait upcasting `&dyn Component` to `&dyn Any` (Rust 1.86+), the `Box::type_id` pitfall | Original |
| 0590 | [Fully Qualified Syntax](exercises/p0590_fully_qualified_syntax.md) | Easy | methods with the same name in several traits, `Trait::method(&value)`, `<Type as Trait>::function()` for associated functions without `self` | Original |
| 0591 | [Supertraits: Outline Printing](exercises/p0591_supertrait_outline_print.md) | Easy | supertraits (`trait OutlinePrint: fmt::Display`), default methods that use the supertrait, implementing a trait with an empty impl block | Original |
| 0592 | [Newtype for an External Trait](exercises/p0592_newtype_external_trait.md) | Easy | the orphan rule, wrapping a foreign type (`Vec<String>`, `Vec<u8>`) in a local newtype to implement foreign traits (`Display`, `FromStr`), `Deref` to keep the inner API | Original |
| 0593 | [Associated Constants](exercises/p0593_associated_consts.md) | Easy | `const` items in traits, default constant values, associated functions using `Self::CONST`, generic functions reading `U::SYMBOL` | Original |
| 0594 | [Associated Types](exercises/p0594_associated_types_container.md) | Medium | `type Item` in a trait, `Self::Item` in signatures, bounds on associated types (`C::Item: Display`), equality constraints (`C: Container<Item = i64>`) | Original |
| 0595 | [Generic Traits vs Associated Types](exercises/p0595_generic_trait_vs_associated_type.md) | Medium | a generic trait `ConvertTo<T>` implemented several times for one type, a trait with `type Output` implemented once, type annotations and fully qualified calls to pick an impl | Original |
| 0596 | [Blanket Implementations](exercises/p0596_blanket_impls.md) | Medium | `impl<T: Display + ?Sized> Trait for T`, blanket impls for `&T` and `Box<T>`, `?Sized` so that `str` and `dyn Trait` are covered, coherence | Original |
| 0597 | [Type Aliases, `!` and `?Sized`](exercises/p0597_type_aliases_never_sized.md) | Medium | type aliases (plain and generic), the never type `!` (`continue`, `panic!`, diverging functions), dynamically sized types and `T: ?Sized`, `size_of_val` | Original |
| 0598 | [A Generic State Machine](exercises/p0598_state_machine_associated_types.md) | Medium | a trait combining associated types (`State`, `Input`) with bounds and an associated constant (`START`), generic runners over `M: StateMachine`, `M: StateMachine<Input = char>` | Original |
| 0599 | [Units of Measure](exercises/p0599_units_of_measure_phantom.md) | Hard | `Length<U>` with `PhantomData<U>`, unit marker types with associated consts, operators that only accept the **same** unit, generic conversion `to::<V>()`, manual trait impls without bounds on `U` | Original |
| 0600 | [A Const-Generic Matrix](exercises/p0600_const_generic_matrix.md) | Hard | const generics (`Matrix<const R: usize, const C: usize>`), dimension-checked `Mul<Matrix<K, C>> for Matrix<R, K>`, impl blocks for square matrices only (`impl<const N: usize> Matrix<N, N>`), fast exponentiation | Original |

<!-- PROBLEM_TABLE:END -->
