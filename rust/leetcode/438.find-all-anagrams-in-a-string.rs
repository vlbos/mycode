/*
 * @lc app=leetcode id=438 lang=rust
 *
 * [438] Find All Anagrams in a String
 */

// @lc code=start
impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        if s.len() < p.len() {
            return Vec::new();
        }
        let mut a = vec![0; 26];
        for b in p.bytes() {
            a[(b - b'a') as usize] += 1;
        }

        let mut sa = vec![0; 26];
        let mut l = 0;
        let mut r = p.len() - 1;
        let sv = s.bytes().collect::<Vec<u8>>();
        let mut ans = Vec::new();
        for i in 0..p.len() - 1 {
            sa[(sv[i] - b'a') as usize] += 1;
        }
        while r < sv.len() {
            sa[(sv[r] - b'a') as usize] += 1;
            let mut flag = true;
            for i in 0..26 {
                if sa[i] != a[i] {
                    flag = false;
                    break;
                }
            }
            if flag {
                ans.push(l as i32);
            }
            sa[(sv[l] - b'a') as usize] -= 1;
            l += 1;
            r += 1;
        }
        ans
    }
}
// @lc code=end
