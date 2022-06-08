/*
 * @lc app=leetcode id=1043 lang=rust
 *
 * [1043] Partition Array for Maximum Sum
 */

// @lc code=start
impl Solution {
    pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
       let n = arr.len();
        let mut dp =vec![0;n];
        let mut currmax = arr[0];
        dp[0]=currmax;
        let k = k as usize;
        for i in 1..k{
            currmax=currmax.max(arr[i]);
            dp[i]=currmax*(i as i32+1);
        }
        for i in k..n{
            for j in i-k..i{
                dp[i]=dp[i].max(dp[j]+*arr[j+1..i+1].iter().max().unwrap()*((i-j) as i32));
            }
        }
        dp[n-1]
    }
}
// @lc code=end

