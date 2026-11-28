# Problem 0389: Reveal Cards In Increasing Order

- **Difficulty:** Medium
- **Topics:** simulating a process on a `VecDeque` of **positions**, `pop_front` + `push_back`, filling a result by index
- **Source:** [LeetCode 950. Reveal Cards In Increasing Order](https://leetcode.com/problems/reveal-cards-in-increasing-order/)

## Problem

You have a deck of cards with **distinct** integer values. You may order the deck any way you
like, and then you repeat these steps until no card is left:

1. reveal the top card and remove it from the deck;
2. if cards remain, move the next top card to the **bottom** of the deck (without revealing it).

Return an ordering of the deck (top card first) so that the cards are revealed in
**increasing** order.

Implement `Solution::deck_revealed_increasing(deck: Vec<i32>) -> Vec<i32>`.

## Examples

```text
Input:  deck = [17, 13, 11, 2, 3, 5, 7]
Output: [2, 13, 3, 11, 5, 17, 7]
Explanation: reveal 2, move 13 to the bottom: [3, 11, 5, 17, 7, 13];
             reveal 3, move 11: [5, 17, 7, 13, 11]; reveal 5, move 17: [7, 13, 11, 17];
             reveal 7, move 13: [11, 17, 13]; reveal 11, move 17: [13, 17];
             reveal 13, move 17: [17]; reveal 17.

Input:  deck = [1, 1000]
Output: [1, 1000]
```

## Constraints

- `1 <= deck.len() <= 1000`
- `1 <= deck[i] <= 10^6`, all distinct

## Hints

1. The process does not depend on the card values: it always reveals the **positions** of the
   deck in the same order.
2. Simulate the process on a `VecDeque` of positions `0, 1, ..., n - 1` to learn that order.
3. Then give the smallest card to the first revealed position, the next smallest to the second,
   and so on.

## Starter code

Write your code in [`src/practice/p0389_reveal_cards_in_increasing_order.rs`](../src/practice/p0389_reveal_cards_in_increasing_order.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0389
```

```rust
use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn deck_revealed_increasing(mut deck: Vec<i32>) -> Vec<i32> {
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

    /// Runs the reveal process and returns the cards in the order they are revealed.
    fn reveal(deck: &[i32]) -> Vec<i32> {
        let mut q: VecDeque<i32> = deck.iter().copied().collect();
        let mut out = Vec::new();
        while let Some(top) = q.pop_front() {
            out.push(top);
            if let Some(next) = q.pop_front() {
                q.push_back(next);
            }
        }
        out
    }

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::deck_revealed_increasing(vec![17, 13, 11, 2, 3, 5, 7]),
            vec![2, 13, 3, 11, 5, 17, 7]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::deck_revealed_increasing(vec![1, 1000]),
            vec![1, 1000]
        );
    }

    #[test]
    fn single_and_three_cards() {
        assert_eq!(Solution::deck_revealed_increasing(vec![5]), vec![5]);
        assert_eq!(
            Solution::deck_revealed_increasing(vec![3, 1, 2]),
            vec![1, 3, 2]
        );
    }

    #[test]
    fn large_deck_reveals_sorted() {
        let deck: Vec<i32> = (1..=1000).map(|i| i * 997 % 1_000_003).collect();
        let ordered = Solution::deck_revealed_increasing(deck.clone());
        let mut expected = deck;
        expected.sort_unstable();
        assert_eq!(reveal(&ordered), expected);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** run the reveal process on positions instead of cards. Each popped position is the next
one to be revealed, so it receives the next smallest card; then the following position moves to
the back, exactly as a card would.

**Complexity:** `O(n log n)` time for the sort, `O(n)` space.

**Rust concepts:**
- `(0..n).collect::<VecDeque<usize>>()` builds the queue of positions.
- `let Some(pos) = positions.pop_front() else { break; };` takes the next revealed position.
- `if let Some(next) = positions.pop_front() { positions.push_back(next); }` moves a card to the
  bottom only if one is left.
- Writing into `result[pos]` (a pre-filled `vec![0; n]`) places cards out of order.

```rust
use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn deck_revealed_increasing(mut deck: Vec<i32>) -> Vec<i32> {
        deck.sort_unstable();
        let n = deck.len();
        let mut positions: VecDeque<usize> = (0..n).collect();
        let mut result = vec![0; n];
        for card in deck {
            let Some(pos) = positions.pop_front() else {
                break;
            };
            result[pos] = card;
            if let Some(next) = positions.pop_front() {
                positions.push_back(next);
            }
        }
        result
    }
}
```

Full tested solution: [`src/solutions/p0389_reveal_cards_in_increasing_order.rs`](../src/solutions/p0389_reveal_cards_in_increasing_order.rs)

</details>
