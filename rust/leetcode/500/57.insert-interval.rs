/*
 * @lc app=leetcode id=57 lang=rust
 *
 * [57] Insert Interval
 */

// @lc code=start
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        let mut new_interval = new_interval;
        let mut i = 0;
        let mut index = 0;
        while i < intervals.len() {
            if  !(intervals[i][0] > new_interval[1] || intervals[i][1] < new_interval[0])
            {
                new_interval[0] = new_interval[0].min(intervals[i][0]);
                new_interval[1] = new_interval[1].max(intervals[i][1]);
                intervals.remove(i);
                index = i;
            } else {
                if new_interval[1]<intervals[i][0] {
                    intervals.insert(i, new_interval);
                    return intervals;
                }
                i += 1;
                index=i;
            }
        }
        if index >= intervals.len() {
            intervals.push(new_interval);
        } else {
            intervals.insert(index, new_interval);
        }

        intervals
    }
}
// @lc code=end
