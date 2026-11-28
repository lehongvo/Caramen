# Problem 0500: Number of Atoms

- **Difficulty:** Hard
- **Topics:** parsing with `Peekable<Chars>` and `next_if`, a stack of `BTreeMap`s, `mem::take` / `mem::replace`, `entry().or_default()`, building the output with `map` + `collect::<String>()`
- **Source:** [LeetCode 726. Number of Atoms](https://leetcode.com/problems/number-of-atoms/)

## Problem

A chemical formula is made of:

- **atoms**: an uppercase letter followed by zero or more lowercase letters (`H`, `Mg`), with an
  optional count of one or more digits (`H2`). A missing count means `1`; a written count is never
  `1` and never starts with `0`;
- **groups**: a formula in parentheses with an optional count, which multiplies every atom inside
  (`(OH)2`). Groups can be nested and concatenated.

Return the total count of every element as a string: the element names in **sorted** order, each
followed by its count **only if** the count is greater than `1`.

Implement `Solution::count_of_atoms(formula: String) -> String`.

## Examples

```text
Input:  formula = "H2O"
Output: "H2O"

Input:  formula = "Mg(OH)2"
Output: "H2MgO2"
Explanation: H: 2, Mg: 1, O: 2.

Input:  formula = "K4(ON(SO3)2)2"
Output: "K4N2O14S4"
Explanation: K: 4, N: 2, O: (1 + 3 * 2) * 2 = 14, S: 2 * 2 = 4.
```

## Constraints

- `1 <= formula.len() <= 1000`
- `formula` contains English letters, digits, `'('` and `')'`, and is always valid.
- Every count (also after multiplying) fits in an `i32`.

## Hints

1. Keep the counts of the group being read in a `BTreeMap<String, u64>` (sorted keys make the
   output easy) and the enclosing groups on a stack.
2. `'('`: push the current map and start an empty one (`std::mem::take`). `')'`: read the
   multiplier, pop the outer map, and add every inner count times the multiplier into it.
3. A small helper `fn read_count(chars: &mut Peekable<Chars>) -> u64` returns the number that
   follows, or `1` when there is none.

## Starter code

Write your code in [`src/practice/p0500_number_of_atoms.rs`](../src/practice/p0500_number_of_atoms.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0500
```

```rust
use std::collections::BTreeMap;
use std::iter::Peekable;
use std::str::Chars;

pub struct Solution;

impl Solution {
    pub fn count_of_atoms(formula: String) -> String {
        todo!("Write your solution here")
    }
}
```

## Tests

<details>
<summary>Show the tests used to check your solution</summary>

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn run(formula: &str) -> String {
        Solution::count_of_atoms(formula.to_string())
    }

    #[test]
    fn example_water() {
        assert_eq!(run("H2O"), "H2O");
    }

    #[test]
    fn example_group() {
        assert_eq!(run("Mg(OH)2"), "H2MgO2");
    }

    #[test]
    fn example_nested_groups() {
        assert_eq!(run("K4(ON(SO3)2)2"), "K4N2O14S4");
    }

    #[test]
    fn repeated_elements_and_groups_without_count() {
        assert_eq!(run("Be32"), "Be32");
        assert_eq!(run("HHe(H)"), "H2He");
        assert_eq!(run("((H))2"), "H2");
        assert_eq!(run("C6H12O6"), "C6H12O6");
    }

    #[test]
    fn large_multipliers() {
        assert_eq!(run("(H2O)1000"), "H2000O1000");
        assert_eq!(run("((((Fe)10)10)10)10"), "Fe10000");
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** a single left-to-right pass with a stack of count maps, one per open parenthesis. Atoms
add to the innermost map; a closing parenthesis multiplies the innermost map and merges it into
the next one. At the end, the remaining map holds the totals in sorted order.

**Complexity:** `O(n^2)` in the worst case (merging maps at each nesting level), `O(n)` space;
with `n <= 1000` this is instant.

**Rust concepts:**
- `Peekable::next_if` reads the lowercase letters of a name and the digits of a count.
- `std::mem::take(&mut current)` moves the map out and leaves an empty one in its place;
  `std::mem::replace` swaps in the popped outer map. Neither needs a clone.
- `*map.entry(name).or_default() += n` adds to a count, inserting `0` first if needed.
- `BTreeMap` iterates in key order, so the output is sorted without an explicit sort.

```rust
use std::collections::BTreeMap;
use std::iter::Peekable;
use std::str::Chars;

pub struct Solution;

fn read_count(chars: &mut Peekable<Chars<'_>>) -> u64 {
    let mut digits = String::new();
    while let Some(d) = chars.next_if(|c| c.is_ascii_digit()) {
        digits.push(d);
    }
    digits.parse().unwrap_or(1)
}

impl Solution {
    pub fn count_of_atoms(formula: String) -> String {
        let mut chars = formula.chars().peekable();
        let mut current: BTreeMap<String, u64> = BTreeMap::new();
        let mut outer: Vec<BTreeMap<String, u64>> = Vec::new();
        while let Some(c) = chars.next() {
            match c {
                '(' => outer.push(std::mem::take(&mut current)),
                ')' => {
                    let multiplier = read_count(&mut chars);
                    let inner = std::mem::replace(&mut current, outer.pop().unwrap_or_default());
                    for (name, n) in inner {
                        *current.entry(name).or_default() += n * multiplier;
                    }
                }
                _ => {
                    let mut name = String::from(c);
                    while let Some(l) = chars.next_if(|c| c.is_ascii_lowercase()) {
                        name.push(l);
                    }
                    let n = read_count(&mut chars);
                    *current.entry(name).or_default() += n;
                }
            }
        }
        current
            .into_iter()
            .map(|(name, n)| if n > 1 { format!("{name}{n}") } else { name })
            .collect()
    }
}
```

Full tested solution: [`src/solutions/p0500_number_of_atoms.rs`](../src/solutions/p0500_number_of_atoms.rs)

</details>
