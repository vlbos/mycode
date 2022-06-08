/*
 * @lc app=leetcode id=435 lang=rust
 *
 * [435] Non-overlapping Intervals
 */

// @lc code=start
impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.is_empty(){
        return 0;
        }
        let mut intervals = intervals;
        intervals.sort_by(|a,b|a[1].cmp(&b[1]));
        let mut ans = 1;
        let mut right = intervals[0][1];
        for i in &intervals{
            if i[0]>=right{
                ans+=1;
                right=i[1];
            }
        }
        intervals.len() as i32-ans
    }
}
// @lc code=end

