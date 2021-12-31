/*
 * @lc app=leetcode id=806 lang=rust
 *
 * [806] Number of Lines To Write String
 */

// @lc code=start
impl Solution {
    pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
         let mut lines = 0;
        let mut units = 0;
        for c in s.chars() {
            let l = widths[(c as u8 - 'a' as u8) as usize];
            if units + l > 100 {
                lines += 1;
                units = l;
            } else {
                units += l;
            }
        }
        if units > 0 {
            lines += 1;
        }
        [lines, units].to_vec()
    }
}
// @lc code=end

