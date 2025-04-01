//! https://leetcode.com/problems/word-search/
//! Medium - vec![array, string, backtracking, depth-first-search, matrix]
//!
//! Given an `m x n` grid of characters board and a string word, return `true`
//! if word exists in the grid.
//! The word can be constructed from letters of sequentially adjacent cells,
//! where adjacent cells are horizontally or vertically neighboring.
//! The same letter cell may not be used more than once.
//!
//! Example 1:
//! Input: `board = [['A','B','C','E'],['S','F','C','S'],['A','D','E','E']]`, `word = "ABCCED"`
//! Output: `true`
//!
//! Example 2:
//! Input: board = `[['A','B','C','E'],['S','F','C','S'],['A','D','E','E']]`, `word = "SEE"`
//! Output: `true`
//! Example 3:
//! Input: `board = [['A','B','C','E'],['S','F','C','S'],['A','D','E','E']]`, `word = "ABCB"`
//! Output: `false`
//!
//! Constraints:
//! - `m == board.length`
//! - `n = board[i].length`
//! - `1 <= m, n <= 6`
//! - `1 <= word.length <= 15`
//! - board and word consists of only lowercase and uppercase English letters.
//!
//! Follow up: Could you use search pruning to make your solution faster with a larger board?

pub type Coord = (usize, usize);
pub type Alphabet = [usize; 26];

pub struct Solution {}

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        Self::exist_ref(&board, &word)
    }

    pub fn exist_ref(board: &[Vec<char>], word: &str) -> bool {
        // Break if the board does not have a correct letter distribution.
        let word_chars = word.chars().collect::<Vec<char>>();
        if !Self::check_char_count(&word_chars, board) {
            return false;
        }

        let mut visited = vec![vec![false; board[0].len()]; board.len()];
        let mut current = 0;
        let mut found = false;

        for row in 0..board.len() {
            for col in 0..board[0].len() {
                if found {
                    return true;
                }
                Self::exist_helper(
                    &(row, col),
                    &mut current,
                    &mut visited,
                    &mut found,
                    board,
                    &word_chars,
                );
            }
        }

        found
    }

    fn count_chars(word: &[char]) -> (Alphabet, Alphabet) {
        let mut lowercase_counts = [0; 26];
        let mut uppercase_counts = [0; 26];
        for &ch in word {
            if ch.is_lowercase() {
                lowercase_counts[ch as usize - 'a' as usize] += 1;
            }
            if ch.is_uppercase() {
                uppercase_counts[ch as usize - 'A' as usize] += 1;
            }
        }
        (lowercase_counts, uppercase_counts)
    }

    fn check_char_count(word: &[char], board: &[Vec<char>]) -> bool {
        let word_counts = Self::count_chars(word);
        let mut board_str = Vec::with_capacity(board.len() * board[0].len());
        for line in board {
            board_str.extend(line);
        }
        let board_counts = Self::count_chars(&board_str);
        for i in 0..26 {
            if word_counts.0[i] > board_counts.0[i] || word_counts.1[i] > board_counts.1[i] {
                return false;
            }
        }
        true
    }

    fn exist_helper(
        coord: &Coord,
        current: &mut usize,
        visited: &mut [Vec<bool>],
        found: &mut bool,
        board: &[Vec<char>],
        word: &[char],
    ) {
        if *found {
            return;
        }
        if visited[coord.0][coord.1] {
            return;
        }
        visited[coord.0][coord.1] = true;

        if word[*current] == board[coord.0][coord.1] {
            *current += 1;
        } else {
            // Backtracking from a wrong branch does not require subtracting
            // from current because we never added to it in the first place.
            visited[coord.0][coord.1] = false;
            return;
        }
        if *current == word.len() {
            *found = true;
            return;
        }

        // Recursion.
        for neighbor in Self::get_neighbors(coord, board) {
            Self::exist_helper(&neighbor, current, visited, found, board, word)
        }
        // Backtracking from a correct branch DOES require updating the length.
        visited[coord.0][coord.1] = false;
        *current = current.saturating_sub(1);
    }

    fn get_neighbors(coord: &Coord, board: &[Vec<char>]) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::with_capacity(4);
        if coord.0 > 0 {
            coords.push((coord.0 - 1, coord.1))
        }
        if coord.0 < board.len() - 1 {
            coords.push((coord.0 + 1, coord.1))
        }
        if coord.1 > 0 {
            coords.push((coord.0, coord.1 - 1))
        }
        if coord.1 < board[0].len() - 1 {
            coords.push((coord.0, coord.1 + 1))
        }
        coords
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0079() {
        let matrix = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        assert!(Solution::exist_ref(&matrix, "ABCCED"),);
        assert!(Solution::exist_ref(&matrix, "SEE"),);
        assert!(!Solution::exist_ref(&matrix, "ABCB"),);

        assert!(Solution::exist_ref(
            &[vec!['a', 'b'], vec!['c', 'd']],
            "cdba"
        ))
    }
}
