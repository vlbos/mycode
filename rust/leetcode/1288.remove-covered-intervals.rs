/*
 * @lc app=leetcode id=1288 lang=rust
 *
 * [1288] Remove Covered Intervals
 */

// @lc code=start
impl Solution {
    pub fn remove_covered_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals=intervals;
        intervals.sort_by_key(|x| vec![x[0],-x[1]]);
        let mut ans = 0;
        let mut prev = 0;
        for v in &intervals{
             if v[1]>prev{
                ans+=1;
                prev=v[1];
             }
        }
        ans
    }
}
// @lc code=end

