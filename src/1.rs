use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());
        for (i, val) in nums.iter().enumerate() {
            match map.get(&(target - val)) {
                Some(index) => return vec![*index, i as i32],
                None => map.insert(val, i as i32),
            };
        }
        vec![]
    }
}
