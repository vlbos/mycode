/*
 * @lc app=leetcode id=1017 lang=rust
 *
 * [1017] Convert to Base -2
 */

// @lc code=start
impl Solution {
    pub fn base_neg2(n: i32) -> String {
        let mut max = 1;
        while max>0 && max<n {
            max = (max<<2)+ 1;
        }
        format!("{:b}", max ^ (max - n))
    }
}
// @lc code=end

