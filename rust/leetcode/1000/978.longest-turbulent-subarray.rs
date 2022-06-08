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
