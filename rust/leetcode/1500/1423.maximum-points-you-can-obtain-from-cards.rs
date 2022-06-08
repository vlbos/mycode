/*
 * @lc app=leetcode id=1423 lang=rust
 *
 * [1423] Maximum Points You Can Obtain from Cards
 */

// @lc code=start
impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let sum:i32 = card_points.iter().sum();
        let k = k as usize;
        let nk = card_points.len() -k;
        let mut wsum :i32= card_points[0..nk].iter().sum();
        let mut minsum=wsum;
        for i in nk..card_points.len(){
             wsum+=card_points[i]-card_points[i-nk];
             minsum=minsum.min(wsum);
        }
        sum-minsum
    }
}
// @lc code=end

