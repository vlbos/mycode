/*
 * @lc app=leetcode id=826 lang=rust
 *
 * [826] Most Profit Assigning Work
 */

// @lc code=start
impl Solution {
    pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
        let mut dp = difficulty.iter().zip(profit.iter()).map(|(a,b)|(*a,*b)).collect::<Vec<(i32,i32)>>();
        dp.sort();
        let mut worker=worker;
        worker.sort();
        let mut i = 0;
        let n = difficulty.len();
        let mut best = 0;
        let mut ans =0;
        for w in &worker{
             while i<n && *w>=dp[i].0{
                best = best.max(dp[i].1);
                i+=1;
             }
            ans+=best;
        }
        ans
    }
}
// @lc code=end

