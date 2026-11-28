# Caramen

Learning Rust step by step, following *The Rust Programming Language*: one Cargo project per lesson,
then 1000 practice problems from easy to hard.

## Lessons

Tick a lesson when you finish it.

- [x] [01 · Hello, Cargo](learn_01_hello/)
- [x] [02 · Guessing game](learn_02_gessing_game/)
- [x] [03 · Variables and data types](learn_03_variables/README.md)
- [x] [04 · Functions and control flow](learn_04_functions_control_flow/README.md)
- [x] [05 · Ownership](learn_05_ownership/README.md)
- [x] [06 · Structs and `impl`](learn_06_structs/README.md)
- [x] [07 · Enums, `match` and `Option`](learn_07_enums_match/README.md)
- [x] [08 · Collections](learn_08_collections/README.md)
- [x] [09 · Error handling](learn_09_error_handling/README.md)
- [ ] [10 · Generics, traits and lifetimes](learn_10_generics_traits_lifetimes/README.md) (in progress)
- [ ] [11 · Practice: 1000 problems in 10 levels](learn_11_leetcode_practice/README.md)

## Practice progress

Progress in [lesson 11](learn_11_leetcode_practice/README.md). A problem counts as **solved** when every test of
your file in `src/practice/` passes, and as **started** when you edited the file but some tests still fail.

Update this table after a practice session:

```bash
python3 -I learn_11_leetcode_practice/tools/progress.py
```

<!-- PRACTICE_PROGRESS:START -->

_Last updated: 2026-11-28 21:29. Run `python3 -I learn_11_leetcode_practice/tools/progress.py` to refresh._

| Level | Solved | Progress | Started | Next problem |
| ----- | ------ | -------- | ------- | ------------ |
| [1. Basics](learn_11_leetcode_practice/level_01_basics/README.md) | 0/100 | `░░░░░░░░░░   0%` | - | [0001](learn_11_leetcode_practice/level_01_basics/exercises/p0001_fizz_buzz.md) |
| [2. Ownership & Strings](learn_11_leetcode_practice/level_02_ownership_strings/README.md) (being written) | 0/71 | `░░░░░░░░░░   0%` | - | [0101](learn_11_leetcode_practice/level_02_ownership_strings/exercises/p0101_take_and_give_back.md) |
| [3. Structs, Enums & Pattern Matching](learn_11_leetcode_practice/level_03_structs_enums_match/README.md) | - | not available yet | - | - |
| [4. Collections](learn_11_leetcode_practice/level_04_collections/README.md) | - | not available yet | - | - |
| [5. Errors, Closures & Iterators](learn_11_leetcode_practice/level_05_errors_closures_iterators/README.md) | - | not available yet | - | - |
| [6. Generics, Traits & Lifetimes](learn_11_leetcode_practice/level_06_generics_traits_lifetimes/README.md) | - | not available yet | - | - |
| [7. Smart Pointers, Linked Lists & Trees](learn_11_leetcode_practice/level_07_smart_pointers_lists_trees/README.md) | - | not available yet | - | - |
| [8. Algorithm Techniques](learn_11_leetcode_practice/level_08_algorithms/README.md) | - | not available yet | - | - |
| [9. DP, Graphs & Concurrency](learn_11_leetcode_practice/level_09_dp_graphs_concurrency/README.md) | - | not available yet | - | - |
| [10. Advanced](learn_11_leetcode_practice/level_10_advanced/README.md) | - | not available yet | - | - |
| **Total** | **0/1000** | `░░░░░░░░░░   0%` | | |

<!-- PRACTICE_PROGRESS:END -->

## How to practise

```bash
cd learn_11_leetcode_practice/level_01_basics
# 1. read the problem:   exercises/p0001_fizz_buzz.md
# 2. write your code in: src/practice/p0001_fizz_buzz.rs
cargo test practice::p0001        # 3. run the tests until they all pass
```

Then refresh the progress table with the command above.
