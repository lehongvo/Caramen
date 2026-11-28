# Problem 0268: Robot Bounded In Circle

- **Difficulty:** Medium
- **Topics:** a heading `enum` with turn methods, a robot `struct` with `&mut self` methods, simulation, reasoning about periodicity
- **Source:** [LeetCode 1041. Robot Bounded In Circle](https://leetcode.com/problems/robot-bounded-in-circle/)

## Problem

A robot stands at `(0, 0)` on an infinite plane, facing **north**. It understands three
instructions:

- `'G'`: go straight one unit;
- `'L'`: turn 90 degrees to the left (counter-clockwise);
- `'R'`: turn 90 degrees to the right (clockwise).

The robot executes the string `instructions` in order and then **repeats it forever**. Return
`true` if the robot stays within some circle around the origin forever (it never wanders off to
infinity).

Implement `Solution::is_robot_bounded(instructions: String) -> bool`.

For this level, write a `Heading` enum (`North`, `East`, `South`, `West`) with `left`/`right`
methods and a `Robot` struct with a `step(&mut self, instruction: char)` method.

## Examples

```text
Input:  instructions = "GGLLGG"
Output: true     (it walks to (0, 2), turns around, walks back to (0, 0), and repeats)

Input:  instructions = "GG"
Output: false    (it keeps going north)

Input:  instructions = "GL"
Output: true     (after 4 repetitions it is back at the origin facing north)
```

## Constraints

- `1 <= instructions.len() <= 100`
- `instructions` contains only `'G'`, `'L'` and `'R'`.

## Hints

1. Simulate one pass only.
2. If the robot is back at the origin after one pass, it is clearly bounded. If it is somewhere
   else but **not facing north**, then after 2 or 4 passes the displacements cancel out.

## Starter code

Write your code in [`src/practice/p0268_robot_bounded_in_circle.rs`](../src/practice/p0268_robot_bounded_in_circle.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0268
```

```rust
pub struct Solution;

impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
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

    fn bounded(s: &str) -> bool {
        Solution::is_robot_bounded(s.to_string())
    }

    #[test]
    fn example_back_and_forth() {
        assert!(bounded("GGLLGG"));
    }

    #[test]
    fn example_straight_line() {
        assert!(!bounded("GG"));
    }

    #[test]
    fn example_square() {
        assert!(bounded("GL"));
    }

    #[test]
    fn turning_in_place() {
        assert!(bounded("L"));
        assert!(bounded("RRRR"));
        assert!(!bounded("GLRG"));
    }

    #[test]
    fn long_instructions() {
        // 99 steps north, then a right turn: drifts in a square pattern, so bounded.
        let s = "G".repeat(99) + "R";
        assert!(bounded(&s));
        // A full turn brings the heading back north after moving: unbounded.
        let t = "G".repeat(96) + "LLLL";
        assert!(!bounded(&t));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** run the instructions once. The robot is bounded exactly when it ends at the origin
or ends facing a direction other than north. If it faces north again, every pass adds the same
non-zero displacement and it drifts away; otherwise the pass's displacement is rotated each
time, and two (U-turn) or four (quarter turn) rotated copies sum to zero.

**Complexity:** `O(n)` time, `O(1)` space.

**Rust concepts:**
- The heading's `delta(self) -> (i32, i32)` keeps the coordinate conventions in one `match`.
- `Robot::new()` spells out the start state (origin, facing north) in one place.
- The final check is a `match` on the robot itself: the struct pattern `Robot { x: 0, y: 0, .. }`
  means "at the origin", and the second arm binds only the `heading` field.

```rust
pub struct Solution;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Heading {
    North,
    East,
    South,
    West,
}

impl Heading {
    fn right(self) -> Heading {
        match self {
            Heading::North => Heading::East,
            Heading::East => Heading::South,
            Heading::South => Heading::West,
            Heading::West => Heading::North,
        }
    }

    fn left(self) -> Heading {
        self.right().right().right()
    }

    fn delta(self) -> (i32, i32) {
        match self {
            Heading::North => (0, 1),
            Heading::East => (1, 0),
            Heading::South => (0, -1),
            Heading::West => (-1, 0),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    x: i32,
    y: i32,
    heading: Heading,
}

impl Robot {
    fn new() -> Robot {
        Robot {
            x: 0,
            y: 0,
            heading: Heading::North,
        }
    }

    fn step(&mut self, instruction: char) {
        match instruction {
            'G' => {
                let (dx, dy) = self.heading.delta();
                self.x += dx;
                self.y += dy;
            }
            'L' => self.heading = self.heading.left(),
            'R' => self.heading = self.heading.right(),
            _ => {}
        }
    }
}

impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
        let mut robot = Robot::new();
        for c in instructions.chars() {
            robot.step(c);
        }
        match robot {
            Robot { x: 0, y: 0, .. } => true,
            Robot { heading, .. } => heading != Heading::North,
        }
    }
}
```

Full tested solution: [`src/solutions/p0268_robot_bounded_in_circle.rs`](../src/solutions/p0268_robot_bounded_in_circle.rs)

</details>
