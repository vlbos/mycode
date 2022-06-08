/*
 * @lc app=leetcode id=517 lang=rust
 *
 * [517] Super Washing Machines
 */

// @lc code=start
impl Solution {
    pub fn find_min_moves(machines: Vec<i32>) -> i32 {
        let n = machines.len() as i32;
        let total = machines.iter().sum::<i32>();
        if total % n > 0 {
            return -1;
        }
        let avg = total / n;
        let mut ans = 0;
        let mut sum = 0;
        for &num in &machines {
            sum += num - avg;
            ans = ans.max(sum.abs().max(num- avg));
        }
        ans
    }
}
// @lc code=end
