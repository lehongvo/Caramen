# Problem 0755: Assign Cookies

- **Difficulty:** Easy
- **Topics:** greedy matching, sorting both inputs, two pointers
- **Source:** [LeetCode 455. Assign Cookies](https://leetcode.com/problems/assign-cookies/)

## Problem

Each child `i` has a greed factor `g[i]`: the smallest cookie size that makes them happy. Each
cookie `j` has a size `s[j]`. Child `i` is content when they get one cookie with
`s[j] >= g[i]`. Every child gets at most one cookie and every cookie goes to at most one child.

Return the maximum number of content children.

Implement `Solution::find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32`.

## Examples

```text
Input:  g = [1, 2, 3], s = [1, 1]
Output: 1             (both cookies only satisfy the child with greed 1)

Input:  g = [1, 2], s = [1, 2, 3]
Output: 2
```

## Constraints

- `1 <= g.len() <= 3 * 10^4`
- `0 <= s.len() <= 3 * 10^4`
- `1 <= g[i], s[j] <= 2^31 - 1`

## Hints

1. The least greedy child is the easiest to satisfy. Which cookie should they get?
2. Sort both lists and walk through the cookies from small to large.

## Starter code

Write your code in [`src/practice/p0755_assign_cookies.rs`](../src/practice/p0755_assign_cookies.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0755
```

```rust
pub struct Solution;

impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
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
    fn example_one_content_child() {
        assert_eq!(
            Solution::find_content_children(vec![1, 2, 3], vec![1, 1]),
            1
        );
    }

    #[test]
    fn example_everyone_content() {
        assert_eq!(
            Solution::find_content_children(vec![1, 2], vec![1, 2, 3]),
            2
        );
    }

    #[test]
    fn no_cookies() {
        assert_eq!(Solution::find_content_children(vec![1], vec![]), 0);
    }

    #[test]
    fn unsorted_inputs() {
        assert_eq!(
            Solution::find_content_children(vec![10, 9, 8, 7], vec![5, 6, 7, 8]),
            2
        );
    }

    #[test]
    fn huge_values() {
        assert_eq!(
            Solution::find_content_children(vec![i32::MAX, 1], vec![i32::MAX - 1, i32::MAX]),
            2
        );
    }

    #[test]
    fn large_input() {
        let g: Vec<i32> = (1..=30_000).collect();
        let s: Vec<i32> = (1..=30_000).map(|x| x * 2).collect();
        // Walking up the sorted lists, cookie 2k goes to the child with greed k.
        assert_eq!(Solution::find_content_children(g, s), 30_000);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** greedy matching: sort greed factors and cookie sizes. Go through the cookies from
smallest to largest. If the current cookie satisfies the least greedy child still waiting, give
it to them; otherwise the cookie is too small for **every** waiting child and is thrown away.
Giving the smallest sufficient cookie to the least greedy child never hurts: any larger cookie
is at least as useful for the greedier children.

**Complexity:** `O(n log n + m log m)` time for the sorts, `O(1)` extra space.

**Rust concepts:**
- `g.get(child)` returns `None` once every child is content, so the loop needs no separate
  bounds check.
- `let mut g = g;` re-binds the owned parameter as mutable to sort it in place.

```rust
pub struct Solution;

impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let (mut g, mut s) = (g, s);
        g.sort_unstable();
        s.sort_unstable();
        let mut child = 0; // the least greedy child still waiting
        for &cookie in &s {
            if g.get(child).is_some_and(|&greed| cookie >= greed) {
                child += 1;
            }
        }
        child as i32
    }
}
```

Full tested solution: [`src/solutions/p0755_assign_cookies.rs`](../src/solutions/p0755_assign_cookies.rs)

</details>
