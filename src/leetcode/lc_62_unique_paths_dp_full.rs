struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut dp = vec![vec![0; m + 1]; n + 1];
        dp[0][1] = 1;
        for i in 1..=n {
            for j in 1..=m {
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
            }
        }
        dp[n][m]
    }
}

fn main() {
    assert_eq!(Solution::unique_paths(3, 7), 28);
}
