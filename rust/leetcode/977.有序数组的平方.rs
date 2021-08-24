/*
 * @lc app=leetcode.cn id=977 lang=rust
 *
 * [977] 有序数组的平方
 */

// @lc code=start
impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut n =nums.clone();
        let mut i = 0;
        let mut j = nums.len()-1;
        let mut k = n.len()-1;
        while  i<=j  {
                if nums[i]*nums[i]>nums[j]*nums[j]{
                    n[k]=nums[i]*nums[i];
                    i+=1;
                }else{
                    n[k]=nums[j]*nums[j];
                    if j==0{break;}
                    j-=1;
                }
                k-=1;
        }
        n
    }
}
// @lc code=end

