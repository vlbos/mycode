/*
 * @lc app=leetcode id=1370 lang=rust
 *
 * [1370] Increasing Decreasing String
 */

// @lc code=start
impl Solution {
    pub fn sort_string(s: String) -> String {
        let mut r = vec![0; 26];
        for c in s.chars() {
            let i = (c as u8 - 'a' as u8) as usize;
            r[i] += 1;
        }
        let mut ans = Vec::new();
        let mut d = 1;
        let mut i = 0;
        while ans.len() < s.len() {
            if r[i as usize] > 0 {
                ans.push(('a' as u8 + i as u8) as char);
                r[i as usize] -= 1;
            }
            i += d ;

            if i < 0 || i > 25 {
                i = if i<0{0}else{25};
                d *= -1;
            }
            
        }
        ans.iter().collect()
    }
}
// @lc code=end
