/*
 * @lc app=leetcode id=1812 lang=rust
 *
 * [1812] Determine Color of a Chessboard Square
 */

// @lc code=start
impl Solution {
    pub fn square_is_white(coordinates: String) -> bool {
        let x = coordinates.chars().nth(0).unwrap() as u8 - 'a' as u8;
        let y = coordinates.chars().nth(1).unwrap() as u8 - '1' as u8;
        !(x%2==y%2)
    }
}
// @lc code=end

