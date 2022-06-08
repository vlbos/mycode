/*
 * @lc app=leetcode id=1790 lang=rust
 *
 * [1790] Check if One String Swap Can Make Strings Equal
 */

// @lc code=start
impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        if s1 == s2 {
            return true;
        }
        if s1.len() != s2.len() {
            return false;
        }
        let v1 = s1.chars().collect::<Vec<char>>();
        let v2 = s2.chars().collect::<Vec<char>>();
        let mut ans = Vec::new();
        for i in 0..v1.len() {
            if v1[i] != v2[i] {
                if ans.len() == 2 {
                    return false;
                }
                ans.push(i);
            }
        }
        ans.len()==2 && v1[ans[0]] == v2[ans[1]] && v1[ans[1]] == v2[ans[0]]
    }
}
// @lc code=end
