/*
 * @lc app=leetcode id=14 lang=rust
 *
 * [14] Longest Common Prefix
 */

// @lc code=start
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
         if (strs.len() == 1) {
            return (*strs.get(0).unwrap()).clone();
        }
        let mut i = usize::MAX;
        i = strs.iter().map(|v| v.len()).min().unwrap();
        while i > 0 {
            let substr = &strs.get(0).unwrap()[0..i];
            let mut r = strs.iter().filter(|v| substr != &v[0..i]);
            if r.next() == None {
                return String::from(substr);
            }

            i -= 1;
        }
        String::from("")
    }
}
// @lc code=end

