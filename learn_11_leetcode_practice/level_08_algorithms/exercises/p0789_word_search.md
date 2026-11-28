# Problem 0789: Word Search

- **Difficulty:** Medium
- **Topics:** DFS with backtracking on a grid, temporary marking and restoring, pruning, `Iterator::any`
- **Source:** [LeetCode 79. Word Search](https://leetcode.com/problems/word-search/)

## Problem

Given an `m x n` grid of letters `board` and a string `word`, return `true` if `word` can be
spelled by a path of cells where each next cell is **horizontally or vertically** adjacent to
the previous one. A cell may be used **at most once** in the path.

Implement `Solution::exist(board: Vec<Vec<char>>, word: String) -> bool`.

## Examples

```text
board = [['A', 'B', 'C', 'E'],
         ['S', 'F', 'C', 'S'],
         ['A', 'D', 'E', 'E']]

Input:  word = "ABCCED"
Output: true

Input:  word = "SEE"
Output: true

Input:  word = "ABCB"
Output: false         (the path would need the 'B' twice)
```

## Constraints

- `1 <= m, n <= 6`
- `1 <= word.len() <= 15`
- `board` and `word` contain only uppercase and lowercase English letters.

## Hints

1. Try every cell as a start. From a cell that matches the current letter, try the four
   neighbours for the next letter.
2. To prevent reusing a cell, overwrite it with a marker before going deeper and **restore** it
   afterwards (backtracking).
3. Cheap pruning: if the board does not contain enough copies of some letter, answer `false`
   immediately.

## Starter code

Write your code in [`src/practice/p0789_word_search.rs`](../src/practice/p0789_word_search.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0789
```

```rust
pub struct Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
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

    fn sample() -> Vec<Vec<char>> {
        ["ABCE", "SFCS", "ADEE"]
            .iter()
            .map(|row| row.chars().collect())
            .collect()
    }

    fn run(board: Vec<Vec<char>>, word: &str) -> bool {
        Solution::exist(board, word.to_string())
    }

    #[test]
    fn example_long_path() {
        assert!(run(sample(), "ABCCED"));
    }

    #[test]
    fn example_short_path() {
        assert!(run(sample(), "SEE"));
    }

    #[test]
    fn example_cell_reuse_is_not_allowed() {
        assert!(!run(sample(), "ABCB"));
    }

    #[test]
    fn single_cell_board() {
        assert!(run(vec![vec!['a']], "a"));
        assert!(!run(vec![vec!['a']], "b"));
        assert!(!run(vec![vec!['a']], "aa"));
    }

    #[test]
    fn letters_are_case_sensitive() {
        assert!(!run(vec![vec!['a', 'B']], "ab"));
        assert!(run(vec![vec!['a', 'B']], "aB"));
    }

    #[test]
    fn marking_is_undone_between_attempts() {
        // From the 'C' the search first goes down to the 'E' at (1, 2) and fails after using the
        // three lower-right 'E's; the real path needs those cells again, so they must be restored.
        let board: Vec<Vec<char>> = ["ABCE", "SFES", "ADEE"]
            .iter()
            .map(|r| r.chars().collect())
            .collect();
        assert!(run(board, "ABCESEEEFS"));
    }

    #[test]
    fn worst_case_board_is_fast_thanks_to_pruning() {
        let mut board = vec![vec!['A'; 6]; 6];
        assert!(!run(board.clone(), "AAAAAAAAAAAAAAB")); // no 'B' at all
        board[5][5] = 'B';
        assert!(run(board.clone(), "AAAAAAAAAAAAAAB")); // searched from the single 'B'
        assert!(!run(board, "ABBBB"));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** DFS + backtracking: `dfs(r, c, rest)` checks whether `rest` can be spelled starting
at `(r, c)`. If the cell does not match `rest[0]`, fail. If it is the last letter, succeed.
Otherwise replace the cell by `'#'` (so the path cannot come back to it), try the four
neighbours with `rest[1..]`, and **restore** the letter before returning, so that other paths
can still use the cell. That undo step is what makes it backtracking.

Two prunings keep the worst case small: give up when some letter of the word occurs more often
than on the board, and search the word **reversed** when its last letter is rarer on the board
than its first letter (fewer starting points, the same answer).

**Complexity:** `O(m * n * 3^L)` time in the worst case for a word of length `L` (after the first
step each cell has at most 3 unvisited neighbours), `O(L)` recursion depth.

**Rust concepts:**
- `[(-1, 0), ...].into_iter().any(|(dr, dc)| ...)` stops at the first successful direction;
  the closure borrows `board` mutably for the recursive call, which `any` allows (it takes
  an `FnMut`).
- `match (r.checked_add_signed(dr), c.checked_add_signed(dc)) { (Some(nr), Some(nc)) if ... => ... }`
  combines the bounds checks in one pattern with a guard.
- `word: &[char]` and `&word[1..]` pass the rest of the word without copying.

```rust
pub struct Solution;

/// Can `word` be spelled starting at `(r, c)` without reusing cells?
fn dfs(board: &mut [Vec<char>], word: &[char], r: usize, c: usize) -> bool {
    if board[r][c] != word[0] {
        return false;
    }
    if word.len() == 1 {
        return true;
    }
    let letter = board[r][c];
    board[r][c] = '#'; // in use by the current path
    let found = [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .any(
            |(dr, dc)| match (r.checked_add_signed(dr), c.checked_add_signed(dc)) {
                (Some(nr), Some(nc)) if nr < board.len() && nc < board[0].len() => {
                    dfs(board, &word[1..], nr, nc)
                }
                _ => false,
            },
        );
    board[r][c] = letter; // backtrack
    found
}

fn count(chars: impl Iterator<Item = char>, target: char) -> usize {
    chars.filter(|&ch| ch == target).count()
}

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut board = board;
        let mut word: Vec<char> = word.chars().collect();
        // Pruning 1: the board must contain every letter often enough.
        let on_board = |ch: char| count(board.iter().flatten().copied(), ch);
        if word
            .iter()
            .any(|&ch| count(word.iter().copied(), ch) > on_board(ch))
        {
            return false;
        }
        // Pruning 2: start from the rarer end of the word.
        if on_board(word[0]) > on_board(word[word.len() - 1]) {
            word.reverse();
        }
        let (m, n) = (board.len(), board[0].len());
        (0..m).any(|r| (0..n).any(|c| dfs(&mut board, &word, r, c)))
    }
}
```

Full tested solution: [`src/solutions/p0789_word_search.rs`](../src/solutions/p0789_word_search.rs)

</details>
