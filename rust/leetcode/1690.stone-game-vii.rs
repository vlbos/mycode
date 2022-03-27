/*
 * @lc app=leetcode id=1690 lang=rust
 *
 * [1690] Stone Game VII
 */

// @lc code=start
impl Solution {
    pub fn stone_game_vii(stones: Vec<i32>) -> i32 {
        let n = stones.len();
        let mut dp=vec![0;n];
        let mut sums=stones.clone();
        for i in (1..n).rev(){
            for j in 0..i{
                let k = n+j-i;
                sums[j]+=stones[k];
                let (x,y)=(sums[j]-stones[j],sums[j]-stones[k]);
                dp[j]=(x-dp[j+1]).max(y-dp[j]);
            }
        }
        dp[0]
    }
}
// @lc code=end

