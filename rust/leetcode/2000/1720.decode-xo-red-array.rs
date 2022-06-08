/*
 * @lc app=leetcode id=1720 lang=rust
 *
 * [1720] Decode XORed Array
 */

// @lc code=start
impl Solution {
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let mut ans = vec![0;encoded.len()+1];
        ans[0]=first;
        for i in 1..ans.len(){
            ans[i]=ans[i-1]^encoded[i-1];
        }
        ans
    }
}
// @lc code=end

