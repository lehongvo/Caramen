//! # Problem 0745: Asteroid Collision
//!
//! - **Difficulty:** Medium
//! - **Topics:** stack simulation, `match` with guards on `last()`, `Ordering`
//! - **Source:** [LeetCode 735. Asteroid Collision](https://leetcode.com/problems/asteroid-collision/)
//!
//! ## Problem
//!
//! A row of asteroids is given by `asteroids`. The absolute value of each number is the asteroid's
//! size, and its sign is its direction: positive moves right, negative moves left. All asteroids
//! move at the same speed.
//!
//! When two asteroids meet, the smaller one explodes; if they have the same size, both explode.
//! Asteroids moving in the same direction never meet. Return the asteroids that remain after all
//! collisions, in their original left-to-right order.
//!
//! Implement `Solution::asteroid_collision(asteroids: Vec<i32>) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  asteroids = [5, 10, -5]
//! Output: [5, 10]       (-5 hits 10 and explodes)
//!
//! Input:  asteroids = [8, -8]
//! Output: []            (same size: both explode)
//!
//! Input:  asteroids = [10, 2, -5]
//! Output: [10]          (-5 destroys 2, then 10 destroys -5)
//!
//! Input:  asteroids = [-2, -1, 1, 2]
//! Output: [-2, -1, 1, 2] (the left movers are already left of the right movers)
//! ```
//!
//! ## Constraints
//!
//! - `2 <= asteroids.len() <= 10^4`
//! - `-1000 <= asteroids[i] <= 1000` and `asteroids[i] != 0`
//!
//! ## Hints
//!
//! 1. A collision only happens when a left-moving asteroid arrives after a right-moving one.
//! 2. Keep the survivors so far on a stack. A new left mover fights the right movers on top of
//!    the stack, one at a time.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** stack simulation: the stack holds the survivors so far (in order). Right movers and
//! left movers that meet nothing are simply pushed. A left mover `a` repeatedly looks at the top:
//! if it is a right mover, they collide. If the top is smaller than `|a|`, it explodes (pop) and
//! `a` continues; if equal, both explode; if larger, `a` explodes. When the top is a left mover or
//! the stack is empty, `a` survives and is pushed. Each asteroid is pushed and popped at most once.
//!
//! **Complexity:** `O(n)` time, `O(n)` space.
//!
//! **Rust concepts:**
//! - `match stack.last() { Some(&top) if top > 0 => ..., _ => break }` combines the "is there a
//!   top?" and "does it move right?" checks with a match guard.
//! - `top.cmp(&-a)` returns an `Ordering`, and the nested `match` lists the three collision
//!   outcomes.
// @solution-end

use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        // @solution-start
        let mut stack: Vec<i32> = Vec::with_capacity(asteroids.len());
        for &a in &asteroids {
            let mut alive = true;
            while alive && a < 0 {
                match stack.last() {
                    Some(&top) if top > 0 => match top.cmp(&-a) {
                        Ordering::Less => {
                            stack.pop(); // the right mover explodes, `a` keeps going
                        }
                        Ordering::Equal => {
                            stack.pop();
                            alive = false;
                        }
                        Ordering::Greater => alive = false,
                    },
                    _ => break, // nothing moving right in front of `a`
                }
            }
            if alive {
                stack.push(a);
            }
        }
        stack
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_small_one_explodes() {
        assert_eq!(Solution::asteroid_collision(vec![5, 10, -5]), vec![5, 10]);
    }

    #[test]
    fn example_both_explode() {
        assert!(Solution::asteroid_collision(vec![8, -8]).is_empty());
    }

    #[test]
    fn example_chain() {
        assert_eq!(Solution::asteroid_collision(vec![10, 2, -5]), vec![10]);
    }

    #[test]
    fn example_no_collision() {
        assert_eq!(
            Solution::asteroid_collision(vec![-2, -1, 1, 2]),
            vec![-2, -1, 1, 2]
        );
    }

    #[test]
    fn left_mover_destroys_everything() {
        assert_eq!(
            Solution::asteroid_collision(vec![1, -2, -2, -2]),
            vec![-2, -2, -2]
        );
        assert_eq!(
            Solution::asteroid_collision(vec![1, 2, 3, -10, 4]),
            vec![-10, 4]
        );
    }

    #[test]
    fn long_input() {
        let mut asteroids = vec![1; 5_000];
        asteroids.extend(vec![-2; 5_000]);
        assert_eq!(Solution::asteroid_collision(asteroids), vec![-2; 5_000]);
    }
}
