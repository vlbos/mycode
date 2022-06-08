/*
 * @lc app=leetcode id=179 lang=rust
 *
 * [179] Largest Number
 */

// @lc code=start
impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        if nums.len()==1{
                return nums[0].to_string();
        }
        let mut v = nums.iter().map(|x|x.to_string()).collect::<Vec<String>>();
        v.sort_by(|a,b|b.chars().chain(b.chars()).cmp(a.chars().chain(b.chars())));
        if v[0].as_str()=="0"{
            return "0".to_string();
        }
        v.concat()
    }
}
// @lc code=end

