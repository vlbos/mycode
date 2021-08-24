/*
 * @lc app=leetcode.cn id=704 lang=rust
 *
 * [704] 二分查找
 */

// @lc code=start
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len()==1 {
            return if  nums[0]==target{
            0}else{-1};
        }
        let mut i =0;
        let mut j = nums.len()-1;
        while i<=j{
            let mid = (i+j)/2;
            if nums[mid]==target{
            return mid as i32;
            }else if nums[mid]>target{
                    if mid > 0{
                        j=mid-1;
                    }else{
                    break;
                    }
            }else {
                if mid+1<nums.len(){
                i=mid+1;
}else{break;}
            }
        }
        -1
    }
}
// @lc code=end

