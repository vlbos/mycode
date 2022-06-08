/*
 * @lc app=leetcode id=777 lang=rust
 *
 * [777] Swap Adjacent in LR String
 */

// @lc code=start
impl Solution {
    pub fn can_transform(start: String, end: String) -> bool {
        if start.replace("X", "") != end.replace("X", "") {
            return false;
        }
        let n = start.len();
        let s = start.chars().collect::<Vec<char>>();
        let e = end.chars().collect::<Vec<char>>();
        let mut i = 0;
        let mut j = 0;
        while i < n && j < n {
            while i < n && s[i] == 'X' {
                i += 1;
            }
            while j < n && e[j] == 'X' {
                j += 1;
            }
            if (i < n) ^ (j < n) {
                return false;
            }
            if i < n && j < n {
                if s[i] != e[j] || (s[i] == 'L' && i < j) || (s[i] == 'R' && i > j) {
                    return false;
                }
            }
            i += 1;
            j += 1;
        }

        true
    }
}
// @lc code=end
