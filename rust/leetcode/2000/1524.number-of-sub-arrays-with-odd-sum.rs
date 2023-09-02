/*
 * @lc app=leetcode id=1524 lang=rust
 *
 * [1524] Number of Sub-arrays With Odd Sum
 */

// @lc code=start
impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        let (mut even, mut odd) = (1, 0);
        let mut ans = 0;
        let mut sum = 0;
        for &v in &arr {
            sum += v;
            ans += if sum % 2 == 0 { odd } else { even };
            ans %= 1000_000_007;
            if sum % 2 == 0 {
                even += 1;
            } else {
                odd += 1;
            }
        }
        ans
    }
}
// @lc code=end
impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        let mut dp=arr[0]%2;
        let mut ans=dp as i64;
        for i in 1..arr.len(){
            if arr[i]%2==1{
                dp=1+i as i32-dp ;
            }
            ans=(ans+dp as i64)%1_000_000_007;
        }
        ans as _
    }
}