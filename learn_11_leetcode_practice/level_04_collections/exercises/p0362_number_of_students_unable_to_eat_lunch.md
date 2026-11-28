# Problem 0362: Number of Students Unable to Eat Lunch

- **Difficulty:** Easy
- **Topics:** `VecDeque` simulation, detecting a stalled queue, counting as a shortcut
- **Source:** [LeetCode 1700. Number of Students Unable to Eat Lunch](https://leetcode.com/problems/number-of-students-unable-to-eat-lunch/)

## Problem

A cafeteria serves two kinds of sandwiches, type `0` and type `1`. The students wait in a queue;
`students[i]` is the kind preferred by the `i`-th student from the front. The sandwiches are in a
stack; `sandwiches[0]` is the one on **top**. There are as many sandwiches as students.

Repeatedly, the student at the front of the queue looks at the top sandwich:

- if it is the kind they prefer, they take it and leave the queue;
- otherwise they go to the **back** of the queue.

This stops when no student in the queue wants the top sandwich. Return the number of students
who cannot eat.

Implement `Solution::count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32`.

## Examples

```text
Input:  students = [1, 1, 0, 0], sandwiches = [0, 1, 0, 1]
Output: 0     (after some rotations, every student gets a sandwich)

Input:  students = [1, 1, 1, 0, 0, 1], sandwiches = [1, 0, 0, 0, 1, 1]
Output: 3     (when the top sandwich is 0, only students preferring 1 remain)
```

## Constraints

- `1 <= students.len() == sandwiches.len() <= 100`
- Every value is `0` or `1`.

## Hints

1. Simulate with a `VecDeque` for the queue and an index for the top of the sandwich stack.
2. The process is stuck when every student in the queue has refused the current sandwich in a
   row: count consecutive refusals.
3. Shortcut: the order of the queue never matters in the end, only how many students want each
   kind.

## Starter code

Write your code in [`src/practice/p0362_number_of_students_unable_to_eat_lunch.rs`](../src/practice/p0362_number_of_students_unable_to_eat_lunch.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0362
```

```rust
use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
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
            Solution::count_students(vec![1, 1, 0, 0], vec![0, 1, 0, 1]),
            0
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::count_students(vec![1, 1, 1, 0, 0, 1], vec![1, 0, 0, 0, 1, 1]),
            3
        );
    }

    #[test]
    fn single_student() {
        assert_eq!(Solution::count_students(vec![0], vec![0]), 0);
        assert_eq!(Solution::count_students(vec![0], vec![1]), 1);
    }

    #[test]
    fn hundred_students() {
        // 50 want 0 and 50 want 1, but the first 51 sandwiches are 0: one 0 too many.
        let students: Vec<i32> = (0..100).map(|i| i % 2).collect();
        let mut sandwiches = vec![0; 51];
        sandwiches.extend(vec![1; 49]);
        assert_eq!(Solution::count_students(students, sandwiches), 50);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** simulate. Keep the students in a `VecDeque` and an index `top` into `sandwiches`. If
the front student matches, both leave and the refusal counter resets; otherwise the student
rotates to the back and the counter grows. When the counter equals the queue length, everyone has
refused the current sandwich and nothing can change any more.

**Complexity:** `O(n^2)` time in the worst case (each served sandwich can cost a full rotation),
`O(n)` space.

The counting shortcut is `O(n)`: count how many students want `0` and `1`, then walk the
sandwiches; the first sandwich nobody remaining wants stops the line, and the students left over
are the answer.

**Rust concepts:**
- `VecDeque::from(students)` reuses the vector's buffer for the queue.
- `pop_front` + `push_back` rotates one student.
- `while refusals < queue.len()` stops both when the queue is empty and when it is stuck.

```rust
use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut queue = VecDeque::from(students);
        let mut top = 0;
        let mut refusals = 0;
        while refusals < queue.len() {
            let Some(student) = queue.pop_front() else {
                break;
            };
            if sandwiches.get(top) == Some(&student) {
                top += 1;
                refusals = 0;
            } else {
                queue.push_back(student);
                refusals += 1;
            }
        }
        queue.len() as i32
    }
}
```

Full tested solution: [`src/solutions/p0362_number_of_students_unable_to_eat_lunch.rs`](../src/solutions/p0362_number_of_students_unable_to_eat_lunch.rs)

</details>
