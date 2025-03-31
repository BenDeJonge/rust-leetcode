//! https://leetcode.com/problems/edit-distance/
//! Medium - [string, dynamic-programming]
//!
//! Given two strings word1 and word2, return the minimum number of operations required to convert word1 to word2.
//! You have the following three operations permitted on a word:
//! - Insert a character
//! - Delete a character
//! - Replace a character
//!
//! Example 1:
//! Input: `word1 = "horse", word2 = "ros"`
//! Output: 3
//! Explanation:
//! - horse -> rorse (replace `h` with `r`)
//! - rorse -> rose (remove `r`)
//! - rose -> ros (remove `e`)
//!
//! Example 2:
//! Input: `word1 = "intention", word2 = "execution"`
//! Output: 5
//! Explanation:
//! - intention -> inention (remove `t`)
//! - inention -> enention (replace `i` with `e`)
//! - enention -> exention (replace `n` with `x`)
//! - exention -> exection (replace `n` with `c`)
//! - exection -> execution (insert `u`)
//!
//! Constraints:
//! - `0 <= word1.length, word2.length <= 500`
//! - `word1` and `word2` consist of lowercase English letters.`

pub struct Solution {}

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        Self::min_distance_dp_iterative_matrix(&word1, &word2)
            .try_into()
            .unwrap()
    }

    /// Solves in time and memory complexity:
    /// - `O_t(m*n)`: with `m` and `n` the length of the words.
    /// - `O_m(min(m,n))`: with `m` and `n` the length of the words.
    ///
    /// This is basically the optimal implementation of the [Levenshtein distance](
    /// https://en.wikipedia.org/wiki/Levenshtein_distance#Iterative_with_full_matrix
    /// ).
    fn min_distance_dp_iterative_matrix(word1: &str, word2: &str) -> usize {
        // For minimal memory usage, link the matrix rows to the shorter string.
        if word2.len() > word1.len() {
            return Self::min_distance_dp_iterative_matrix(word2, word1);
        }

        // This limits both words to UTF-8.
        let chars1 = word1.as_bytes();
        let chars2 = word2.as_bytes();

        // Rather than constructing the whole distance matrix M a priori,
        // only the required two rows for each iteration are generated.
        let mut previous_row: Vec<usize> = (0..=word2.len()).collect();
        let mut current_row: Vec<usize> = Vec::with_capacity(word2.len() + 1);

        for row in 1..=chars1.len() {
            // Prepend the first column with the index.
            // This represents adding to an empty word2.
            current_row.push(row);
            let c1 = chars1[row - 1];
            for col in 1..=chars2.len() {
                let c2 = chars2[col - 1];
                let substitution_cost = (c1 != c2) as usize;
                current_row.push(
                    [
                        // Previous row -> shorten word1 -> deletion.
                        previous_row[col] + 1,
                        // Previous column -> shorten word2 -> insertion.
                        current_row[col - 1] + 1,
                        // Previous row and col -> shorten both -> substitution.
                        previous_row[col - 1] + substitution_cost,
                    ]
                    .into_iter()
                    .min()
                    .unwrap(),
                );
            }
            previous_row = std::mem::take(&mut current_row);
        }

        *previous_row.last().unwrap()
    }

    fn min_distance_dp_complete_matrix(word1: &str, word2: &str) -> usize {
        // This limits both words to UTF-8.
        let chars1 = word1.as_bytes();
        let chars2 = word2.as_bytes();

        // Construct the following distance matrix `M`,
        // where `M[i1][i2]` encodes the distance between:
        // - the first `i1` characters of `word1`; and
        // - the first `i2` characters of `word2`.
        //      | W O R D 2
        //      | .  p  e  r  f  e  c  t
        //    --|-----------------------
        // W  . | 0  1  2  3  4  5  6  7
        // O  r | 1  0  0  0  0  0  0  0
        // R  u | 2  0  0  0  0  0  0  0
        // D  s | 3  0  0  0  0  0  0  0
        // 1  t | 4  0  0  0  0  0  0  0
        // The idea here is that every distance `(i,j)` can be computed
        // "recursively" from three different neighbors:
        // 1. `(i-1, j)`: shorten the "vertical" string i.e., a deletion.
        // 2. `(i, j-1)`: shorten the "horizontal" string i.e., an insertion.
        // 3. `(i-1, j-1)`: shorten both strings i.e., a substitution.
        //    The cost of the substitution is 0 when both characters are equal.
        // An extra row and column (symbolized by the `.`) are prepended to:
        // - iterate over the previous row/column of the first row/column.
        // - deal with empty strings.
        let mut distance_matrix = vec![vec![0; word2.len() + 1]; word1.len() + 1];
        distance_matrix[0] = (0..=word2.len()).collect();
        (0..=word1.len()).for_each(|i2| {
            distance_matrix[i2][0] = i2;
        });

        for (i1, c1) in chars1.iter().enumerate() {
            // i1 is the index in chars1 but an extra column has been prepended.
            let row = i1 + 1;
            for (i2, c2) in chars2.iter().enumerate() {
                let col = i2 + 1;
                let substitution_cost = (c1 != c2) as usize;
                distance_matrix[row][col] = [
                    // Previous row -> shorten word1 -> deletion.
                    distance_matrix[row - 1][col] + 1,
                    // Previous col -> shorten word2 -> insertion.
                    distance_matrix[row][col - 1] + 1,
                    // Previous row and col -> shorten both -> substitution.
                    distance_matrix[row - 1][col - 1] + substitution_cost,
                ]
                .into_iter()
                .min()
                .unwrap()
            }
        }
        distance_matrix[word1.len()][word2.len()]
    }

    fn min_distance_naive_recursion(word1: &str, word2: &str) -> usize {
        if word1.is_empty() {
            return word2.len();
        }
        if word2.is_empty() {
            return word1.len();
        }

        let substitution_cost = (word1[0..1] != word2[0..1]) as usize;
        [
            // Shorten word1 -> deletion.
            Self::min_distance_dp_iterative_matrix(&word1[..word1.len() - 1], word2) + 1,
            // Shorten word2 -> insertion.
            Self::min_distance_dp_iterative_matrix(word1, &word2[..word2.len() - 1]) + 1,
            // Keep lengths constant -> substitution.
            Self::min_distance_dp_iterative_matrix(
                &word1[..word1.len() - 1],
                &word2[..word2.len() - 1],
            ) + substitution_cost,
        ]
        .into_iter()
        .min()
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0072() {
        //   | .  h  o  r  s  e
        // --|-----------------
        // . | 0  1  2  3  4  5
        // r | 1  1  2  2  3  4
        // o | 2  2  1  2  3  4
        // s | 3  3  2  2  2  3
        assert_eq!(
            Solution::min_distance_dp_iterative_matrix("ros", "horse"),
            3
        );
        //   | .  e  x  e  c  u  t  i  o  n
        // --|-----------------------------
        // . | 0  1  2  3  4  5  6  7  8  9
        // i | 1  1  2  3  4  5  6  6  7  8
        // n | 2  2  2  3  4  5  6  7  7  7
        // t | 3  3  3  3  4  5  5  6  7  8
        // e | 4  3  4  3  4  5  6  6  7  8
        // n | 5  4  4  4  4  5  6  7  7  7
        // t | 6  5  5  5  5  5  5  6  7  8
        // i | 7  6  6  6  6  6  6  5  6  7
        // o | 8  7  7  7  7  7  7  6  5  6
        // n | 9  8  8  8  8  8  8  7  6  5
        assert_eq!(
            Solution::min_distance_dp_iterative_matrix("intention", "execution"),
            5
        );
        assert_eq!(Solution::min_distance_dp_iterative_matrix("empty", ""), 5);
        assert_eq!(Solution::min_distance_dp_iterative_matrix("", ""), 0);
    }
}
