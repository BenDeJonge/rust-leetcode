/// https://leetcode.com/problems/longest-substring-without-repeating-characters/
/// Medium - [hash table, string, sliding window]
/// Given a string s, find the length of the longest substring without repeating characters.
use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn length_of_longest_substring_better(s: String) -> i32 {
        let mut best = 0;
        let mut byte_cache = [0; 256];
        let chars = s.bytes().collect::<Vec<u8>>();
        let mut i_start = 0;
        for (i_stop, &ch) in chars.iter().enumerate().take(s.len()) {
            // Get the previous occurence of the character from the cache.
            // let ch_as_idx = chars[i_stop] as usize;
            let i_previous = byte_cache[ch as usize];
            // It occurred before...
            if i_previous > 0 {
                // ... but only overwrite the start index if that occurence is in this block.
                i_start = i_start.max(i_previous);
            }
            // Update the cache and best results.
            byte_cache[ch as usize] = i_stop + 1;
            best = best.max(i_stop - i_start + 1)
        }
        best as i32
    }

    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut i_start = 0;
        let mut best = 0;
        let chars = s.chars().collect::<Vec<char>>();
        // Keep looping as long as the next set could still be longer than the best.
        while i_start + best < s.len() {
            // Insert characters until a duplicate.
            let mut i_stop = i_start;
            let mut slice_map = HashSet::new();
            // Use lazy boolean operation to keep the stop index within bounds.
            while (i_stop < s.len()) && slice_map.insert(chars[i_stop]) {
                // Not a duplicate, advance to the next.
                i_stop += 1;
            }
            // A duplicate has been reached. Store the current result.
            if i_stop - i_start > best {
                best = i_stop - i_start
            }
            // Advance to the next window.
            i_start += 1;
        }
        best as i32
    }
}

fn main() {}

#[cfg(test)]

mod tests {
    use super::Solution;

    #[test]
    fn test_0003() {
        assert_eq!(
            Solution::length_of_longest_substring_better(" ".to_string()),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring_better("au".to_string()),
            2
        );
        assert_eq!(
            Solution::length_of_longest_substring_better("abcabcbb".to_string()),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring_better("bbbbb".to_string()),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring_better("pwwkew".to_string()),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring_better("abcdafghija".to_string()),
            9
        );
    }
}
