//! https://leetcode.com/problems/longest-common-prefix/
//! Easy - [string, trie]
//! Write a function to find the longest common prefix string amongst an array of strings.
//! If there is no common prefix, return an empty string "".

pub struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        strs.into_iter()
            // Recursively apply the function. The output (intially the 1st element) is stored in acc (accumulated) and
            // every element is processed one after the other. The output is then stored again in acc.
            .reduce(|acc, s| {
                acc.chars()
                    .zip(s.chars())
                    .take_while(|(c_p, c_s)| c_p == c_s)
                    .map(|(c, _)| c)
                    .collect()
            })
            .unwrap()
    }

    pub fn longest_common_prefix_simple(strs: Vec<String>) -> String {
        // The current prefix is the first word.
        let mut prefix = strs
            .first()
            .expect("strs is empty")
            .chars()
            .collect::<Vec<char>>();
        // Check all other words.
        for s in strs.iter().skip(1) {
            // Prefix cannot be longer than shortest word.
            if prefix.len() > s.len() {
                prefix.drain(s.len()..);
            }
            for (i, c_s) in s.chars().enumerate() {
                if i >= prefix.len() {
                    break;
                }
                // The prefix only matches until here with the current word.
                if prefix[i] != c_s {
                    prefix.drain(i..);
                    break;
                }
            }
        }
        prefix.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0014() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            "fl"
        );
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ]),
            ""
        );
        assert_eq!(
            Solution::longest_common_prefix(vec!["ab".to_string(), "a".to_string(),]),
            "a"
        );
    }
}
