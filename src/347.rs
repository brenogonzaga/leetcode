use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        for i in nums.iter() {
            *map.entry(*i).or_default() += 1;
        }

        let mut pairs: Vec<_> = map.iter().collect();
        pairs.sort_by(|a, b| b.1.cmp(a.1));
        pairs
            .iter()
            .map(|(&num, &_freq)| num)
            .take(k as usize)
            .collect()
    }
}
