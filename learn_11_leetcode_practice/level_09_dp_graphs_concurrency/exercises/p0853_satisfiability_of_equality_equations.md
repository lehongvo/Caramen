# Problem 0853: Satisfiability of Equality Equations

- **Difficulty:** Medium
- **Topics:** union-find over a fixed alphabet (`[usize; 26]`), two passes, byte patterns
- **Source:** [LeetCode 990. Satisfiability of Equality Equations](https://leetcode.com/problems/satisfiability-of-equality-equations/)

## Problem

Each string in `equations` has length 4 and is either `"x==y"` or `"x!=y"`, where `x` and `y`
are lowercase letters naming integer variables (they may be the same letter). Return `true` if
integers can be assigned to the variables so that every equation holds at the same time.

Implement `Solution::equations_possible(equations: Vec<String>) -> bool`.

## Examples

```text
Input:  equations = ["a==b", "b!=a"]
Output: false

Input:  equations = ["b==a", "a==b"]
Output: true

Input:  equations = ["a==b", "b!=c", "c==a"]
Output: false
Explanation: a == b and c == a force b == c.
```

## Constraints

- `1 <= equations.len() <= 500`
- `equations[i].len() == 4`; characters 0 and 3 are lowercase letters; characters 1-2 are
  `"=="` or `"!="`.

## Hints

1. Equalities group the variables into classes (equality is transitive).
2. Process all `==` equations first, then check that no `!=` equation has both sides in the same
   class. The order of the equations in the input does not matter.

## Starter code

Write your code in [`src/practice/p0853_satisfiability_of_equality_equations.rs`](../src/practice/p0853_satisfiability_of_equality_equations.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0853
```

```rust
pub struct Solution;

impl Solution {
    pub fn equations_possible(equations: Vec<String>) -> bool {
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

    fn run(eqs: &[&str]) -> bool {
        Solution::equations_possible(eqs.iter().map(|e| e.to_string()).collect())
    }

    #[test]
    fn example_contradiction() {
        assert!(!run(&["a==b", "b!=a"]));
    }

    #[test]
    fn example_consistent() {
        assert!(run(&["b==a", "a==b"]));
    }

    #[test]
    fn example_transitivity() {
        assert!(!run(&["a==b", "b!=c", "c==a"]));
    }

    #[test]
    fn same_letter() {
        assert!(!run(&["a!=a"]));
        assert!(run(&["c==c", "b==d", "x!=z"]));
    }

    #[test]
    fn inequality_before_equalities() {
        // The order of the input does not matter: z != a is checked after all equalities.
        let mut eqs = vec!["z!=a".to_string()];
        eqs.extend((b'a'..=b'y').map(|c| format!("{}=={}", c as char, (c + 1) as char)));
        assert!(!Solution::equations_possible(eqs));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** a 26-element union-find. Pass 1 unions the two letters of every `"x==y"`. Pass 2
returns `false` if some `"x!=y"` has `find(x) == find(y)` (this includes `"a!=a"`).

**Complexity:** `O(n * alpha(26))`, effectively `O(n)` time, `O(1)` space.

**Rust concepts:**
- A fixed array `[usize; 26]` initialised with `std::array::from_fn(|i| i)` is the parent table.
- Slice patterns on bytes: `match eq.as_bytes() { &[x, b'=', b'=', y] => ..., _ => ... }`
  destructure the 4 characters and check the operator in one step.
- A nested helper `fn find(parent: &mut [usize; 26], x: usize) -> usize` keeps the function
  self-contained.

```rust
pub struct Solution;

impl Solution {
    pub fn equations_possible(equations: Vec<String>) -> bool {
        fn find(parent: &mut [usize; 26], mut x: usize) -> usize {
            while parent[x] != x {
                parent[x] = parent[parent[x]];
                x = parent[x];
            }
            x
        }
        let letter = |b: u8| (b - b'a') as usize;
        let mut parent: [usize; 26] = std::array::from_fn(|i| i);
        for eq in &equations {
            if let &[x, b'=', b'=', y] = eq.as_bytes() {
                let (rx, ry) = (find(&mut parent, letter(x)), find(&mut parent, letter(y)));
                parent[rx] = ry;
            }
        }
        equations.iter().all(|eq| match eq.as_bytes() {
            &[x, b'!', b'=', y] => find(&mut parent, letter(x)) != find(&mut parent, letter(y)),
            _ => true,
        })
    }
}
```

Full tested solution: [`src/solutions/p0853_satisfiability_of_equality_equations.rs`](../src/solutions/p0853_satisfiability_of_equality_equations.rs)

</details>
