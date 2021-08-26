/*
 * @lc app=leetcode.cn id=830 lang=rust
 *
 * [830] 较大分组的位置
 */

// @lc code=start
impl Solution {
    pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
        let mut r = Vec::<Vec<i32>>::new();
        let mut j = 0;
        let mut p = ' ';
        for (i, c) in s.chars().enumerate() {
            if i == 0 {
                p = c;
            } else if c != p {
                if i - j > 2 {
                    r.push(vec![j as i32, (i - 1) as i32].to_vec());
                }
                j = i;
                p = c;
            }
            if i == s.len() - 1 {
                if i - j >= 2 {
                    r.push(vec![j as i32, (i) as i32].to_vec());
                }
            }
        }

        r
    }
}
// @lc code=end
