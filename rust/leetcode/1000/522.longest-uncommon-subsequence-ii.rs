/*
 * @lc app=leetcode id=522 lang=rust
 *
 * [522] Longest Uncommon Subsequence II
 */

// @lc code=start
impl Solution {
    pub fn find_lu_slength(strs: Vec<String>) -> i32 {
        let is_subseq = |s1: &String, s2: &String| -> bool {
            let sv1 = s1.chars().collect::<Vec<char>>();
            let mut l = 0;
            for c in s2.chars() {
                if l == sv1.len() {
                    break;
                }
                if sv1[l] == c {
                    l += 1;
                }
            }
            l == s1.len()
        };
        let mut strs = strs;
        strs.sort_by(|a, b| b.len().cmp(&a.len()));
        for (i,s) in strs.iter().enumerate() {
            let mut flag = true;
            for (j,ss) in strs.iter().enumerate() {
                if i == j {
                    continue;
                }
                if is_subseq(s, ss) {
                    flag = false;
                    break;
                }
            }
            if flag {
                return s.len() as i32;
            }
        }
        -1
    }
}
// @lc code=end
