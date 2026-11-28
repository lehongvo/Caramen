//! # Problem 0988: Word Search II
//!
//! - **Difficulty:** Hard
//! - **Topics:** trie + grid DFS, marking visited cells in place, pruning found words, arena-allocated trie nodes
//! - **Source:** [LeetCode 212. Word Search II](https://leetcode.com/problems/word-search-ii/)
//!
//! ## Problem
//!
//! Given an `m x n` board of lowercase letters and a list of distinct `words`, return every word that
//! can be traced on the board, in any order. A word is traced through **sequentially adjacent**
//! cells (horizontally or vertically neighbouring), and one cell may not be used twice in the same
//! word.
//!
//! Implement `Solution::find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  board = ["oaan",
//!                  "etae",
//!                  "ihkr",
//!                  "iflv"],  words = ["oath", "pea", "eat", "rain"]
//! Output: ["eat", "oath"]
//!
//! Input:  board = ["ab", "cd"], words = ["abcb"]
//! Output: []
//! ```
//!
//! ## Constraints
//!
//! - `1 <= m, n <= 12`
//! - `1 <= words.len() <= 3 * 10^4`, `1 <= words[i].len() <= 10`, all words distinct.
//!
//! ## Hints
//!
//! 1. Searching each word separately repeats a lot of work. Put all the words in a trie and run one
//!    DFS from every cell, following the trie as you walk.
//! 2. Store the index of the word at the node where it ends; when the DFS reaches such a node, record
//!    the word and clear the index so it is reported once.
//! 3. Temporarily replace the current cell with `'#'` to mark it visited, and restore it afterwards.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** build an arena trie of the words. From each cell, walk the board and the trie together;
//! a missing trie child ends the branch immediately, which is what makes this fast. When a node with
//! a word index is reached, the word is added to the output and its index removed (so duplicates are
//! impossible).
//!
//! **Complexity:** `O(m * n * 4 * 3^(L - 1))` in the worst case for word length `L`, usually much less
//! thanks to the trie; `O(total letters)` space.
//!
//! **Rust concepts:**
//! - The board is taken by value, so it can be mutated for the visited marks.
//! - `Option<usize>::take()` both reads and clears the stored word index.
//! - Arena nodes with `[usize; 26]` children (0 = none) avoid `Box` recursion.
// @solution-end

pub struct Solution;

// @solution-start remove
struct Node {
    children: [usize; 26],
    word: Option<usize>,
}

fn dfs(
    board: &mut [Vec<char>],
    r: usize,
    c: usize,
    node: usize,
    nodes: &mut [Node],
    words: &[String],
    found: &mut Vec<String>,
) {
    let ch = board[r][c];
    if ch == '#' {
        return;
    }
    let next = nodes[node].children[(ch as u8 - b'a') as usize];
    if next == 0 {
        return;
    }
    if let Some(index) = nodes[next].word.take() {
        found.push(words[index].clone());
    }
    board[r][c] = '#';
    if r > 0 {
        dfs(board, r - 1, c, next, nodes, words, found);
    }
    if r + 1 < board.len() {
        dfs(board, r + 1, c, next, nodes, words, found);
    }
    if c > 0 {
        dfs(board, r, c - 1, next, nodes, words, found);
    }
    if c + 1 < board[0].len() {
        dfs(board, r, c + 1, next, nodes, words, found);
    }
    board[r][c] = ch;
}
// @solution-end

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        // @solution-start
        let mut board = board;
        let mut nodes = vec![Node {
            children: [0; 26],
            word: None,
        }];
        for (index, word) in words.iter().enumerate() {
            let mut current = 0;
            for b in word.bytes() {
                let slot = usize::from(b - b'a');
                if nodes[current].children[slot] == 0 {
                    nodes.push(Node {
                        children: [0; 26],
                        word: None,
                    });
                    nodes[current].children[slot] = nodes.len() - 1;
                }
                current = nodes[current].children[slot];
            }
            nodes[current].word = Some(index);
        }
        let mut found = Vec::new();
        for r in 0..board.len() {
            for c in 0..board[0].len() {
                dfs(&mut board, r, c, 0, &mut nodes, &words, &mut found);
            }
        }
        found
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(board: &[&str], words: &[&str]) -> Vec<String> {
        let board = board.iter().map(|r| r.chars().collect()).collect();
        let mut found = Solution::find_words(board, words.iter().map(|w| w.to_string()).collect());
        found.sort();
        found
    }

    #[test]
    fn examples() {
        assert_eq!(
            run(
                &["oaan", "etae", "ihkr", "iflv"],
                &["oath", "pea", "eat", "rain"]
            ),
            ["eat", "oath"]
        );
        assert!(run(&["ab", "cd"], &["abcb"]).is_empty());
    }

    #[test]
    fn cells_are_not_reused_but_paths_turn() {
        assert_eq!(run(&["a"], &["a", "aa"]), ["a"]);
        assert_eq!(
            run(&["ab", "dc"], &["abcd", "adcb", "acdb", "ba"]),
            ["abcd", "adcb", "ba"]
        );
    }

    #[test]
    fn prefixes_of_each_other() {
        assert_eq!(
            run(
                &["abc", "aed", "afg"],
                &["abcdefg", "gfedcbaaa", "eaabcdgfa", "befa", "dgc", "ade"]
            ),
            ["abcdefg", "befa", "eaabcdgfa", "gfedcbaaa"]
        );
    }

    #[test]
    fn big_board_of_one_letter() {
        let rows = vec!["a".repeat(12); 12];
        let refs: Vec<&str> = rows.iter().map(String::as_str).collect();
        let words: Vec<String> = (1..=10)
            .map(|k| "a".repeat(k))
            .chain(["ab".to_string()])
            .collect();
        let word_refs: Vec<&str> = words.iter().map(String::as_str).collect();
        let found = run(&refs, &word_refs);
        assert_eq!(found.len(), 10);
        assert!(!found.contains(&"ab".to_string()));
    }
}
