/*
 * @lc app=leetcode id=764 lang=rust
 *
 * [764] Largest Plus Sign
 */

// @lc code=start
impl Solution {
    pub fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32 {
        let banned = mines.iter().map(|x|(x[0]*n+x[1]) as usize).collect::<std::collections::HashSet<usize>>();
        let mut count =0;
        let n = n as usize;
        let mut dp =vec![vec![0;n];n];
        for r in 0..n{
             count =0;
             for c in 0..n{
                  count = if banned.contains(&(r*n+c)){0}else{count+1};
                  dp[r][c]=count;
             }

            count =0;
             for c in (0..n).rev(){
                  count = if banned.contains(&(r*n+c)){0}else{count+1};
                  dp[r][c]= dp[r][c].min(count);
             }
        }
        let mut ans = 0;
        for c in 0..n{
             count =0;
             for r in 0..n{
                  count = if banned.contains(&(r*n+c)){0}else{count+1};
                  dp[r][c]= dp[r][c].min(count);
             }

            count =0;
             for r in (0..n).rev(){
                  count = if banned.contains(&(r*n+c)){0}else{count+1};
                  dp[r][c]= dp[r][c].min(count);
                  ans = ans.max(dp[r][c]);
             }
        }
        ans
    }
}
// @lc code=end

