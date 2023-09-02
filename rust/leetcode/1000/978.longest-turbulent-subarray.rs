/*
 * @lc app=leetcode id=978 lang=rust
 *
 * [978] Longest Turbulent Subarray
 */

// @lc code=start
impl Solution {
    pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
       let mut ans = 1;
        let (mut l, mut r) = (0, 0);
        while r < arr.len() - 1 {
            if l == r {
                if arr[l] == arr[l + 1] {
                    l += 1;
                }
                r += 1;
            } else {
                 if arr[r-1]<arr[r] && arr[r]>arr[r+1]||arr[r-1]>arr[r] && arr[r]<arr[r+1]{
                 r+=1;
                 }else{
                l=r;
                }
            }
            ans = ans.max(r-l+1);
        }
        ans as _
    }
}
// @lc code=end
impl Solution {
    pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
        let mut ans=1;
        let (mut dp0,mut dp1)=(1,1);
        for i in 1..arr.len(){
            if arr[i-1]>arr[i]{
                dp0=dp1+1;
                dp1=1;
            }else if arr[i-1]<arr[i]{
                dp1=dp0+1;
                dp0=1;
            }else{
                dp0=1;
                dp1=1;
            }
            ans=ans.max(dp0.max(dp1));
        }
        ans
    }
}