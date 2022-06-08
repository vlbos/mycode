/*
 * @lc app=leetcode id=1927 lang=rust
 *
 * [1927] Sum Game
 */

// @lc code=start
impl Solution {
    pub fn sum_game(num: String) -> bool {
        let mut a = vec![vec![0, 0]; 2];
        let n2 = num.len() / 2;
        for (i, b) in num.bytes().enumerate() {
            if b == b'?' {
                a[i / n2][1] += 1;
            } else {
                a[i / n2][0] += (b - b'0') as i32;
            }
        }
        (a[0][1] + a[1][1]) % 2 > 0 || (a[0][0] - a[1][0]) != (a[1][1] - a[0][1]) * 9 / 2
    }
}
// @lc code=end
