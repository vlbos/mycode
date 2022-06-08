/*
 * @lc app=leetcode id=718 lang=rust
 *
 * [718] Maximum Length of Repeated Subarray
 */

// @lc code=start
impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n1=nums1.len();
        let n2=nums2.len();
        let mut dp=vec![vec![0;n2+1];n1+1];
        let mut ans = 0;
        for i in (0..n1).rev(){
            for j in (0..n2).rev(){
                dp[i][j]=if nums1[i]==nums2[j] {dp[i+1][j+1]+1}else{0};
                ans=ans.max(dp[i][j]);
            }
        }
        ans
    }
}
// @lc code=end

