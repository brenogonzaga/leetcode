use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = nums.into_iter().collect();
        let mut max_len = 0;

        println!("{:?}", set);
        for &num in set.iter() {
            if !set.contains(&(num - 1)) {
                let mut current_num = num;
                let mut current_len = 1;

                while set.contains(&(current_num + 1)) {
                    current_num += 1;
                    current_len += 1;
                }

                max_len = max_len.max(current_len);
            }
        }

        max_len
    }
}
