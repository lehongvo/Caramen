# Problem 0059: Count of Matches in Tournament

- **Difficulty:** Easy
- **Topics:** `while` simulation, integer division, odd/even cases
- **Source:** [LeetCode 1688. Count of Matches in Tournament](https://leetcode.com/problems/count-of-matches-in-tournament/)

## Problem

A knockout tournament starts with `n` teams. Each round works like this:

- if the number of teams is even, they are paired up: `teams / 2` matches are played and
  `teams / 2` teams advance;
- if it is odd, one team advances without playing; the others are paired up:
  `(teams - 1) / 2` matches are played and `(teams - 1) / 2 + 1` teams advance.

Rounds continue until a single winner is left. Return the total number of matches played.

Implement `Solution::number_of_matches(n: i32) -> i32`.

## Examples

```text
Input:  n = 7
Output: 6
Explanation: 7 teams: 3 matches, 4 advance; 4 teams: 2 matches, 2 advance; 2 teams: 1 match.

Input:  n = 14
Output: 13
Explanation: 7 + 3 + 2 + 1 = 13.
```

## Constraints

- `1 <= n <= 200`

## Hints

1. Simulate the rounds with `while teams > 1`.
2. For an odd number of teams, `teams / 2` already rounds down to `(teams - 1) / 2`.
3. Think about it differently: every match eliminates exactly one team. How many teams must be eliminated?

## Starter code

Write your code in [`src/practice/p0059_count_of_matches_in_tournament.rs`](../src/practice/p0059_count_of_matches_in_tournament.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0059
```

```rust
pub struct Solution;

impl Solution {
    pub fn number_of_matches(n: i32) -> i32 {
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
    fn example_seven() {
        assert_eq!(Solution::number_of_matches(7), 6);
    }

    #[test]
    fn example_fourteen() {
        assert_eq!(Solution::number_of_matches(14), 13);
    }

    #[test]
    fn one_team_plays_no_match() {
        assert_eq!(Solution::number_of_matches(1), 0);
    }

    #[test]
    fn always_n_minus_one() {
        for n in 1..=200 {
            assert_eq!(Solution::number_of_matches(n), n - 1, "n = {n}");
        }
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** simulate: each round adds `teams / 2` matches and leaves `teams - teams / 2` teams
(for odd `teams` that includes the one with a bye). The tests also confirm the shortcut: since
each match eliminates one team and `n - 1` teams must go, the answer is always `n - 1`.

**Complexity:** `O(log n)` time for the simulation, `O(1)` space.

**Rust concepts:**
- `teams / 2` rounds down for positive integers, so it covers the even and odd cases at once.
- `teams - teams / 2` is the number of teams that advance (`ceil(teams / 2)`); it could also be
  written `teams.div_ceil(2)` or `(teams + 1) / 2`.

```rust
pub struct Solution;

impl Solution {
    pub fn number_of_matches(n: i32) -> i32 {
        let mut teams = n;
        let mut matches = 0;
        while teams > 1 {
            let played = teams / 2;
            matches += played;
            teams -= played;
        }
        matches
    }
}
```

Full tested solution: [`src/solutions/p0059_count_of_matches_in_tournament.rs`](../src/solutions/p0059_count_of_matches_in_tournament.rs)

</details>
