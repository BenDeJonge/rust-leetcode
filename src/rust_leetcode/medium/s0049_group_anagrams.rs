//! https://leetcode.com/problems/group-anagrams/
//! Medium - [array, hash table, string, sorting]
//! Given an array of strings, group the anagrams together.

use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    /// 1. For every string, count the letters in a vector.
    /// 2. Use these vectors as keys in a hashmap, with a vector of strings as vakyes.
    /// 3. Iterate over the hashmap and return a vector of values.
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagrams: HashMap<[usize; 26], Vec<String>> = HashMap::new();
        for s in strs {
            let mut letter_counts = [0usize; 26];
            for ch in s.bytes() {
                letter_counts[(ch - b'a') as usize] += 1;
            }
            anagrams
                .entry(letter_counts)
                .and_modify(|strings| strings.push(s.to_owned()))
                .or_insert(vec![s]);
        }
        anagrams.values().cloned().collect()
    }
}

#[cfg(test)]
mod tests {

    use super::Solution;

    fn sort_anagrams(anagrams: Vec<Vec<String>>) -> Vec<Vec<String>> {
        // Not strictly needed but easier for testing.
        let mut ret: Vec<Vec<String>> = anagrams
            .iter()
            .map(|v| {
                let mut c = v.clone();
                c.sort();
                c
            })
            .collect();
        ret.sort();
        ret
    }

    #[test]
    fn group_anagrams() {
        assert_eq!(
            sort_anagrams(Solution::group_anagrams(vec![
                "eat".to_owned(),
                "tea".to_owned(),
                "tan".to_owned(),
                "ate".to_owned(),
                "nat".to_owned(),
                "bat".to_owned()
            ])),
            vec![vec!["ate", "eat", "tea"], vec!["bat"], vec!["nat", "tan"]]
        );
        assert_eq!(
            sort_anagrams(Solution::group_anagrams(vec!["".to_owned()])),
            vec![vec![""]]
        );
        assert_eq!(
            sort_anagrams(Solution::group_anagrams(vec!["a".to_owned()])),
            vec![vec!["a"]]
        )
    }
}
