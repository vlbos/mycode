/*
 * @lc app=leetcode id=1848 lang=rust
 *
 * [1848] Minimum Distance to the Target Element
 */

// @lc code=start
impl Solution {
    pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
        let mut min = i32::MAX;
        for (i,n) in nums.iter().enumerate(){
            if *n==target && (i as i32 -start).abs()<min{
                min= (i as i32 -start).abs();
            }
        }
        min
    }
}
// @lc code=end

