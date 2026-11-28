# Level 10: Advanced (problems 0901-1000)

> Focus: hard problems, advanced data structures, macros, `unsafe`, async, advanced traits.

This last level has three parts. Problems 0901-0944 are original exercises on the advanced corners of
the language: macros, module privacy, clone-on-write, iterator and const-generic APIs, typestate,
sealed traits, `Any`, lazy statics, higher-ranked bounds, `unsafe` code and async Rust built from
scratch with the standard library only. Problems 0945-0961 implement advanced data structures (tries,
Fenwick and segment trees, interval maps, LFU caches, binary lifting), and 0962-1000 are classic
LeetCode Hard problems. 56 problems come from LeetCode (53 Hard and 3 Medium, all with LeetCode's
exact Rust signatures); the other 44 are original.

## Prerequisites

- Levels 1-9, or the same knowledge: ownership and borrowing, traits and generics, lifetimes,
  `Box`/`Rc`/`RefCell`, iterators and closures, the algorithm techniques of level 8 and the dynamic
  programming and graph algorithms of level 9.
- Read [The Rust Book 20.5: Macros](https://doc.rust-lang.org/book/ch20-05-macros.html) before 0904,
  [20.1: Unsafe Rust](https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html) before 0931 and
  [chapter 17: Async and Await](https://doc.rust-lang.org/book/ch17-00-async-await.html) before 0938.
  [The Rustonomicon](https://doc.rust-lang.org/nomicon/index.html) is the companion for the `unsafe`
  problems.
- Tree problems use LeetCode's `TreeNode`, defined (with test helpers) in `src/common.rs`, exactly as
  in level 7.

## What you will practice

- Modules and visibility inside one file: `mod`, `self`/`super` paths, `pub(crate)`, `pub(super)`,
  private fields guarding invariants and `pub use` facades:
  [Book 7.2](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html),
  [Reference: visibility](https://doc.rust-lang.org/reference/visibility-and-privacy.html).
- Declarative macros: recursion, repetitions, counting, collection literals, nested repetitions,
  early-return macros and a recursive `json!` built on token trees:
  [Book 20.5](https://doc.rust-lang.org/book/ch20-05-macros.html),
  [Reference: macros by example](https://doc.rust-lang.org/reference/macros-by-example.html).
- [`Cow`](https://doc.rust-lang.org/std/borrow/enum.Cow.html): borrowed results when nothing changes,
  `to_mut` and clone-on-write slices.
- Iterator traits for your own types:
  [`DoubleEndedIterator`](https://doc.rust-lang.org/std/iter/trait.DoubleEndedIterator.html),
  [`ExactSizeIterator`](https://doc.rust-lang.org/std/iter/trait.ExactSizeIterator.html),
  [`FusedIterator`](https://doc.rust-lang.org/std/iter/trait.FusedIterator.html),
  [`FromIterator`](https://doc.rust-lang.org/std/iter/trait.FromIterator.html),
  [`Extend`](https://doc.rust-lang.org/std/iter/trait.Extend.html), and a lending iterator with
  generic associated types.
- Const generics and `const fn`: fixed-capacity stacks, `Grid<W, H>`, compile-time lookup tables:
  [Reference: generics](https://doc.rust-lang.org/reference/items/generics.html),
  [Reference: constant evaluation](https://doc.rust-lang.org/reference/const_eval.html).
- Type-level design: [`PhantomData`](https://doc.rust-lang.org/std/marker/struct.PhantomData.html)
  typestate machines and builders, sealed traits, [`Any`](https://doc.rust-lang.org/std/any/index.html)
  and `TypeId` for panic payloads and a typed event bus.
- Lazy initialisation: [`OnceCell`](https://doc.rust-lang.org/std/cell/struct.OnceCell.html),
  [`OnceLock`](https://doc.rust-lang.org/std/sync/struct.OnceLock.html) and
  [`LazyLock`](https://doc.rust-lang.org/std/sync/struct.LazyLock.html); higher-ranked trait bounds
  `for<'a>` ([Nomicon: HRTB](https://doc.rust-lang.org/nomicon/hrtb.html)) and
  [`ControlFlow`](https://doc.rust-lang.org/std/ops/enum.ControlFlow.html).
- `unsafe` Rust: raw pointers, `unsafe fn` contracts, a safe `split_at_mut`, `std::ptr` moves,
  unsafe traits, `MaybeUninit`, a raw-pointer iterator and a linked queue with a raw tail:
  [Book 20.1](https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html),
  [`std::ptr`](https://doc.rust-lang.org/std/ptr/index.html),
  [`MaybeUninit`](https://doc.rust-lang.org/std/mem/union.MaybeUninit.html).
- Async without crates: implementing [`Future`](https://doc.rust-lang.org/std/future/trait.Future.html)
  by hand, [`Pin`](https://doc.rust-lang.org/std/pin/index.html), wakers via
  [`Wake`](https://doc.rust-lang.org/std/task/trait.Wake.html), `block_on`, `join`, `select`,
  `async fn`, a oneshot channel and a task executor with `spawn`.
- Advanced data structures and Hard algorithms: tries, Fenwick and segment trees, interval sets in
  [`BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html), two heaps with
  [`BinaryHeap`](https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html) and
  [`Reverse`](https://doc.rust-lang.org/std/cmp/struct.Reverse.html), monotonic stacks and
  [`VecDeque`](https://doc.rust-lang.org/std/collections/struct.VecDeque.html) windows, binary search
  on the answer, greedy proofs, backtracking with bitmasks, interval and string DP, and graph
  searches on grids and routes.

## Roadmap

| Problems | Theme |
| -------- | ----- |
| 0901-0903 | Modules and visibility: paths, `pub(crate)`/`pub(super)`, invariants behind private fields, `pub use` facades |
| 0904-0910 | `macro_rules!`: recursion, counting, `hashmap!`, nested repetitions, `ensure!`/`bail!`, `compose!`, `json!` |
| 0911-0913 | `Cow`: borrowed or owned strings, lazy unescaping, clone-on-write slices |
| 0914-0916 | Iterator traits: a double-ended ring buffer, `FromIterator`/`Extend` for a multimap, a GAT lending iterator |
| 0917-0919 | Const generics and `const fn`: a fixed-capacity stack, Game of Life on `Grid<W, H>`, compile-time tables |
| 0920-0925 | Type-level design: typestate with `PhantomData`, a typestate builder, sealed traits, `Any` payloads, a typed event bus |
| 0926-0930 | `OnceCell`, `LazyLock`/`OnceLock`, higher-ranked closures and trait bounds, `ControlFlow` |
| 0931-0937 | `unsafe`: raw pointers, safe `split_at_mut`, `std::ptr` insert/remove, unsafe traits, raw iterators, `MaybeUninit`, a raw-tail queue |
| 0938-0944 | Async from scratch: hand-written futures, `block_on`, `join`, `select`, `async fn`, a oneshot channel, a task executor |
| 0945-0953 | Tries, Fenwick trees and segment trees: Trie, wildcard search, range sums, inversion-style counting, falling squares, LIS II, stream matching |
| 0954-0961 | Design problems: median finder, interval sets and range modules, calendars, frequency stack, LFU cache, kth ancestor |
| 0962-0971 | Arrays, stacks and windows: first missing positive, candy, parentheses, histograms, sliding window maximum, minimum window, deques |
| 0972-0980 | Binary search on the answer and greedy with heaps: median of two arrays, split array, k-th smallest values, IPO, schedules, refuelling, ranges, skyline |
| 0981-0988 | Strings and backtracking: text justification, number words, KMP, N-Queens, Sudoku, expression operators, word search with a trie |
| 0989-0995 | Dynamic programming: regex and wildcard matching, scramble strings, palindrome cuts, stock trading, dungeon, Russian dolls |
| 0996-1000 | Trees and graphs: recovering a tree, binary tree cameras, increasing paths, 3D rain water, bus routes |

## How to work on a problem

```bash
cd learn_11_leetcode_practice/level_10_advanced

cargo test practice::p0901      # run the tests for the problem you are solving
cargo test solutions::p0901     # run the tests against the reference solution
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
| 0901 | [Modules, Paths and `pub(crate)`](exercises/p0901_module_tree_and_paths.md) | Medium | inline `mod` blocks, `self`/`super` paths, `pub(crate)`, `pub(super)`, private items | Original |
| 0902 | [Invariants Behind Private Fields](exercises/p0902_invariants_behind_private_fields.md) | Medium | private fields, validated constructors, getters, `pub(crate)`, all-or-nothing updates | Original |
| 0903 | [A Facade with `pub use`](exercises/p0903_facade_with_pub_use.md) | Medium | private modules, `pub use` re-exports, `use ... as`, a `prelude` module, `dyn Trait` | Original |
| 0904 | [Recursive `max!` and `min!` Macros](exercises/p0904_recursive_max_min_macros.md) | Medium | `macro_rules!`, `$x:expr` fragments, repetitions `$(...),+`, recursive macros, trailing commas | Original |
| 0905 | [Counting Tokens with `count!`](exercises/p0905_counting_macro.md) | Medium | `macro_rules!`, `tt` fragments, helper macros, recursion limit, constant expressions | Original |
| 0906 | [Collection Literals: `hashmap!` and `hashset!`](exercises/p0906_hashmap_literal_macro.md) | Medium | `macro_rules!` repetitions, `$k:expr => $v:expr`, absolute paths `::std::...`, pre-allocation | Original |
| 0907 | [Nested Repetitions: `matrix!`](exercises/p0907_matrix_literal_macro.md) | Medium | nested `$( $( ... ),+ );+` repetitions, separators, runtime checks in expansions | Original |
| 0908 | [Early Returns: `ensure!` and `bail!`](exercises/p0908_ensure_and_bail_macros.md) | Medium | macros that `return`, forwarding format arguments with `$($arg:tt)+`, `stringify!`, `From` | Original |
| 0909 | [Function Composition with `compose!`](exercises/p0909_compose_macro.md) | Medium | recursive macros that build closures, `move` closures, `=>` separators, closure type inference | Original |
| 0910 | [A `json!` Macro](exercises/p0910_json_literal_macro.md) | Hard | recursive `macro_rules!` over token trees, `$key:literal`, `From` conversions, `Display` | Original |
| 0911 | [Normalize Whitespace with `Cow<str>`](exercises/p0911_cow_normalize_whitespace.md) | Medium | `std::borrow::Cow`, `Cow::Borrowed` vs `Cow::Owned`, avoiding allocations, lifetimes in return types | Original |
| 0912 | [Unescape a String Literal Lazily](exercises/p0912_cow_unescape_string_literal.md) | Medium | `Cow<'a, str>` in `Result` and `Vec`, borrowed sub-slices, `char::from_u32`, `Peekable<Chars>` | Original |
| 0913 | [Clone-on-Write Slices with `to_mut`](exercises/p0913_cow_clamp_slice.md) | Medium | `Cow<'a, [T]>`, `Cow::to_mut`, `ToOwned`, `Into<Cow<'static, str>>`, counting clones | Original |
| 0914 | [A Ring Buffer with Double-Ended Iterators](exercises/p0914_ring_buffer_iterators.md) | Hard | `DoubleEndedIterator`, `ExactSizeIterator`, `FusedIterator`, `IntoIterator` for `T` and `&T`, modular indexing | Original |
| 0915 | [`FromIterator` and `Extend` for a MultiMap](exercises/p0915_multimap_from_iterator_extend.md) | Hard | `FromIterator`, `Extend` (owned and `&`-items), `IntoIterator` for `T`/`&T`, `Borrow`, naming iterator types with `fn` pointers | Original |
| 0916 | [A Lending Iterator with GATs](exercises/p0916_lending_iterator_gat.md) | Hard | generic associated types (`type Item<'a> where Self: 'a`), lending vs. ordinary iterators, `&mut [T]` windows, reusable buffers | Original |
| 0917 | [A Fixed-Capacity Stack](exercises/p0917_const_generic_array_stack.md) | Medium | const generics `<T, const N: usize>`, `[Option<T>; N]`, `std::array::from_fn`, associated consts, `Result<(), T>` to give a value back | Original |
| 0918 | [Game of Life on a `Grid<W, H>`](exercises/p0918_const_generic_life_grid.md) | Medium | two const generic parameters, `[[bool; W]; H]`, arrays as parameters (`[&str; H]`), `Copy` types, `Display` | Original |
| 0919 | [Compile-Time Tables with `const fn`](exercises/p0919_const_fn_lookup_tables.md) | Medium | `const fn`, `while` loops in const context, `const` items computed at compile time, const generic return arrays, CRC tables | Original |
| 0920 | [A Typestate Connection](exercises/p0920_typestate_connection.md) | Hard | `PhantomData<S>`, zero-sized marker types, typestate, a sealed `State` trait, generic methods that change the type parameter | Original |
| 0921 | [A Builder That Enforces Required Fields](exercises/p0921_typestate_builder.md) | Hard | builder pattern, typestate with two type parameters, `PhantomData<(U, M)>`, partially generic `impl` blocks, validation in `build` | Original |
| 0922 | [A Sealed `Integer` Trait](exercises/p0922_sealed_integer_trait.md) | Medium | sealed traits, associated constants, implementing a trait for many types with a macro, generic numeric algorithms, `try_fold` | Original |
| 0923 | [A Sealed Extension Trait](exercises/p0923_sealed_extension_trait.md) | Medium | extension traits, implementing a trait for `str` (an unsized type), sealing, `?Sized` bounds, delegating impls | Original |
| 0924 | [Reading Panic Payloads with `dyn Any`](exercises/p0924_panic_payload_any.md) | Medium | `std::any::Any`, `downcast_ref`, `Box<dyn Any + Send>`, `std::panic::catch_unwind`, `panic_any`, `UnwindSafe` | Original |
| 0925 | [A Typed Event Bus](exercises/p0925_typed_event_bus.md) | Hard | `TypeId`, `HashMap<TypeId, Vec<Box<dyn Fn(&dyn Any)>>>`, type erasure, generic methods with `'static` bounds, closures wrapping closures | Original |
| 0926 | [Memoized Fields with `OnceCell`](exercises/p0926_oncecell_memoized_fields.md) | Medium | `std::cell::OnceCell`, `get_or_init`, `set`/`get`, resetting caches with `take`, `Cell` counters, returning references into a cache | Original |
| 0927 | [Global Tables with `LazyLock` and `OnceLock`](exercises/p0927_lazylock_static_tables.md) | Medium | `std::sync::LazyLock` statics, `std::sync::OnceLock`, thread-safe one-time initialisation, `AtomicUsize`, `&'static` results | Original |
| 0928 | [Higher-Ranked Closures](exercises/p0928_hrtb_str_pipeline.md) | Hard | higher-ranked trait bounds `for<'a> Fn(&'a str) -> &'a str`, boxed HRTB closures, lifetime elision in `Fn` bounds, borrowing from locals | Original |
| 0929 | [Higher-Ranked Bounds on Your Own Trait](exercises/p0929_hrtb_trait_bounds.md) | Hard | traits with a lifetime parameter, `E: for<'a> Extract<'a>`, blanket impls for closures, closure signature inference | Original |
| 0930 | [Short-Circuiting with `ControlFlow`](exercises/p0930_control_flow_try_fold.md) | Medium | `std::ops::ControlFlow`, `try_fold`, `try_for_each`, the `?` operator on `ControlFlow`, early exit from recursion | Original |
| 0931 | [Raw Pointers and `unsafe fn`](exercises/p0931_raw_pointer_basics.md) | Medium | `*const T` / `*mut T`, `unsafe` blocks, `unsafe fn` with a `# Safety` contract, `ptr.add`, `offset_from`, `ptr::swap`, safe wrappers | Original |
| 0932 | [A Safe `split_at_mut`](exercises/p0932_safe_split_at_mut.md) | Hard | `std::slice::from_raw_parts_mut`, building safe abstractions over `unsafe`, disjoint `&mut` borrows, lifetimes of returned borrows | Original |
| 0933 | [Vec Insert and Remove with `std::ptr`](exercises/p0933_ptr_copy_insert_remove.md) | Hard | `ptr::copy` (memmove), `ptr::copy_nonoverlapping`, `ptr::read` / `ptr::write`, `Vec::set_len`, `reserve`, moving non-`Copy` values without double drops | Original |
| 0934 | [Unsafe Traits: `Zeroable`](exercises/p0934_unsafe_trait_zeroable.md) | Hard | `unsafe trait` and `unsafe impl`, `std::mem::zeroed`, `#[repr(C)]`, invariants that generic safe code relies on, `Option<Box<T>>` niche | Original |
| 0935 | [A Raw-Pointer Slice Iterator](exercises/p0935_raw_slice_iter_phantom.md) | Hard | `PhantomData<&'a T>` and `PhantomData<&'a mut T>`, start/end pointer iteration, `offset_from`, `DoubleEndedIterator`, manual `Clone` without a `T: Clone` bound | Original |
| 0936 | [A `StackVec` with `MaybeUninit`](exercises/p0936_maybe_uninit_stack_vec.md) | Hard | `std::mem::MaybeUninit`, `[const { MaybeUninit::uninit() }; N]`, `assume_init_read`, `ptr::drop_in_place`, `slice::from_raw_parts`, a correct `Drop` | Original |
| 0937 | [A Queue with a Raw Tail Pointer](exercises/p0937_raw_tail_queue.md) | Hard | `Box::into_raw` / `Box::from_raw`, `*mut Node<T>`, `ptr::null_mut`, ownership through raw pointers, `PhantomData<Box<Node<T>>>`, iterative `Drop` | Original |
| 0938 | [Implement `Future` by Hand](exercises/p0938_hand_written_future.md) | Hard | the `Future` trait, `Poll`, `Pin<&mut Self>`, `Context` and `Waker`, `Unpin`, `std::pin::pin!`, `Waker::noop` | Original |
| 0939 | [A `block_on` Executor](exercises/p0939_waker_and_block_on.md) | Hard | `std::task::Wake`, `Waker::from(Arc<W>)`, `thread::park` / `Thread::unpark`, a minimal executor, futures woken from another thread | Original |
| 0940 | [Join Two Futures](exercises/p0940_join_two_futures.md) | Hard | combinator futures, `Pin<Box<F>>`, a `MaybeDone` state enum, manual `Unpin` impls, concurrency without threads | Original |
| 0941 | [Race Two Futures](exercises/p0941_select_two_futures.md) | Hard | `select`-style combinators, an `Either` output, cancellation by `Drop`, `async fn` built on a hand-written combinator | Original |
| 0942 | [`async fn` and `.await` on Your Executor](exercises/p0942_async_fn_with_yield.md) | Hard | `async fn`, `.await`, laziness of futures, borrowing parameters in `async fn`, recursive `async fn` with `Box::pin`, counting polls | Original |
| 0943 | [A Oneshot Channel Future](exercises/p0943_oneshot_channel_future.md) | Hard | a future woken by another thread, storing a `Waker` in shared state, `Arc<Mutex<..>>`, `Drop` signalling cancellation, `Send` futures | Original |
| 0944 | [A Task Executor with `spawn`](exercises/p0944_task_executor_spawn.md) | Hard | tasks as `Arc<Task>`, a run queue over `std::sync::mpsc`, `Wake` that reschedules, `Pin<Box<dyn Future + Send>>`, spawning from inside tasks, termination | Original |
| 0945 | [Implement Trie (Prefix Tree)](exercises/p0945_implement_trie_prefix_tree.md) | Medium | tries, `[Option<Box<Node>>; 26]`, `Default` for arrays of `Option`, walking a tree with `&` and `&mut` references | [LC 208](https://leetcode.com/problems/implement-trie-prefix-tree/) |
| 0946 | [Design Add and Search Words Data Structure](exercises/p0946_design_add_and_search_words_data_structure.md) | Medium | tries, wildcard search with recursive DFS over `&Node`, byte slices, `Iterator::any` | [LC 211](https://leetcode.com/problems/design-add-and-search-words-data-structure/) |
| 0947 | [Range Sum Query - Mutable](exercises/p0947_range_sum_query_mutable.md) | Medium | Fenwick tree (binary indexed tree), lowest set bit `i & i.wrapping_neg()`, prefix sums under updates | [LC 307](https://leetcode.com/problems/range-sum-query-mutable/) |
| 0948 | [Count of Smaller Numbers After Self](exercises/p0948_count_of_smaller_numbers_after_self.md) | Hard | Fenwick tree over values, coordinate compression with `sort_unstable` + `dedup` + `binary_search`, scanning from the right | [LC 315](https://leetcode.com/problems/count-of-smaller-numbers-after-self/) |
| 0949 | [Reverse Pairs](exercises/p0949_reverse_pairs.md) | Hard | divide and conquer, merge sort that counts, `i64` to avoid overflow, `split_at_mut`, two pointers on sorted halves | [LC 493](https://leetcode.com/problems/reverse-pairs/) |
| 0950 | [Count of Range Sum](exercises/p0950_count_of_range_sum.md) | Hard | prefix sums in `i64`, merge sort with a sliding window over the right half, `slice::sort` of merged halves | [LC 327](https://leetcode.com/problems/count-of-range-sum/) |
| 0951 | [Falling Squares](exercises/p0951_falling_squares.md) | Hard | segment tree with lazy range assignment, coordinate compression, half-open intervals, recursion on `(node, lo, hi)` | [LC 699](https://leetcode.com/problems/falling-squares/) |
| 0952 | [Longest Increasing Subsequence II](exercises/p0952_longest_increasing_subsequence_ii.md) | Hard | DP over values, iterative (bottom-up) segment tree for range maximum, point updates, half-open query ranges | [LC 2407](https://leetcode.com/problems/longest-increasing-subsequence-ii/) |
| 0953 | [Stream of Characters](exercises/p0953_stream_of_characters.md) | Hard | a trie of **reversed** words, an arena of nodes in a `Vec` with `u32` child indices, bounded history buffer | [LC 1032](https://leetcode.com/problems/stream-of-characters/) |
| 0954 | [Find Median from Data Stream](exercises/p0954_find_median_from_data_stream.md) | Hard | two heaps, `BinaryHeap` as a max-heap and with `std::cmp::Reverse` as a min-heap, invariants between structures, `f64` results | [LC 295](https://leetcode.com/problems/find-median-from-data-stream/) |
| 0955 | [Data Stream as Disjoint Intervals](exercises/p0955_data_stream_as_disjoint_intervals.md) | Hard | `BTreeMap` as an interval set, `range(..=x).next_back()` to find the predecessor, merging neighbours, `remove` + `insert` | [LC 352](https://leetcode.com/problems/data-stream-as-disjoint-intervals/) |
| 0956 | [Range Module](exercises/p0956_range_module.md) | Hard | half-open intervals in a `BTreeMap`, merging on insert, splitting on removal, collecting keys before mutating | [LC 715](https://leetcode.com/problems/range-module/) |
| 0957 | [Count Integers in Intervals](exercises/p0957_count_integers_in_intervals.md) | Hard | closed intervals in a `BTreeMap`, amortised merging with `while let`, maintaining a running total | [LC 2276](https://leetcode.com/problems/count-integers-in-intervals/) |
| 0958 | [My Calendar III](exercises/p0958_my_calendar_iii.md) | Hard | sweep line over a `BTreeMap` of boundary deltas, `entry().or_insert(0)`, `scan` for running sums | [LC 732](https://leetcode.com/problems/my-calendar-iii/) |
| 0959 | [Maximum Frequency Stack](exercises/p0959_maximum_frequency_stack.md) | Hard | one stack per frequency (`Vec<Vec<i32>>`), `HashMap` counters, `entry` API, amortised `O(1)` operations | [LC 895](https://leetcode.com/problems/maximum-frequency-stack/) |
| 0960 | [LFU Cache](exercises/p0960_lfu_cache.md) | Hard | `HashMap` + `BTreeSet<(freq, tick, key)>` as an ordered eviction index, `pop_first`, logical clocks, keeping two structures in sync | [LC 460](https://leetcode.com/problems/lfu-cache/) |
| 0961 | [Kth Ancestor of a Tree Node](exercises/p0961_kth_ancestor_of_a_tree_node.md) | Hard | binary lifting (jump tables of `2^j` ancestors), `Vec<Vec<i32>>` tables, bit iteration of `k` | [LC 1483](https://leetcode.com/problems/kth-ancestor-of-a-tree-node/) |
| 0962 | [First Missing Positive](exercises/p0962_first_missing_positive.md) | Hard | in-place cyclic placement with `swap`, using the array as its own hash table, `O(1)` extra space | [LC 41](https://leetcode.com/problems/first-missing-positive/) |
| 0963 | [Candy](exercises/p0963_candy.md) | Hard | two-pass greedy (left-to-right, then right-to-left), `windows(2)`-style neighbour comparisons, `max` | [LC 135](https://leetcode.com/problems/candy/) |
| 0964 | [Longest Valid Parentheses](exercises/p0964_longest_valid_parentheses.md) | Hard | stack of indices with a sentinel base, byte iteration, `Vec<isize>` to allow `-1` | [LC 32](https://leetcode.com/problems/longest-valid-parentheses/) |
| 0965 | [Largest Rectangle in Histogram](exercises/p0965_largest_rectangle_in_histogram.md) | Hard | monotonic stack of indices, a sentinel bar of height 0, `chain(once(0))`, widths from neighbouring stack entries | [LC 84](https://leetcode.com/problems/largest-rectangle-in-histogram/) |
| 0966 | [Maximal Rectangle](exercises/p0966_maximal_rectangle.md) | Hard | reducing a 2D problem to row histograms, monotonic stack, `Vec<Vec<char>>` input, reusing a helper | [LC 85](https://leetcode.com/problems/maximal-rectangle/) |
| 0967 | [Sliding Window Maximum](exercises/p0967_sliding_window_maximum.md) | Hard | monotonic `VecDeque` of indices, `front`/`back`/`pop_front`/`pop_back`, amortised analysis | [LC 239](https://leetcode.com/problems/sliding-window-maximum/) |
| 0968 | [Minimum Window Substring](exercises/p0968_minimum_window_substring.md) | Hard | variable-size sliding window, byte-indexed count arrays `[i32; 128]`, a "missing" counter, slicing the answer | [LC 76](https://leetcode.com/problems/minimum-window-substring/) |
| 0969 | [Substring with Concatenation of All Words](exercises/p0969_substring_with_concatenation_of_all_words.md) | Hard | sliding window measured in words, one pass per offset `0..word_len`, `HashMap<&str, usize>` with borrowed keys | [LC 30](https://leetcode.com/problems/substring-with-concatenation-of-all-words/) |
| 0970 | [Subarrays with K Different Integers](exercises/p0970_subarrays_with_k_different_integers.md) | Hard | "exactly k = at most k - at most (k - 1)", sliding window with a count array, counting subarrays by their right end | [LC 992](https://leetcode.com/problems/subarrays-with-k-different-integers/) |
| 0971 | [Shortest Subarray with Sum at Least K](exercises/p0971_shortest_subarray_with_sum_at_least_k.md) | Hard | prefix sums in `i64`, monotonic deque of prefix indices, why a plain sliding window fails with negative numbers | [LC 862](https://leetcode.com/problems/shortest-subarray-with-sum-at-least-k/) |
| 0972 | [Median of Two Sorted Arrays](exercises/p0972_median_of_two_sorted_arrays.md) | Hard | binary search on a partition, sentinels with `i32::MIN`/`i32::MAX` via `Option` defaults, `std::mem::swap` of `Vec`s | [LC 4](https://leetcode.com/problems/median-of-two-sorted-arrays/) |
| 0973 | [Split Array Largest Sum](exercises/p0973_split_array_largest_sum.md) | Hard | binary search on the answer, a greedy feasibility check, monotone predicates, `i64` sums | [LC 410](https://leetcode.com/problems/split-array-largest-sum/) |
| 0974 | [Kth Smallest Number in Multiplication Table](exercises/p0974_kth_smallest_number_in_multiplication_table.md) | Hard | binary search on values, counting elements `<= x` row by row, `min`, implicit matrices | [LC 668](https://leetcode.com/problems/kth-smallest-number-in-multiplication-table/) |
| 0975 | [Find K-th Smallest Pair Distance](exercises/p0975_find_k_th_smallest_pair_distance.md) | Hard | sort + binary search on the distance, counting pairs with two pointers, `sort_unstable` | [LC 719](https://leetcode.com/problems/find-k-th-smallest-pair-distance/) |
| 0976 | [IPO](exercises/p0976_ipo.md) | Hard | greedy with a max-heap of affordable profits, sorting pairs by capital, `zip`, `BinaryHeap` | [LC 502](https://leetcode.com/problems/ipo/) |
| 0977 | [Course Schedule III](exercises/p0977_course_schedule_iii.md) | Hard | exchange-argument greedy, sorting by deadline with `sort_unstable_by_key`, a max-heap of durations to undo choices | [LC 630](https://leetcode.com/problems/course-schedule-iii/) |
| 0978 | [Minimum Number of Refueling Stops](exercises/p0978_minimum_number_of_refueling_stops.md) | Hard | lazy greedy ("refuel in hindsight"), a max-heap of passed stations, `i64` fuel, early exit | [LC 871](https://leetcode.com/problems/minimum-number-of-refueling-stops/) |
| 0979 | [Smallest Range Covering Elements from K Lists](exercises/p0979_smallest_range_covering_elements_from_k_lists.md) | Hard | k-way merge with a min-heap of `Reverse((value, list, index))`, tracking the current maximum, tie-breaking ranges | [LC 632](https://leetcode.com/problems/smallest-range-covering-elements-from-k-lists/) |
| 0980 | [The Skyline Problem](exercises/p0980_the_skyline_problem.md) | Hard | sweep line over sorted events, a multiset as `BTreeMap<height, count>`, grouping events by x, `chunk_by` | [LC 218](https://leetcode.com/problems/the-skyline-problem/) |
| 0981 | [Text Justification](exercises/p0981_text_justification.md) | Hard | greedy line packing, distributing spaces with `/` and `%`, `String::with_capacity`, `" ".repeat(n)` | [LC 68](https://leetcode.com/problems/text-justification/) |
| 0982 | [Integer to English Words](exercises/p0982_integer_to_english_words.md) | Hard | decomposition into groups of three digits, `const` word tables, a recursive helper below 1000, `Vec<&str>` + `join` | [LC 273](https://leetcode.com/problems/integer-to-english-words/) |
| 0983 | [Shortest Palindrome](exercises/p0983_shortest_palindrome.md) | Hard | KMP prefix function (failure function), longest palindromic prefix, building a combined byte string with a separator | [LC 214](https://leetcode.com/problems/shortest-palindrome/) |
| 0984 | [N-Queens](exercises/p0984_n_queens.md) | Hard | backtracking row by row, column and diagonal occupancy with `Vec<bool>`, building boards as `Vec<String>` | [LC 51](https://leetcode.com/problems/n-queens/) |
| 0985 | [N-Queens II](exercises/p0985_n_queens_ii.md) | Hard | bitmask backtracking, `u32` masks shifted per row, `trailing_zeros` and `x & x.wrapping_neg()` to pick bits | [LC 52](https://leetcode.com/problems/n-queens-ii/) |
| 0986 | [Sudoku Solver](exercises/p0986_sudoku_solver.md) | Hard | backtracking with bitmasks per row/column/box, choosing the most constrained cell, `&mut Vec<Vec<char>>`, `count_ones` | [LC 37](https://leetcode.com/problems/sudoku-solver/) |
| 0987 | [Expression Add Operators](exercises/p0987_expression_add_operators.md) | Hard | backtracking over digit splits, evaluating `*` precedence on the fly with "last operand", `i64`, a shared `String` buffer with `truncate` | [LC 282](https://leetcode.com/problems/expression-add-operators/) |
| 0988 | [Word Search II](exercises/p0988_word_search_ii.md) | Hard | trie + grid DFS, marking visited cells in place, pruning found words, arena-allocated trie nodes | [LC 212](https://leetcode.com/problems/word-search-ii/) |
| 0989 | [Regular Expression Matching](exercises/p0989_regular_expression_matching.md) | Hard | 2D dynamic programming over suffixes, `'.'` and `'*'` semantics, byte slices, `vec![vec![false; m + 1]; n + 1]` | [LC 10](https://leetcode.com/problems/regular-expression-matching/) |
| 0990 | [Wildcard Matching](exercises/p0990_wildcard_matching.md) | Hard | DP with a rolling 1D row, `'?'` and `'*'` semantics, `std::mem::swap` of rows, large inputs | [LC 44](https://leetcode.com/problems/wildcard-matching/) |
| 0991 | [Scramble String](exercises/p0991_scramble_string.md) | Hard | interval DP over (start1, start2, length), memoisation with a flat `Vec<Option<bool>>`, letter-count pruning | [LC 87](https://leetcode.com/problems/scramble-string/) |
| 0992 | [Palindrome Partitioning II](exercises/p0992_palindrome_partitioning_ii.md) | Hard | DP over prefixes combined with expand-around-center palindrome enumeration, `O(n)` memory | [LC 132](https://leetcode.com/problems/palindrome-partitioning-ii/) |
| 0993 | [Best Time to Buy and Sell Stock III](exercises/p0993_best_time_to_buy_and_sell_stock_iii.md) | Hard | state-machine DP with four running states, `fold` over prices, `i32::MIN / 2` style sentinels avoided by ordering updates | [LC 123](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/) |
| 0994 | [Dungeon Game](exercises/p0994_dungeon_game.md) | Hard | grid DP computed **backwards** from the goal, minimum required value, `max(1, ...)`, a padded DP table | [LC 174](https://leetcode.com/problems/dungeon-game/) |
| 0995 | [Russian Doll Envelopes](exercises/p0995_russian_doll_envelopes.md) | Hard | sorting by `(width asc, height desc)` with `sort_unstable_by`, LIS with patience sorting and `partition_point`, `Ordering::then` | [LC 354](https://leetcode.com/problems/russian-doll-envelopes/) |
| 0996 | [Recover a Tree From Preorder Traversal](exercises/p0996_recover_a_tree_from_preorder_traversal.md) | Hard | parsing with a byte cursor, a stack of `Rc<RefCell<TreeNode>>` indexed by depth, `Rc::clone`, `borrow_mut` | [LC 1028](https://leetcode.com/problems/recover-a-tree-from-preorder-traversal/) |
| 0997 | [Binary Tree Cameras](exercises/p0997_binary_tree_cameras.md) | Hard | greedy post-order tree DP with three states, an `enum` for node states, recursion over `Option<Rc<RefCell<TreeNode>>>` | [LC 968](https://leetcode.com/problems/binary-tree-cameras/) |
| 0998 | [Longest Increasing Path in a Matrix](exercises/p0998_longest_increasing_path_in_a_matrix.md) | Hard | DFS with memoisation on a DAG, or topological "peeling" with a `VecDeque` (used here to avoid deep recursion), out-degrees | [LC 329](https://leetcode.com/problems/longest-increasing-path-in-a-matrix/) |
| 0999 | [Trapping Rain Water II](exercises/p0999_trapping_rain_water_ii.md) | Hard | priority-first flood fill from the border, `BinaryHeap<Reverse<(i32, usize, usize)>>`, visited grid, water level = running max | [LC 407](https://leetcode.com/problems/trapping-rain-water-ii/) |
| 1000 | [Bus Routes](exercises/p1000_bus_routes.md) | Hard | BFS over **routes** instead of stops, `HashMap<stop, Vec<route>>` as an inverted index, visited sets for both routes and stops | [LC 815](https://leetcode.com/problems/bus-routes/) |

<!-- PROBLEM_TABLE:END -->
