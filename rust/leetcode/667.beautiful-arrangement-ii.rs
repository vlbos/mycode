/*
 * @lc app=leetcode id=667 lang=rust
 *
 * [667] Beautiful Arrangement II
 */

// @lc code=start
impl Solution {
    pub fn construct_array(n: i32, k: i32) -> Vec<i32> {
        let n = n as usize;
        let k = k as usize;
        let mut ans = vec![0;n];
        for i in 0..n-k-1{
             ans[i]=i as i32+1;
        }
        let mut left = (n-k) as i32;
        let mut right = n  as i32;
        for i in n-k-1..n{
            if i%2==(n-k-1)%2{
                ans[i]=left;
                left+=1;
            }else{
                ans[i]=right;
                right-=1;
            }
        }
        ans
    }
}
// @lc code=end

