# Problem 0846: Course Schedule

- **Difficulty:** Medium
- **Topics:** cycle detection in a directed graph, Kahn's algorithm (topological sort by
  in-degree), `VecDeque`
- **Source:** [LeetCode 207. Course Schedule](https://leetcode.com/problems/course-schedule/)

## Problem

There are `num_courses` courses labelled `0..num_courses`. `prerequisites[i] = [a, b]` means
you must finish course `b` before you can take course `a`. Return `true` if it is possible to
finish all the courses, and `false` otherwise.

Implement `Solution::can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool`.

## Examples

```text
Input:  num_courses = 2, prerequisites = [[1, 0]]
Output: true
Explanation: take course 0, then course 1.

Input:  num_courses = 2, prerequisites = [[1, 0], [0, 1]]
Output: false
Explanation: each course requires the other one first.
```

## Constraints

- `1 <= num_courses <= 2000`
- `0 <= prerequisites.len() <= 5000`, `prerequisites[i].len() == 2`
- `0 <= a, b < num_courses`; all pairs are distinct.

## Hints

1. Draw an edge `b -> a` for every pair. All courses can be finished exactly when this directed
   graph has no cycle.
2. A course with no remaining prerequisites (in-degree `0`) can be taken now. Taking it lowers
   the in-degree of the courses that depend on it.
3. If you can take every course this way, there is no cycle.

## Starter code

Write your code in [`src/practice/p0846_course_schedule.rs`](../src/practice/p0846_course_schedule.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0846
```

```rust
use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
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

    fn run(n: i32, pre: &[[i32; 2]]) -> bool {
        Solution::can_finish(n, pre.iter().map(|p| p.to_vec()).collect())
    }

    #[test]
    fn example_possible() {
        assert!(run(2, &[[1, 0]]));
    }

    #[test]
    fn example_two_cycle() {
        assert!(!run(2, &[[1, 0], [0, 1]]));
    }

    #[test]
    fn no_prerequisites() {
        assert!(run(1, &[]));
        assert!(run(5, &[]));
    }

    #[test]
    fn cycle_hidden_in_a_bigger_graph() {
        // 0 -> 1 -> 2 is fine, but 3 -> 4 -> 5 -> 3 is a cycle.
        assert!(!run(6, &[[1, 0], [2, 1], [4, 3], [5, 4], [3, 5]]));
        assert!(run(6, &[[1, 0], [2, 1], [4, 3], [5, 4], [5, 0]]));
    }

    #[test]
    fn long_chain() {
        let pre: Vec<[i32; 2]> = (1..2000).map(|c| [c, c - 1]).collect();
        assert!(run(2000, &pre));
        let mut with_cycle = pre.clone();
        with_cycle.push([0, 1999]);
        assert!(!run(2000, &with_cycle));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** Kahn's algorithm. Compute every in-degree, queue the courses with in-degree `0`, and
repeatedly "take" a queued course: count it and decrement the in-degree of each dependent,
queueing those that reach `0`. Courses on a cycle never reach in-degree `0`, so the count equals
`num_courses` exactly when there is no cycle.

**Complexity:** `O(V + E)` time and space.

**Rust concepts:**
- `(0..n).filter(|&c| indegree[c] == 0).collect::<VecDeque<_>>()` builds the initial queue.
- Kahn's algorithm is iterative, so a long chain of prerequisites cannot overflow the stack.

```rust
use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let n = num_courses as usize;
        let mut next: Vec<Vec<usize>> = vec![Vec::new(); n];
        let mut indegree = vec![0_usize; n];
        for p in &prerequisites {
            let (course, before) = (p[0] as usize, p[1] as usize);
            next[before].push(course);
            indegree[course] += 1;
        }
        let mut ready: VecDeque<usize> = (0..n).filter(|&c| indegree[c] == 0).collect();
        let mut taken = 0;
        while let Some(c) = ready.pop_front() {
            taken += 1;
            for &d in &next[c] {
                indegree[d] -= 1;
                if indegree[d] == 0 {
                    ready.push_back(d);
                }
            }
        }
        taken == n
    }
}
```

Full tested solution: [`src/solutions/p0846_course_schedule.rs`](../src/solutions/p0846_course_schedule.rs)

</details>
