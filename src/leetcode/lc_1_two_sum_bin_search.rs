struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums: Vec<(usize, i32)> = nums.into_iter().enumerate().collect();
        nums.sort_by_key(|&(_, num)| { num });

        for (idx, &(i, num)) in nums.iter().enumerate() {
            let suffix = &nums[(idx + 1)..];
            if let Ok(jdx) = suffix.binary_search_by_key(&(target - num), |&(_, num)| num) {
                return vec![i as i32, suffix[jdx].0 as i32];
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