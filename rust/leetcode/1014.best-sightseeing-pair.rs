/*
 * @lc app=leetcode id=1014 lang=rust
 *
 * [1014] Best Sightseeing Pair
 */

// @lc code=start
impl Solution {
    pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut mx = values[0]+0;
        for j in 1..values.len(){
            ans = ans.max(mx+values[j]-j as i32);
            mx=mx.max(values[j]+j as i32);
        }
        ans
    }
}
// @lc code=end

