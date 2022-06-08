/*
 * @lc app=leetcode id=1642 lang=rust
 *
 * [1642] Furthest Building You Can Reach
 */

// @lc code=start
impl Solution {
    pub fn furthest_building(heights: Vec<i32>, bricks: i32, ladders: i32) -> i32 {
       let mut q = std::collections::BinaryHeap::new();
        let mut bricks = bricks;
        let mut ladders = ladders;
        for i in 0..heights.len() - 1 {
            let d = heights[i + 1] - heights[i];
            if d > 0 {
                q.push(-d);
            }
            if q.len() > ladders as usize{
                bricks += q.pop().unwrap();
            }
         
            if bricks < 0 {
                return i as _;
            }
        }
        heights.len() as i32 - 1  
    }
}
// @lc code=end
