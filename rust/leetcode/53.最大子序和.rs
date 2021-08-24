/*
 * @lc app=leetcode.cn id=53 lang=rust
 *
 * [53] 最大子序和
 */

// @lc code=start
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
            if nums.len()==1{
                return nums[0];
            }
            let mut max = i32::MIN;
            let mut sum  =0;
            let mut fsum = Vec::<i32>::new();
            for i in 0..nums.len(){
                sum+=nums[i];
                fsum.push(sum);
                if sum>max{
                    max =sum;
                }
                if nums[i]>max{
                    max =nums[i];
                }
            }

            for i in 1..fsum.len(){
                for j in 0..i{
                   let s =  fsum[i]-fsum[j];
                if s>max{
                max =s;
                }
                }
            }

            max
    }
}
// @lc code=end

