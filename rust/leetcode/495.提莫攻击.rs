/*
 * @lc app=leetcode.cn id=495 lang=rust
 *
 * [495] 提莫攻击
 */

// @lc code=start
impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        if time_series.is_empty(){
        return 0;
        }
        if time_series.len()==1{
            return duration;
        }
        let mut r = 0;
        let mut e = 0;
        for t in time_series{
            r+=duration;
            if e>t{
                r-=e-t;
            }
            e = t+duration;
        }
        r
    }
}
// @lc code=end

