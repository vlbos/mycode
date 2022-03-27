/*
 * @lc app=leetcode id=1049 lang=rust
 *
 * [1049] Last Stone Weight II
 */

// @lc code=start
impl Solution {
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        let sum = stones.iter().sum::<i32>();
        let m = (sum/2+1) as usize;
        let mut dp =vec![false;m ];
        dp[0]=true;
        for &w in &stones{
            let w = w as usize;
             for  i in (w..m).rev(){
                dp[i] |=dp[i-w];
            }
        }
        sum-2*(m-1-dp.iter().rev().position(|&x|x).unwrap_or(0)) as i32
    }
}
// @lc code=end

