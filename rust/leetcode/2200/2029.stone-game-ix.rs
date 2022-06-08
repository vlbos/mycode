/*
 * @lc app=leetcode id=2029 lang=rust
 *
 * [2029] Stone Game IX
 */

// @lc code=start
impl Solution {
    pub fn stone_game_ix(stones: Vec<i32>) -> bool {
        let mut cnt = vec![0i32; 3];
        for &v in &stones {
            let i = (v % 3) as usize;
            cnt[i] += 1;
        }
        if cnt[0] % 2 == 0 {
            cnt[1] >= 1 && cnt[2] >= 1
        } else {
            (cnt[1] - cnt[2]).abs() > 2
        }
    }
}
// @lc code=end
