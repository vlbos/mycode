/*
 * @lc app=leetcode id=1138 lang=rust
 *
 * [1138] Alphabet Board Path
 */

// @lc code=start
impl Solution {
    pub fn alphabet_board_path(target: String) -> String {
        let (mut x, mut y) = (0, 0);
        let mut ans = String::new();
        for b in target.bytes() {
            let d = (b - b'a') as i32;
            let (dx, dy) = (d / 5, d % 5);
            let (mx, my) = (dx - x, dy - y);
            if dy < y {
                ans.push_str(("L").repeat(my.abs() as usize).as_str());
            }
            if dx < x {
                ans.push_str(("U").repeat(mx.abs() as usize).as_str());
            }
            if dy > y {
                ans.push_str(("R").repeat(my.abs() as usize).as_str());
            }
            if dx > x {
                ans.push_str(("D").repeat(mx.abs() as usize).as_str());
            }
            ans.push('!');
            x = dx;
            y = dy;
        }
        ans
    }
}
// @lc code=end
