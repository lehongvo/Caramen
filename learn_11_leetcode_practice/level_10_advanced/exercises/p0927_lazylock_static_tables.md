# Problem 0927: Global Tables with `LazyLock` and `OnceLock`

- **Difficulty:** Medium
- **Topics:** `std::sync::LazyLock` statics, `std::sync::OnceLock`, thread-safe one-time initialisation, `AtomicUsize`, `&'static` results
- **Source:** Original · [std: `LazyLock`](https://doc.rust-lang.org/std/sync/struct.LazyLock.html) · [std: `OnceLock`](https://doc.rust-lang.org/std/sync/struct.OnceLock.html)

## Problem

A `static` must be initialised by a constant expression, so a `HashMap` or a computed table cannot
be a plain `static`. Two std types solve this, both safe to share between threads:

- `static T: LazyLock<V> = LazyLock::new(|| ...);` runs the closure the first time `T` is used.
- `OnceLock<V>` is filled explicitly with `get_or_init`, which is handy inside a function.

Implement:

1. `static ROMAN: LazyLock<HashMap<char, u32>>` with the seven numerals `I V X L C D M`, and
   `roman_to_int(s: &str) -> Option<u32>`: the value of a Roman numeral using the subtractive rule
   (a symbol smaller than the next one is subtracted), `None` if a character is not a numeral or
   `s` is empty. No further validation is required.
2. `static STOP_WORDS: LazyLock<HashSet<&'static str>>` with `"a"`, `"an"`, `"and"`, `"of"`,
   `"the"`, `"to"`, and `content_words(text: &str) -> Vec<&str>`: the whitespace-separated words of
   `text` that are not stop words, compared case-insensitively, in their original spelling.
3. `primes_below_10000() -> &'static [u32]`: every prime below 10 000, computed **once** with a
   sieve and stored in a `OnceLock<Vec<u32>>` declared inside the function, and
   `sieve_runs() -> usize`: how many times the sieve has run (an `AtomicUsize` static). It must be
   `1` after any number of calls from any number of threads.

## Examples

```text
roman_to_int("MCMXCIV")              => Some(1994)
roman_to_int("XIZ")                  => None
content_words("The Art of War and Peace") => ["Art", "War", "Peace"]
primes_below_10000().len()           => 1229
sieve_runs()                         => 1
```

## Constraints

- `s.len() <= 15` for `roman_to_int`.

## Hints

1. `LazyLock` derefs to its value: `ROMAN.get(&c)` works directly.
2. `static PRIMES: OnceLock<Vec<u32>> = OnceLock::new();` is a valid `static` because
   `OnceLock::new` is a `const fn`.
3. Increment the counter **inside** the `get_or_init` closure; `OnceLock` guarantees that closure
   runs once even if several threads race.

## Starter code

Write your code in [`src/practice/p0927_lazylock_static_tables.rs`](../src/practice/p0927_lazylock_static_tables.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0927
```

```rust
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{LazyLock, OnceLock};

static ROMAN: LazyLock<HashMap<char, u32>> = LazyLock::new(|| todo!());

static STOP_WORDS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| todo!());

static SIEVE_RUNS: AtomicUsize = AtomicUsize::new(0);

pub fn roman_to_int(s: &str) -> Option<u32> {
    todo!("Write your solution here")
}

pub fn content_words(text: &str) -> Vec<&str> {
    todo!("Write your solution here")
}

pub fn primes_below_10000() -> &'static [u32] {
    todo!("Write your solution here")
}

