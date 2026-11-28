# Problem 0223: Furthest Point From Origin

- **Difficulty:** Easy
- **Topics:** enum with a "wildcard" variant, counting with a struct, `match` on `char`
- **Source:** [LeetCode 2833. Furthest Point From Origin](https://leetcode.com/problems/furthest-point-from-origin/)

## Problem

You stand at position `0` on a number line and receive a string `moves` of `'L'`, `'R'` and `'_'`
characters. Process them in order; each one is a single step:

- `'L'` means you **must** step left (position `- 1`);
- `'R'` means you **must** step right (position `+ 1`);
- `'_'` means you may **choose** to step either left or right.

Choosing the free steps as cleverly as possible, return the largest distance from `0` (the
absolute value of the final position) you can end at.

Implement `Solution::furthest_distance_from_origin(moves: String) -> i32`.

## Examples

```text
Input:  moves = "L_RL__R"
Output: 3      (2 forced L, 2 forced R, 3 free steps all to the left: final position -3)

Input:  moves = "_R__LL_"
Output: 5      (forced: 1 R, 2 L; the 4 free steps go left: -1 - 4 = -5)

Input:  moves = "_______"
Output: 7
```

## Constraints

- `1 <= moves.len() <= 50`
- `moves` contains only `'L'`, `'R'` and `'_'`.

## Hints

1. The order of the steps does not matter, only how many of each kind there are.
2. All free steps should go the same way: towards the side that is already winning.

## Starter code

Write your code in [`src/practice/p0223_furthest_point_from_origin.rs`](../src/practice/p0223_furthest_point_from_origin.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0223
```

```rust
pub struct Solution;

impl Solution {
    pub fn furthest_distance_from_origin(moves: String) -> i32 {
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

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::furthest_distance_from_origin("L_RL__R".to_string()),
            3
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::furthest_distance_from_origin("_R__LL_".to_string()),
            5
        );
    }

    #[test]
    fn example_all_free() {
        assert_eq!(
            Solution::furthest_distance_from_origin("_______".to_string()),
            7
        );
    }

    #[test]
    fn forced_steps_only() {
        assert_eq!(
            Solution::furthest_distance_from_origin("LRLR".to_string()),
            0
        );
        assert_eq!(
            Solution::furthest_distance_from_origin("RRR".to_string()),
            3
        );
    }

    #[test]
    fn free_steps_follow_the_majority() {
        // Forced: 3 R, 1 L => +2; the 2 free steps go right too.
        assert_eq!(
            Solution::furthest_distance_from_origin("RRLR__".to_string()),
            4
        );
    }

    #[test]
    fn longest_input() {
        let moves = "L".repeat(20) + &"R".repeat(25) + &"_".repeat(5);
        assert_eq!(Solution::furthest_distance_from_origin(moves), 10);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** count forced lefts, forced rights and free steps in a small `Tally` struct. The forced
steps leave you at distance `|lefts - rights|`; sending every free step further in that same
direction adds `free` more. So the answer is `|lefts - rights| + free`.

**Complexity:** `O(n)` time, `O(1)` space.

**Rust concepts:**
- The `Step` enum names the three kinds of characters; `Step::Either` is a variant, not a
  catch-all `_` pattern, so the counting `match` stays exhaustive.
- `#[derive(Default)]` on `Tally` gives a zeroed counter with `Tally::default()`.
- `i32::abs_diff` returns the distance as a `u32` without overflow worries.

```rust
pub struct Solution;

/// One character of `moves`.
#[derive(Debug, Clone, Copy)]
enum Step {
    Left,
    Right,
    Either,
}

impl Step {
    fn from_char(c: char) -> Option<Step> {
        match c {
            'L' => Some(Step::Left),
            'R' => Some(Step::Right),
            '_' => Some(Step::Either),
            _ => None,
        }
    }
}

/// How many steps of each kind were seen.
#[derive(Debug, Default)]
struct Tally {
    left: i32,
    right: i32,
    free: i32,
}

impl Solution {
    pub fn furthest_distance_from_origin(moves: String) -> i32 {
        let mut tally = Tally::default();
        for step in moves.chars().filter_map(Step::from_char) {
            match step {
                Step::Left => tally.left += 1,
                Step::Right => tally.right += 1,
                Step::Either => tally.free += 1,
            }
        }
        // `abs_diff` is at most 50 here, so the cast back to `i32` is lossless.
        tally.left.abs_diff(tally.right) as i32 + tally.free
    }
}
```

Full tested solution: [`src/solutions/p0223_furthest_point_from_origin.rs`](../src/solutions/p0223_furthest_point_from_origin.rs)

</details>
