# Rust Practice: 1000 Problems from Easy to Hard

> Lesson 11 of this repository: 1000 practice problems in 10 levels that cover the Rust language from the
> first `let` to macros, `unsafe` and async. Many problems are LeetCode problems solved with LeetCode's
> exact Rust signature; the rest are original problems that practise a specific Rust feature.
> Every problem has a detailed statement, starter code, tests and a full, tested answer.

## Levels

| Level | Problems | Focus |
| ----- | -------- | ----- |
| [1. Basics](level_01_basics/README.md) | 0001-0100 | variables, scalar types, operators, control flow, loops, functions |
| [2. Ownership & Strings](level_02_ownership_strings/README.md) | 0101-0200 | ownership, borrowing, slices, `String` vs `&str` |
| [3. Structs, Enums & Pattern Matching](level_03_structs_enums_match/README.md) | 0201-0300 | structs, methods, enums, `match`, `Option` |
| [4. Collections](level_04_collections/README.md) | 0301-0400 | `Vec`, `HashMap`, `HashSet`, `BTreeMap`, `VecDeque`, `BinaryHeap` |
| [5. Errors, Closures & Iterators](level_05_errors_closures_iterators/README.md) | 0401-0500 | `Result`, `?`, custom errors, closures, iterators |
| [6. Generics, Traits & Lifetimes](level_06_generics_traits_lifetimes/README.md) | 0501-0600 | generics, traits, trait objects, operator overloading, lifetimes |
| [7. Smart Pointers, Linked Lists & Trees](level_07_smart_pointers_lists_trees/README.md) | 0601-0700 | `Box`, `Rc`, `RefCell`, `Weak`, linked lists, binary trees |
| [8. Algorithm Techniques](level_08_algorithms/README.md) | 0701-0800 | two pointers, sliding window, binary search, BFS/DFS, backtracking, greedy |
| [9. Dynamic Programming, Graphs & Concurrency](level_09_dp_graphs_concurrency/README.md) | 0801-0900 | DP, graph algorithms, threads, `Arc`, `Mutex`, channels |
| [10. Advanced](level_10_advanced/README.md) | 0901-1000 | hard problems, advanced data structures, macros, `unsafe`, async |

## How to practise

Each level is an independent Cargo package:

```
level_01_basics/
├── README.md              # what this level covers + the list of problems
├── exercises/             # one Markdown file per problem: READ THE PROBLEM HERE
│   └── p0001_fizz_buzz.md
└── src/
    ├── practice/          # starter code with todo!(): WRITE YOUR SOLUTION HERE
    └── solutions/         # complete reference solutions (all tested)
```

1. Open a problem in `exercises/` (for example `level_01_basics/exercises/p0001_fizz_buzz.md`) and read
   the statement, examples, constraints and hints.
2. Write your code in the file with the same name in `src/practice/`, replacing every `todo!(...)`.
   Do not edit the tests at the bottom of the file.
3. Run the tests for that problem:
   ```bash
   cd learn_11_leetcode_practice/level_01_basics
   cargo test practice::p0001
   ```
4. When all tests pass (or you are stuck), open the **Answer** section at the end of the Markdown file.
   It explains the idea, the complexity and the Rust concepts, and shows the full solution.
5. Record your progress in the repository README (solved / started problems per level):
   ```bash
   python3 -I learn_11_leetcode_practice/tools/progress.py   # run from the repository root
   ```

Your code is safe when problems are updated: `src/practice/.generated` remembers which starter files are
untouched, and `tools/generate.py` only rewrites those. Files you edited are never overwritten (unless you run
`tools/generate.py <level> --reset-practice` on purpose).

Problems marked **LC** in a level's table use LeetCode's exact Rust signature: paste the
`impl Solution { ... }` block into LeetCode and submit it. Problems marked **Adapted from LC** keep the idea
but use a different API (for example because LeetCode has no Rust template for them).

Requirements: Rust 1.87 or newer (edition 2024). No external crates are used.

## For authors

Problems are written once, in `src/solutions/`; the Markdown exercises, the starter code and the tables are
generated from them. See [tools/CONVENTIONS.md](tools/CONVENTIONS.md), then:

```bash
python3 -I tools/generate.py level_01_basics --force   # regenerate exercises/, src/practice/, README table
python3 -I tools/check.py level_01_basics              # compile, test, lint and validate one level
python3 -I tools/check.py --all                        # validate everything
```

The LeetCode problem index used to validate numbers, titles and slugs comes from the public dataset
[kaysss/leetcode-problem-set](https://huggingface.co/datasets/kaysss/leetcode-problem-set).
