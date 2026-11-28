# Authoring conventions for learn_11_leetcode_practice

This document is for **problem authors** (including AI agents). Learners only need `README.md`.
The reference problem is `level_01_basics/src/solutions/p0001_fizz_buzz.rs`. When in doubt, copy its shape.

**Everything inside this lesson is written in English**: statements, hints, explanations, test names,
comments and READMEs. Do not write Vietnamese anywhere in `learn_11_leetcode_practice/` (`tools/check.py`
rejects Vietnamese diacritics).

## 1. Layout

```
learn_11_leetcode_practice/
├── README.md                    # overview of the 10 levels (learners start here)
├── tools/
│   ├── CONVENTIONS.md           # this file
│   ├── generate.py              # generates exercises/*.md, src/practice/ and the README table from src/solutions/
│   └── check.py                 # automated checks
└── level_NN_name/               # one independent Cargo package per level
    ├── Cargo.toml               # NO dependencies (standard library only)
    ├── README.md                # level introduction + problem table (the table is generated)
    ├── exercises/               # GENERATED: the Markdown exercise files (statement, starter code, tests, answer)
    │   └── pNNNN_name.md
    └── src/
        ├── lib.rs               # declares `practice`, `solutions` (and `common` if needed)
        ├── common.rs            # optional shared types: ListNode, TreeNode, test helpers...
        ├── solutions/           # SOURCE OF TRUTH: authors only edit files here
        │   ├── mod.rs           # `pub mod pNNNN_name;` lines in increasing order
        │   └── pNNNN_name.rs
        └── practice/            # GENERATED: starter code with todo!() where learners write and test code
```

- Level `k` holds exactly 100 problems numbered `(k-1)*100+1` to `k*100`, always 4 digits (`0001`...`1000`).
- File name: `pNNNN_<snake_case_name>.rs`, e.g. `p0001_fizz_buzz.rs`. For LeetCode problems derive the
  name from the slug (`-` becomes `_`).

## 2. Solution file format

In this order:

1. **Title and metadata** (the very first line of the file):
   ```
   //! # Problem NNNN: <Title>
   //!
   //! - **Difficulty:** Easy | Medium | Hard
   //! - **Topics:** <Rust concepts + techniques, short, `code` for names>
   //! - **Source:** <one of the three forms below>
   ```
   Source forms:
   - A LeetCode problem solved with **LeetCode's exact Rust signature**:
     `[LeetCode 412. Fizz Buzz](https://leetcode.com/problems/fizz-buzz/)`
   - A problem based on LeetCode but with a different API (no Rust template, concurrency problems, ...):
     `Adapted from [LeetCode 1114. Print in Order](https://leetcode.com/problems/print-in-order/)`
   - An original problem: `Original`, optionally followed by documentation links, e.g.
     `Original · [The Rust Book 4.2](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)`

   The LeetCode number, English title and slug must match LeetCode **exactly** (`tools/check.py` compares
   them with `tools/.cache/leetcode_index.csv`). Never use Premium problems. A direct LeetCode problem must
   state LeetCode's own difficulty.

2. **`//! ## Problem`**: the statement **in your own words**. Never copy LeetCode's text verbatim.
   State the inputs, the output, the exact function/method signature to implement, and every rule
   (output order, what to return for empty input, ...).
3. **`//! ## Examples`**: at least 2 examples in a ` ```text ` block, with a short explanation when useful.
4. **`//! ## Constraints`**: input limits (original problems may say "No special constraints.").
5. **`//! ## Hints`**: 1-3 progressive hints that do not give the whole solution away.
6. **The explanation**, wrapped in `// @solution-start remove` ... `// @solution-end`:
   ```
   // @solution-start remove
   //!
   //! ## Solution
   //!
   //! **Idea:** ...
   //!
   //! **Complexity:** `O(...)` time, `O(...)` space.     (required for LeetCode-based problems)
   //!
   //! **Rust concepts:**
   //! - ...
   // @solution-end
   ```
7. **Code**: types, signatures, bodies. Solution bodies are wrapped in marker regions.
8. **Tests**: `#[cfg(test)] mod tests { use super::*; ... }` with at least **3** tests
   (the examples + edge cases + a large/tricky case when it makes sense). Descriptive snake_case names.

## 3. Solution markers

| Opening line | In the starter code (and the "Starter code" section of the .md), the region becomes |
| --- | --- |
| `// @solution-start` | `todo!("Write your solution here")` (same indentation) |
| `// @solution-start remove` | nothing (removed) |
| `// @solution-start replace: <code>` | the line `<code>` (same indentation) |

