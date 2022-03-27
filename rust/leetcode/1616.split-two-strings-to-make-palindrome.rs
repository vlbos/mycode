/*
 * @lc app=leetcode id=1616 lang=rust
 *
 * [1616] Split Two Strings to Make Palindrome
 */

// @lc code=start
impl Solution {
    pub fn check_palindrome_formation(a: String, b: String) -> bool {
        let (aa, bb) = (a.as_bytes(), b.as_bytes());
        let check = |aa: &[u8], bb: &[u8]| -> bool {
            let (mut i, mut j) = (0, a.len() - 1);
            let (mut la, mut lb) = (0, 0);
            let mut mode = 0;
            while i < j {
                if mode == 0 && aa[i] != bb[j] {
                    i -= 1;
                    j += 1;
                    la = i;
                    lb = j;
                    mode = 1;
                } else if mode == 1 && aa[i] != aa[j] {
                    i = la;
                    j = lb;
                    mode = 2;
                } else if mode == 2 && bb[i] != bb[j] {
                    return false;
                }
                i += 1;
                j -= 1;
            }
            true
        };
        if check(aa, bb) || check(bb, aa) {
            return true;
        }
        false
    }
}
// @lc code=end
