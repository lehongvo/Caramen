# Problem 0064: Distribute Candies to People

- **Difficulty:** Easy
- **Topics:** `loop` with `break`, `min`, indexing with `%`, `vec![0; n]`
- **Source:** [LeetCode 1103. Distribute Candies to People](https://leetcode.com/problems/distribute-candies-to-people/)

## Problem

You have `candies` candies and `num_people` people standing in a row. Hand out candies in turns:

- the first person gets 1 candy, the second 2, ..., the last person `num_people` candies;
- then go back to the first person, who gets `num_people + 1`, the second `num_people + 2`, and so on.

Each turn gives one more candy than the previous turn. When fewer candies are left than the
current turn requires, the current person receives **all** the remaining candies and the
distribution ends. Return a vector of length `num_people` with the total each person received.

Implement `Solution::distribute_candies(candies: i32, num_people: i32) -> Vec<i32>`.

## Examples

```text
Input:  candies = 7, num_people = 4
Output: [1, 2, 3, 1]
Explanation: 1, 2 and 3 candies are given; only 1 is left for the fourth person.

Input:  candies = 10, num_people = 3
Output: [5, 2, 3]
Explanation: first round 1, 2, 3 (6 used); the first person then gets the remaining 4.
```

## Constraints

- `1 <= candies <= 10^9`
- `1 <= num_people <= 1000`

## Hints

1. Keep a counter `turn` (0, 1, 2, ...); the person is `turn % num_people` and the amount is `turn + 1`.
2. Give `min(turn + 1, remaining)` each time and stop when nothing is left.

## Starter code

Write your code in [`src/practice/p0064_distribute_candies_to_people.rs`](../src/practice/p0064_distribute_candies_to_people.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0064
```

```rust
pub struct Solution;

impl Solution {
    pub fn distribute_candies(candies: i32, num_people: i32) -> Vec<i32> {
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
    fn example_single_round() {
        assert_eq!(Solution::distribute_candies(7, 4), vec![1, 2, 3, 1]);
    }

    #[test]
    fn example_wraps_around() {
        assert_eq!(Solution::distribute_candies(10, 3), vec![5, 2, 3]);
    }

    #[test]
    fn single_person_gets_everything() {
        assert_eq!(Solution::distribute_candies(1, 1), vec![1]);
        assert_eq!(
            Solution::distribute_candies(1_000_000_000, 1),
            vec![1_000_000_000]
        );
    }

    #[test]
    fn many_people_few_candies() {
        let result = Solution::distribute_candies(5, 1000);
        assert_eq!(&result[..4], &[1, 2, 2, 0]);
        assert_eq!(result.iter().sum::<i32>(), 5);
    }

    #[test]
    fn largest_input_keeps_every_candy() {
        let result = Solution::distribute_candies(1_000_000_000, 1000);
        assert_eq!(result.len(), 1000);
        assert_eq!(result.iter().map(|&c| c as i64).sum::<i64>(), 1_000_000_000);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** simulate with a `loop`. Each turn gives `amount = (turn + 1).min(remaining)` to person
`turn % num_people`. Stop with `break` once `remaining` reaches 0. The number of turns is about
`sqrt(2 * candies)`, under 45 000.

**Complexity:** `O(sqrt(candies) + num_people)` time, `O(num_people)` space.

**Rust concepts:**
- `loop { ... break; }` is an endless loop that you leave explicitly; it fits when the exit
  condition is computed in the middle of the body.
- `vec![0; n]` creates a vector of `n` zeros. The index `turn % n` is a `usize`, so the counter
  is a `usize` and the amount is converted with `as i32` (it stays below 50 000).
- `a.min(b)` picks the smaller value, which handles the "last, partial turn".

```rust
pub struct Solution;

impl Solution {
    pub fn distribute_candies(candies: i32, num_people: i32) -> Vec<i32> {
        let n = num_people as usize;
        let mut result = vec![0; n];
        let mut remaining = candies;
        let mut turn: usize = 0;
        loop {
            let amount = (turn as i32 + 1).min(remaining);
            result[turn % n] += amount;
            remaining -= amount;
            if remaining == 0 {
                break;
            }
            turn += 1;
        }
        result
    }
}
```

Full tested solution: [`src/solutions/p0064_distribute_candies_to_people.rs`](../src/solutions/p0064_distribute_candies_to_people.rs)

</details>
