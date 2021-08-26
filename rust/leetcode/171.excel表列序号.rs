/*
 * @lc app=leetcode.cn id=171 lang=rust
 *
 * [171] Excel表列序号
 */

// @lc code=start
impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut r: i32 = 0;
        let a = 'A' as u8;
        let al = 26;
        let mut i: i32 = 1;
        for c in column_title.bytes().rev() {
            r += ((c - a + 1) as i32) * i as i32;
            i *= al;
        }
        r
    }
}
// @lc code=end
