/*
 * @lc app=leetcode id=1665 lang=rust
 *
 * [1665] Minimum Initial Energy to Finish Tasks
 */

// @lc code=start
impl Solution {
    pub fn minimum_effort(tasks: Vec<Vec<i32>>) -> i32 {
        let mut tasks = tasks;
        tasks.sort_by_key(|x| x[0] - x[1]);
        let mut ans = 0;
        let mut suma = 0;
        for task in &tasks {
            ans = ans.max(suma + task[1]);
            suma += task[0];
        }
        ans
    }
}
// @lc code=end