pub fn sieve_runs() -> usize {
    todo!("Write your solution here")
}
```

## Tests

<details>
<summary>Show the tests used to check your solution</summary>

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn roman_numerals() {
        assert_eq!(roman_to_int("MCMXCIV"), Some(1994));
        assert_eq!(roman_to_int("III"), Some(3));
        assert_eq!(roman_to_int("LVIII"), Some(58));
        assert_eq!(roman_to_int("MMMCMXCIX"), Some(3999));
        assert_eq!(roman_to_int("XIZ"), None);
        assert_eq!(roman_to_int(""), None);
    }

    #[test]
    fn stop_words_are_removed_case_insensitively() {
        assert_eq!(
            content_words("The Art of War and Peace"),
            vec!["Art", "War", "Peace"]
        );
        assert!(content_words("a an AND the").is_empty());
        assert_eq!(content_words("  to be  "), vec!["be"]);
    }

    #[test]
    fn primes_table() {
        let primes = primes_below_10000();
        assert_eq!(primes.len(), 1229);
        assert_eq!(&primes[..5], &[2, 3, 5, 7, 11]);
        assert_eq!(primes.last(), Some(&9973));
        assert!(std::ptr::eq(primes, primes_below_10000()));
        assert_eq!(sieve_runs(), 1);
    }

    #[test]
    fn initialised_once_across_threads() {
        let handles: Vec<_> = (0..8)
            .map(|_| thread::spawn(|| primes_below_10000().as_ptr() as usize))
            .collect();
        let addresses: Vec<usize> = handles.into_iter().map(|h| h.join().unwrap()).collect();
        assert!(addresses.windows(2).all(|w| w[0] == w[1]));
        assert_eq!(sieve_runs(), 1);
        assert_eq!(roman_to_int("XLII"), Some(42));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** the two lookup tables are `LazyLock` statics built on first use. `roman_to_int` maps
characters to values (failing on an unknown one) and adds or subtracts each value depending on
the next one. `primes_below_10000` keeps its sieve result in a function-local `OnceLock` static;
the counter is bumped inside the initialiser, so it records actual computations.

**Complexity:** the sieve is `O(n log log n)` once; later calls are `O(1)`.

**Rust concepts:**
- `LazyLock` (stable since Rust 1.80) and `OnceLock` (1.70) block other threads while the
  initialiser runs, then give every thread the same `&'static` value.
- Returning `&'static [u32]` is possible because the data lives in a `static` forever.
- `AtomicUsize::fetch_add` with `Ordering::Relaxed` is enough for a simple counter.
- Unlike `static mut`, these types need no `unsafe`.

```rust
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{LazyLock, OnceLock};

static ROMAN: LazyLock<HashMap<char, u32>> =
    LazyLock::new(|| {
        HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ])
    });

static STOP_WORDS: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| HashSet::from(["a", "an", "and", "of", "the", "to"]));

static SIEVE_RUNS: AtomicUsize = AtomicUsize::new(0);

pub fn roman_to_int(s: &str) -> Option<u32> {
    if s.is_empty() {
        return None;
    }
    let values: Vec<u32> = s
        .chars()
        .map(|c| ROMAN.get(&c).copied())
        .collect::<Option<_>>()?;
    let mut total = 0;
    for (i, &value) in values.iter().enumerate() {
        match values.get(i + 1) {
            Some(&next) if value < next => total -= value as i64,
            _ => total += value as i64,
        }
    }
    u32::try_from(total).ok()
}

pub fn content_words(text: &str) -> Vec<&str> {
    text.split_whitespace()
        .filter(|word| !STOP_WORDS.contains(word.to_lowercase().as_str()))
        .collect()
}

pub fn primes_below_10000() -> &'static [u32] {
    static PRIMES: OnceLock<Vec<u32>> = OnceLock::new();
    PRIMES.get_or_init(|| {
        SIEVE_RUNS.fetch_add(1, Ordering::Relaxed);
        let limit = 10_000;
        let mut is_prime = vec![true; limit];
        is_prime[0] = false;
        is_prime[1] = false;
        let mut p = 2;
        while p * p < limit {
            if is_prime[p] {
                for multiple in (p * p..limit).step_by(p) {
                    is_prime[multiple] = false;
                }
            }
            p += 1;
        }
        (0..limit as u32)
            .filter(|&n| is_prime[n as usize])
            .collect()
    })
}

pub fn sieve_runs() -> usize {
    SIEVE_RUNS.load(Ordering::Relaxed)
}
```

Full tested solution: [`src/solutions/p0927_lazylock_static_tables.rs`](../src/solutions/p0927_lazylock_static_tables.rs)

</details>
