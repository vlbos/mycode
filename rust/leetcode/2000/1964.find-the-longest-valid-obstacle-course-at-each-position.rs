/*
 * @lc app=leetcode id=1964 lang=rust
 *
 * [1964] Find the Longest Valid Obstacle Course at Each Position
 */

// @lc code=start
impl Solution {
    pub fn longest_obstacle_course_at_each_position(obstacles: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::new();
        let mut d = Vec::new();
        for &ob in &obstacles {
            if d.is_empty() || ob >= *d.last().unwrap() {
                d.push(ob);
                ans.push(d.len() as i32);
                continue;
            }
            let loc = d.partition_point(|x| *x <= ob);
            ans.push(loc as i32 + 1);
            d[loc] = ob;
        }
        ans
    }
}
// @lc code=end
