/*
 * @lc app=leetcode id=456 lang=rust
 *
 * [456] 132 Pattern
 */

// @lc code=start
impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut s = Vec::new();
        s.push(*nums.last().unwrap());
        let mut max_2=i32::MIN;
        for i in (0..nums.len()-1).rev(){
                if nums[i]<max_2{
                    return true;
                }
                while !s.is_empty() && nums[i]>*s.last().unwrap(){
                    max_2=*s.last().unwrap();
                    s.pop();
                }
                if nums[i]>max_2{
                    s.push(nums[i]);
                }
        }
        false
    }
}
// @lc code=end

