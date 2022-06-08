/*
 * @lc app=leetcode id=1094 lang=rust
 *
 * [1094] Car Pooling
 */

// @lc code=start
impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut diff = vec![0;1001];
        for t in &trips{
            diff[t[1] as usize]+=t[0];
            diff[t[2] as usize]-=t[0];
        }
        let mut c = 0;
        for &d in &diff{
            c+=d;
            if c>capacity{
            return false;
            }
        }
        true
    }
}
// @lc code=end

