struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut start, mut end) = (1, nums.len());
        while start < end {
            let sum = nums[start - 1] + nums[end - 1];

            match sum.cmp(&target) {
                std::cmp::Ordering::Equal => return vec![start as i32, end as i32],
                std::cmp::Ordering::Less => start += 1,
                std::cmp::Ordering::Greater => end -= 1,
            }
        }
        vec![]
    }
}
