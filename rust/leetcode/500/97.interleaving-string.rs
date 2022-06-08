/*
 * @lc app=leetcode id=97 lang=rust
 *
 * [97] Interleaving String
 */

// @lc code=start
impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.is_empty() && s2.is_empty() && s3.is_empty() {
            return true;
        }
        if s1.len() + s2.len() != s3.len() {
            return false;
        }
        let mut a = vec![false; s2.len() + 1];
        a[0] = true;
        let sv1 = s1.bytes().collect::<Vec<u8>>();
        let sv2 = s2.bytes().collect::<Vec<u8>>();
        let sv3 = s3.bytes().collect::<Vec<u8>>();
        for i in 0..=s1.len() {
            for j in 0..=s2.len() {
                let k = i + j - 1;
                if i > 0 {
                    a[j] = a[j] && sv1[i - 1] == sv3[k];
                }
                if j > 0 {
                    a[j] = a[j] || a[j - 1] && sv2[j - 1] == sv3[k];
                }
            }
        }
        a[s2.len()]
    }
}
// @lc code=end
