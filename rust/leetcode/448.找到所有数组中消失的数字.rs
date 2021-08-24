/*
 * @lc app=leetcode.cn id=448 lang=rust
 *
 * [448] 找到所有数组中消失的数字
 */

// @lc code=start
impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut nums=nums;
        let mut r = Vec::<i32>::new();
        let n = nums.len() as i32;
        for i in 0..nums.len(){
                let k =  ((nums[i]-1)%n)  as usize;
                nums[k]+=n;
        }
        for i in 0..n{
             if nums[i as usize]<=n{
                r.push((i+1) as i32);
            }
        }
        r
    }
}
// @lc code=end

