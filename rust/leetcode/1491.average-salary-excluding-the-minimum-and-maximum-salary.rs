/*
 * @lc app=leetcode id=1491 lang=rust
 *
 * [1491] Average Salary Excluding the Minimum and Maximum Salary
 */

// @lc code=start
impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let mut a = 0f64;
        let min = *(salary.iter().min().unwrap());
        let max = *(salary.iter().max().unwrap());
        for s in &salary {
            if *s != min && *s != max {
                let f = *s as f64 / (salary.len() - 2) as f64;
                a += f;
            }
        }
        a
    }
}
// @lc code=end
