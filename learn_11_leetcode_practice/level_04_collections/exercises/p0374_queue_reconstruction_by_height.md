# Problem 0374: Queue Reconstruction by Height

- **Difficulty:** Medium
- **Topics:** two-key `sort_unstable_by` (descending, then ascending), greedy `Vec::insert` at an index
- **Source:** [LeetCode 406. Queue Reconstruction by Height](https://leetcode.com/problems/queue-reconstruction-by-height/)

## Problem

A group of people stood in a queue, but the queue was shuffled. Each person is described by a
pair `[h, k]`: `h` is their height, and `k` is the number of people **in front of them** in the
original queue whose height is **greater than or equal to** `h`.

Given the shuffled list `people`, rebuild the original queue and return it (as a list of the
same `[h, k]` pairs, from the front of the queue to the back). The input always describes a
valid queue.

Implement `Solution::reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>>`.

## Examples

```text
Input:  people = [[7, 0], [4, 4], [7, 1], [5, 0], [6, 1], [5, 2]]
Output: [[5, 0], [7, 0], [5, 2], [6, 1], [4, 4], [7, 1]]
Explanation: [6, 1] has exactly one person of height >= 6 in front of it ([7, 0]);
             [4, 4] has four ([5, 0], [7, 0], [5, 2], [6, 1]); and so on.

Input:  people = [[6, 0], [5, 0], [4, 0], [3, 2], [2, 2], [1, 4]]
Output: [[4, 0], [5, 0], [2, 2], [3, 2], [1, 4], [6, 0]]
```

## Constraints

- `1 <= people.len() <= 2000`
- `0 <= h <= 10^6`, `0 <= k < people.len()`
- The input describes a valid queue.

## Hints

1. Shorter people are "invisible" to the `k` of taller people. So place the **tallest** people
   first.
2. Sort by height descending and, for equal heights, by `k` ascending.
3. Then insert each person at index `k` of the result: everyone already placed is at least as
   tall, so exactly `k` of them end up in front.

## Starter code

Write your code in [`src/practice/p0374_queue_reconstruction_by_height.rs`](../src/practice/p0374_queue_reconstruction_by_height.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0374
```

```rust
pub struct Solution;

impl Solution {
    pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
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

    fn pairs(v: &[[i32; 2]]) -> Vec<Vec<i32>> {
        v.iter().map(|p| p.to_vec()).collect()
    }

    /// Checks that every person sees exactly `k` taller-or-equal people in front.
    fn is_consistent(queue: &[Vec<i32>]) -> bool {
        queue
            .iter()
            .enumerate()
            .all(|(i, p)| queue[..i].iter().filter(|q| q[0] >= p[0]).count() == p[1] as usize)
    }

    #[test]
    fn example_one() {
        let people = pairs(&[[7, 0], [4, 4], [7, 1], [5, 0], [6, 1], [5, 2]]);
        assert_eq!(
            Solution::reconstruct_queue(people),
            pairs(&[[5, 0], [7, 0], [5, 2], [6, 1], [4, 4], [7, 1]])
        );
    }

    #[test]
    fn example_two() {
        let people = pairs(&[[6, 0], [5, 0], [4, 0], [3, 2], [2, 2], [1, 4]]);
        assert_eq!(
            Solution::reconstruct_queue(people),
            pairs(&[[4, 0], [5, 0], [2, 2], [3, 2], [1, 4], [6, 0]])
        );
    }

    #[test]
    fn single_person_and_equal_heights() {
        assert_eq!(
            Solution::reconstruct_queue(pairs(&[[3, 0]])),
            pairs(&[[3, 0]])
        );
        assert_eq!(
            Solution::reconstruct_queue(pairs(&[[5, 2], [5, 0], [5, 1]])),
            pairs(&[[5, 0], [5, 1], [5, 2]])
        );
    }

    #[test]
    fn large_queue_is_consistent() {
        // Build a real queue, compute every k, shuffle, and reconstruct.
        let heights: Vec<i32> = (0..2000).map(|i| (i * 7_919) % 1_000).collect();
        let original: Vec<Vec<i32>> = heights
            .iter()
            .enumerate()
            .map(|(i, &h)| vec![h, heights[..i].iter().filter(|&&x| x >= h).count() as i32])
            .collect();
        let mut shuffled = original.clone();
        shuffled.reverse();
        shuffled.rotate_left(777);
        let rebuilt = Solution::reconstruct_queue(shuffled);
        assert!(is_consistent(&rebuilt));
        assert_eq!(rebuilt, original);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** process people from tallest to shortest (equal heights by increasing `k`) and insert
each one at position `k` of the queue being built. When a person is inserted, all people already
in the queue are at least as tall, so position `k` gives exactly `k` taller-or-equal people in
front. People inserted later are shorter (or equal with larger `k`, inserted behind), so they do
not change the count for anyone already placed.

**Complexity:** `O(n^2)` time (each `insert` shifts up to `n` elements), `O(n)` space.

**Rust concepts:**
- `b[0].cmp(&a[0]).then(a[1].cmp(&b[1]))` sorts by height descending, then `k` ascending.
  `then` (not `then_with`) is fine when the second comparison is cheap.
- `Vec::insert(index, value)` shifts the tail right; it would panic for `index > len`, which a
  valid input never produces.
- Iterating `people` by value moves each inner `Vec` into the result, without cloning.

```rust
pub struct Solution;

impl Solution {
    pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        people.sort_unstable_by(|a, b| b[0].cmp(&a[0]).then(a[1].cmp(&b[1])));
        let mut queue: Vec<Vec<i32>> = Vec::with_capacity(people.len());
        for person in people {
            // Valid input guarantees person[1] <= queue.len().
            let index = person[1] as usize;
            queue.insert(index, person);
        }
        queue
    }
}
```

Full tested solution: [`src/solutions/p0374_queue_reconstruction_by_height.rs`](../src/solutions/p0374_queue_reconstruction_by_height.rs)

</details>
