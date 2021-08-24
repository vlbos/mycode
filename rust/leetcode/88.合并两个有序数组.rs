/*
 * @lc app=leetcode.cn id=88 lang=rust
 *
 * [88] 合并两个有序数组
 */

// @lc code=start
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
            let mut i = nums1.len()-nums2.len();
            let mut j = nums2.len();
            while i>0 && j>0{
                if (nums1[i-1]>nums2[j-1]){
                        nums1[i+j-1]=nums1[i-1];
                        i-=1;
                }
                else{
                    nums1[i+j-1]=nums2[j-1];
                    j-=1;
                }
            }

            while i>0 {
                nums1[i+j-1]=nums1[i-1];
                i-=1;
            }
            while j>0 {
                nums1[i+j-1]=nums2[j-1];
                j-=1;
            }
    }
}
// @lc code=end

