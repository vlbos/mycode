/*
 * @lc app=leetcode id=2038 lang=rust
 *
 * [2038] Remove Colored Pieces if Both Neighbors are the Same Color
 */

// @lc code=start
impl Solution {
    pub fn winner_of_game(colors: String) -> bool {
        let mut cnt = vec![0; 2];
        let mut i = 0;
        let bc = colors.as_bytes();
        let n = bc.len();
        while i < n {
            let i0 = i;
            while i < n && bc[i] == bc[i0] {
                i += 1;
            }
            let l = i - i0;
            if l > 2 {
                cnt[(bc[i0] - b'A') as usize] += l - 2;
            }
        }
        cnt[0] > cnt[1]
    }
}
// @lc code=end
