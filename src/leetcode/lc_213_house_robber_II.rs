struct Solution;

impl Solution {
    pub fn solve(nums: impl IntoIterator<Item=i32>) -> (i32, i32) {
        nums.into_iter().fold(
            (0, 0),
            |(q, w), num| (w, w.max(q + num)),
        )
    }


    pub fn rob(nums: Vec<i32>) -> i32 {
        match nums.as_slice() {
            [] => 0,
            [num] => *num,
            _ => {
                Solution::solve(nums.iter().copied()).0.max(
                    Solution::solve(nums.iter().copied().skip(1)).1
                )
            }
        }
    }
}

fn main() {
    assert_eq!(Solution::rob(vec![2, 3, 2]), 3);
    assert_eq!(Solution::rob(vec![2, 1, 1, 2]), 3);
    assert_eq!(Solution::rob(vec![4, 1, 2, 7, 5, 3, 1]), 14);
}
