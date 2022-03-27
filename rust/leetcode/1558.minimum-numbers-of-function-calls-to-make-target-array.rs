/*
 * @lc app=leetcode id=1558 lang=rust
 *
 * [1558] Minimum Numbers of Function Calls to Make Target Array
 */

// @lc code=start
impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut maxn=0;
        for &n in &nums{
            maxn = maxn.max(n);
            let mut m = n;
             while m>0{
                if m&1>0{
                ans+=1;
                }
                m>>=1;
            }
        }
        if maxn>0{
            while maxn>0{
                ans+=1;
                maxn>>=1;
            }
            ans-=1;
        }
        ans
    }
}
// @lc code=end

