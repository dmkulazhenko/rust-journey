struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut value = Vec::with_capacity(nums.len());
        value.push(nums[0]);
        if nums.len() > 1 { value.push(nums[1].max(nums[0])); }

        for i in 2..nums.len() {
            value.push(value[i - 1].max(value[i - 2] + nums[i]));
        }

        value.pop().unwrap().max(value.pop().unwrap_or(0))
    }
}

fn main() {
    println!("{}", Solution::rob(vec![1, 2, 3, 1]));
}