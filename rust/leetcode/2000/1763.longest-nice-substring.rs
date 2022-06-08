/*
 * @lc app=leetcode id=1763 lang=rust
 *
 * [1763] Longest Nice Substring
 */

// @lc code=start
impl Solution {
    pub fn longest_nice_substring(s: String) -> String {
        let mut v = s.chars().collect::<Vec<char>>();
        let mut l = 0;
        let mut u = 0;
        let mut i = 0;
        let mut k = 0;
        let mut beg = 0;
        let mut end = 0;
        while i < v.len() {
            k = i;
            l = 0;
            u = 0;
            for j in i..v.len() {
                let n =  (v[j] as u8) ;
                if v[j].is_ascii_lowercase() {
                    l |= 1<< (n -('a' as u8));
                } else {
                    u |= 1<< (n- ('A' as u8)) ;
                }
                if l == u {
                    k = j;
                }
            }
            if k - i > end - beg {
                beg = i;
                end = k;
            }
            i = k + 1;
        }
        if beg==end{
            "".to_string()
        }else{
           (&v[beg..end+1]).iter().collect()
        }
    }
}
// @lc code=end
