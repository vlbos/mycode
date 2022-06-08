/*
 * @lc app=leetcode id=1104 lang=rust
 *
 * [1104] Path In Zigzag Labelled Binary Tree
 */

// @lc code=start
impl Solution {
    pub fn path_in_zig_zag_tree(label: i32) -> Vec<i32> {
         let mut l = label;
        let mut row = 1;
        let mut rowstart = 1;
        while l >= rowstart << 1 {
            row += 1;
            rowstart <<= 1;
        }
        let get_reverse =
            |row: i32, label: i32| -> i32 { (1 << (row - 1)) + (1 << row) - 1 - label };
        if row & 1 == 0 {
            l = get_reverse(row, l);
        }
        let mut ans = Vec::new();
        while row > 0 {
            ans.push(if row & 1 == 0 { get_reverse(row, l) } else { l });
            row -= 1;
            l >>= 1;
        }
        ans.reverse();
        ans
    }
}
// @lc code=end
