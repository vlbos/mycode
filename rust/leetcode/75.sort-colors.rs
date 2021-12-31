/*
 * @lc app=leetcode id=75 lang=rust
 *
 * [75] Sort Colors
 */

// @lc code=start
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut i = 0;
        let mut j = nums.len()-1;
        let mut k = 0;
        let v = vec![0,1];
        for n in &v{
            i = k;
            j = nums.len()-1;
            while i<nums.len() && i<j{
                while i<nums.len() && nums[i]==*n{
                    i+=1;
                    k=i;
                }
                while j<nums.len() && j>=0 && nums[j]!=*n{
                    if j==0{
                        break;
                    }
                    j-=1;
                }
                if i<nums.len()&& j>=0 && j<nums.len() && i<j{
                    let t = nums[i];
                    nums[i]=nums[j];
                    nums[j]=t;
                    k=i+1;
                }
            }
        }
    }
}
// @lc code=end

