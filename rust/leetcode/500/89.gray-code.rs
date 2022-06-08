/*
 * @lc app=leetcode id=89 lang=rust
 *
 * [89] Gray Code
 */

// @lc code=start
impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut ans = Vec::new();
        ans.push(0);
        let mut p = 1;
        for i in 0..n{
            for j in (0..ans.len()).rev(){
                ans.push(p+ans[j]);
            }
            p<<=1;
        }
        
        ans
    }
}
// @lc code=end
