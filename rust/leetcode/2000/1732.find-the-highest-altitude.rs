/*
 * @lc app=leetcode id=1732 lang=rust
 *
 * [1732] Find the Highest Altitude
 */

// @lc code=start
impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut point_altitudes = vec![0;gain.len()+1];
        point_altitudes[0]=0;
        for i in 1..point_altitudes.len(){
            point_altitudes[i]=point_altitudes[i-1]+gain[i-1];
        }
        *(point_altitudes.iter().max().unwrap())
    }
}
// @lc code=end

