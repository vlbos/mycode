/*
 * @lc app=leetcode id=56 lang=rust
 *
 * [56] Merge Intervals
 */

// @lc code=start
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort();
        let mut ans = Vec::new();
        let mut t= Vec::new();
        for n in &intervals{
            if t.is_empty(){
                t=(*n).clone();
            }else if t[1]>=n[0]{
                t[0]=t[0].min(n[0]);
                t[1]=t[1].max(n[1]);
            }else{
                ans.push(t);
                t=(*n).clone();
            }
        }
        if !t.is_empty(){
            ans.push(t);
        }
        ans
    }
}
// @lc code=end

