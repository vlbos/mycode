/*
 * @lc app=leetcode id=304 lang=rust
 *
 * [304] Range Sum Query 2D - Immutable
 */

// @lc code=start
struct NumMatrix {
    dp:Vec<Vec<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {

    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut dp = vec![vec![0; n+1]; m+1];
        for i in 1..=m {
            dp[i][0] = dp[i - 1][0] + matrix[i-1][0];
        }
        for j in 1..=n {
            dp[0][j] = dp[0][j - 1] + matrix[0][j-1];
        }
        for i in 1..=m {
            for j in 1..=n {
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1] - dp[i - 1][j - 1] + matrix[i-1][j-1];
            }
        }
        Self{dp}
    }
    
    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
          self.dp[row2 as usize+1][col2 as usize+1]+self.dp[row1 as usize][col1 as usize]-self.dp[row2 as usize+1][col1 as usize]-self.dp[row1 as usize][col2 as usize+1]
    }
}

/**
 * Your NumMatrix object will be instantiated and called as such:
 * let obj = NumMatrix::new(matrix);
 * let ret_1: i32 = obj.sum_region(row1, col1, row2, col2);
 */
// @lc code=end