The closing line is always `// @solution-end`. Regions never nest. Every line containing `@solution`
must be a valid marker.

Rules so that the starter code (`src/practice`) **always compiles** and **every one of its tests fails**
before the learner does any work:

- Hide only **bodies** and **private details** (private fields, private helper functions). Keep everything
  the tests use: type names, enum variants, `pub` fields, function/method signatures, traits.
- Helper functions the learner should write themselves: wrap the whole function in a `remove` region.
- Private struct fields: wrap them in a `remove` region (an empty struct `{}` still compiles).
- Functions returning `impl Trait` cannot just use `todo!()` (error E0277: `!` is not an iterator):
  use `// @solution-start replace: std::iter::from_fn(|| todo!())` for `impl Iterator` (verified to compile
  on 1.87 and to panic as soon as a test iterates it), or `Box::new(|_| todo!())`-style stubs for closures.
  Never stub with something that silently works (e.g. `std::iter::empty()`), or tests may pass.
  For macros use `replace:` with a temporary arm such as `($($t:tt)*) => { todo!() };`.
- A test using `#[should_panic]` must have `expected = "..."` (otherwise `todo!()` would pass it).
- `#[ignore]` is not allowed.

## 4. Quality bar

- **Correctness first**: every solution passes its tests; the tests check exactly what the statement asks
  (sort before comparing when "any order" is allowed). Example outputs in the statement must match the tests.
- **Idiomatic Rust**: prefer iterators, `match`, `Option`/`Result`; no careless `unwrap()` in solutions
  (only when the statement guarantees safety, with a short comment). No `unsafe` except in problems that
  teach `unsafe` (level 10).
- **Clean**: `cargo clippy --lib --tests` with zero warnings and `cargo fmt --check` clean. If LeetCode's
  mandated signature triggers a clippy lint, add `#[allow(clippy::lint_name)]` right there with a comment.
- **Toolchain**: the machine runs **Rust 1.87.0** (edition 2024). Do not use newer features
  (for example `if let ... && ...` "let chains" need 1.88). `tools/check.py` catches compile errors.
- **Standard library only.** No crates.
- **Concurrency**: tests must be **deterministic** (use `join`, channels, `Barrier`, ...; never rely on `sleep`).
- **Difficulty**: within a level, problems go from easier to harder. No LeetCode problem twice in one level.
  Avoid reusing a LeetCode problem from another level; if you must, the new problem must practise a different
  technique and say so in the statement.
- **Links**: only real URLs (doc.rust-lang.org/book, /std, /reference, /rust-by-example, /nomicon, ...).
  If unsure, verify on the web first (`tools/check.py --links` verifies doc.rust-lang.org links).

## 5. Workflow

```bash
cd learn_11_leetcode_practice
# 1. write/edit files in level_NN_name/src/solutions/ and add `pub mod ...;` to solutions/mod.rs
# 2. generate exercises/*.md + src/practice/ + the README table
python3 -I tools/generate.py level_NN_name --force
# 3. quick check while authoring (no clippy, no practice tests)
python3 -I tools/check.py level_NN_name --quick
# 4. full check before handing over (must report 0 errors)
cargo fmt --manifest-path level_NN_name/Cargo.toml
python3 -I tools/generate.py level_NN_name --force
python3 -I tools/check.py level_NN_name --links
```

## 6. Learner work in src/practice

Learners write their solutions directly in `src/practice/`. `src/practice/.generated` stores the hash of every
starter file as generated. `tools/generate.py` (with or without `--force`) only rewrites starter files that are
still identical to what it generated; files a learner edited are kept, and `tools/check.py` reports them as a
WARNING (their tests may pass, their formatting is the learner's business). Never edit or delete `.generated`
by hand. `tools/progress.py` counts solved problems and updates the table in the repository README.

## 7. How the Markdown exercise is built

`exercises/pNNNN_name.md` is assembled from the solution file:

1. All `//!` doc lines **before** `## Solution` → the statement (title, metadata, Problem, Examples, Constraints, Hints).
2. `## Starter code` → the code of `src/practice/...rs` (without docs and tests).
3. `## Tests` → the test module (collapsed in `<details>`).
4. `## Answer` → the doc lines **after** `## Solution` + the full solution code (collapsed in `<details>`).

So write statements and explanations as proper Markdown inside `//!`. Comments inside solution bodies are
shown in the Answer section, so keep them helpful and in English.
