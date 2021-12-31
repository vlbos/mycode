/*
 * @lc app=leetcode id=974 lang=rust
 *
 * [974] Subarray Sums Divisible by K
 */

// @lc code=start
impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
         let n =nums.len();
            let mut ans = 0;
            let mut sum = 0;
            let mut m = vec![0;k as usize];
            m[0]=1;
            for &v in &nums{
                sum+=v;
                let key = ((sum%k+k)%k) as usize;
                 ans+=m[key];
                 m[key]+=1;
            }
            ans
    }
}
// @lc code=end

