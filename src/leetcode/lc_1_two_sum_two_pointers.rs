use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums: Vec<(usize, i32)> = nums.into_iter().enumerate().collect();
        nums.sort_by_key(|&(_, num)| num);

        let (mut left, mut right) = (0, nums.len() - 1);
        while right < nums.len() {
            match (nums[left].1 + nums[right].1).cmp(&target) {
                Ordering::Equal => return vec![nums[left].0 as i32, nums[right].0 as i32],
                Ordering::Less => left += 1,
                Ordering::Greater => right -= 1,
            }
        }
        panic!("Not found");
    }
}

fn main() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
}