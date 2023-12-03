use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<Vec<i32>, Vec<String>> = HashMap::new();
        for letter in strs.iter() {
            let mut count = vec![0; 26];
            for c in letter.chars() {
                count[c as usize - 'a' as usize] += 1;
            }
            map.entry(count).or_default().push(letter.clone());
        }
        map.values().cloned().collect()
    }
}
