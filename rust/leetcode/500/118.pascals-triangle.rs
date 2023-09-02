/*
 * @lc app=leetcode id=118 lang=rust
 *
 * [118] Pascal's Triangle
 */

// @lc code=start
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
         let mut a = Vec::<Vec<i32>>::new();
        for i in 0..num_rows {
            let mut b = Vec::<i32>::new();
            for j in 0..i + 1 {
                if 0 == i {
                    b.push(j + 1);
                } else {
                    if 0 == j || i == j {
                        b.push(1);
                    } else {
                        b.push(
                            a[(i - 1) as usize][(j - 1) as usize] + a[(i - 1) as usize][j as usize],
                        );
                    }
                }
            }
            a.push(b);
        }
        a
    }
}
// @lc code=end

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let n=num_rows as usize;
        let mut dp:Vec<Vec<i32>> = (0..n).map(|i| vec![1;i+1]).collect();
        for i in 2..n{
            for j in 1..i{
                dp[i][j]=dp[i-1][j-1]+dp[i-1][j];
            }
        }
        dp
    }
}