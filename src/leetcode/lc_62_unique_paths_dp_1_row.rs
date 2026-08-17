struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut row = vec![1; m];
        for _ in 1..n {
            for j in 1..m {
                row[j] += row[j - 1];
            }
        }
        row[m - 1]
    }
}

fn main() {
    assert_eq!(Solution::unique_paths(3, 7), 28);
}
