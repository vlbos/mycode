/*
 * @lc app=leetcode id=553 lang=rust
 *
 * [553] Optimal Division
 */

// @lc code=start
impl Solution {
    pub fn optimal_division(nums: Vec<i32>) -> String {
        if nums.len()==1{
        return nums[0].to_string();
        }
        if nums.len()==2{
        return format!("{}/{}",nums[0],nums[1]);
        }
        let mut ans = format!("{}/({}",nums[0],nums[1]);
        for n in nums.iter().skip(2){
            ans.push_str(&format!("/{}",n));
        }
        ans.push(')');
        ans
    }
}
// @lc code=end

