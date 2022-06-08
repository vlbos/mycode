/*
 * @lc app=leetcode id=1499 lang=rust
 *
 * [1499] Max Value of Equation
 */

// @lc code=start
impl Solution {
    pub fn find_max_value_of_equation(points: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut q = std::collections::VecDeque::from([(points[0][0], points[0][1] - points[0][0])]);
        let mut ans = i32::MIN;
        for p in points.iter().skip(1) {
            while !q.is_empty() && p[0] - q.front().unwrap().0 > k {
                q.pop_front();
            }
            if !q.is_empty() {
                ans = ans.max(q.front().unwrap().1 + p[0] + p[1]);
            }
            while !q.is_empty() && p[1] - p[0] >= q.back().unwrap().1 {
                q.pop_back();
            }
            q.push_back((p[0],p[1] - p[0]));
        }
        ans
    }
}
// @lc code=end
