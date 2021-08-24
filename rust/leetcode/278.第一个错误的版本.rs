/*
 * @lc app=leetcode.cn id=278 lang=rust
 *
 * [278] 第一个错误的版本
 */

// @lc code=start
// The API isBadVersion is defined for you.
// isBadVersion(versions:i32)-> bool;
// to call it use self.isBadVersion(versions)

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
       if n==1 {
        return 1;
        }
        let mut mid = n/2;
        let mut start = 1;
        let mut end = n;
 
        while start<=end{
            mid = start+(end-start)/2;
             if self.isBadVersion(mid){
                if mid>1 && !self.isBadVersion(mid-1){
                    return mid;
                }
                end =mid-1;
             }
            else{
                start =mid+1;
            }
        }
        mid
    }
}
// @lc code=end

