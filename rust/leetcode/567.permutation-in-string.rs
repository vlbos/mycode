/*
 * @lc app=leetcode id=567 lang=rust
 *
 * [567] Permutation in String
 */

// @lc code=start
impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s2.len() < s1.len() {
            return false;
        }
        let mut s1a = vec![0; 26];
        let mut s2a = vec![0; 26];
        for c in s1.bytes() {
            s1a[(c - b'a') as usize] += 1;
        }
        let sv2 = s2.bytes().collect::<Vec<u8>>();
        for i in 0..s1.len() {
            s2a[(sv2[i] - b'a') as usize] += 1;
        }
        let mut flag = true;
            for j in 0..26 {
                if s1a[j] != s2a[j] {
                    flag = false;
                    break;
                }
            }
            if flag {
                return true;
            }
        for i in s1.len()..s2.len() {
            s2a[(sv2[i - s1.len()] - b'a') as usize] -= 1;
            s2a[(sv2[i] - b'a') as usize] += 1;
            flag = true;
            for j in 0..26 {
                if s1a[j] != s2a[j] {
                    flag = false;
                    break;
                }
            }
            if flag {
                return true;
            }
        }
        false
    }
}
// @lc code=end
