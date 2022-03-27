/*
 * @lc app=leetcode id=1673 lang=rust
 *
 * [1673] Find the Most Competitive Subsequence
 */

// @lc code=start
impl Solution {
    pub fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut d = nums.len() as i32 -k;
        let mut s = Vec::new();
        for &v in &nums{
                while !s.is_empty() && *s.iter().last().unwrap()>v && d>0{
                s.pop();
                d-=1;
                }
                s.push(v);

        }
        while s.len() as i32 >k{
        s.pop();
        }
        s
    }
}
// @lc code=end

