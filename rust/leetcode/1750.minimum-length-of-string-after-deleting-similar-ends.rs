/*
 * @lc app=leetcode id=1750 lang=rust
 *
 * [1750] Minimum Length of String After Deleting Similar Ends
 */

// @lc code=start
impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let mut sv = s.as_bytes();
        let (mut l, mut r) = (0, sv.len() - 1);
        while l < r {
            if sv[l] != sv[r] {
                break;
            }
            let c = sv[l];
            while l < r && sv[l] == c {
                l += 1;
            }
            if l == r {
                return 0;
            }
            while l < r && sv[r] == c {
                r -= 1;
            }
        }
        (r - l + 1) as _
    }
}
// @lc code=end
