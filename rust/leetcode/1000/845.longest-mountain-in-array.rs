/*
 * @lc app=leetcode id=845 lang=rust
 *
 * [845] Longest Mountain in Array
 */

// @lc code=start
impl Solution {
    pub fn longest_mountain(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut ans = 0;
        let mut l = 0;
        let mut r = 0;
        while l + 2 < n {
            r = l + 1;
            if l + 1 < n && arr[l] < arr[l + 1] {
                while r + 1 < n && arr[r] < arr[r + 1] {
                    r += 1;
                }
                if r + 1 < n && arr[r] > arr[r + 1] {
                    while r + 1 < n && arr[r] > arr[r + 1] {
                        r += 1;
                    }
                    ans = ans.max(r-l+1);
                } else {
                    r += 1;
                }
            }
            l = r;
        }
        ans as i32
    }
}
// @lc code=end
impl Solution {
    pub fn longest_mountain(arr: Vec<i32>) -> i32 {
        let n=arr.len();
        if n<3{
            return 0
        }
        let (mut dp1,mut dp2)=(vec![1;n],vec![1;n]);
        for i in 1..n{
            if arr[i]>arr[i-1]{
                dp1[i]=dp1[i-1]+1;
            }
        }
        for i in (0..n-1).rev(){
            if arr[i]>arr[i+1]{
                dp2[i]=dp2[i+1]+1;
            }
        }
        let mut ans=0;
        for i in 1..n-1{
            if dp1[i]>1 && dp2[i]>1{
                let cur=dp1[i]+dp2[i]-1;
                ans=ans.max(cur);
            }
        }
        ans
    }
}