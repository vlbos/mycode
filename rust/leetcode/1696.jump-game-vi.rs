/*
 * @lc app=leetcode id=1696 lang=rust
 *
 * [1696] Jump Game VI
 */

// @lc code=start
impl Solution {
    pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
        let mut q = std::collections::VecDeque::new();
        q.push_back((nums[0], 0));
        let mut ans = nums[0];
        for i in  1..nums.len(){
            while !q.is_empty() && i - q.front().unwrap().1 > k as usize{
                q.pop_front();
            }
            ans = q.front().unwrap().0 + nums[i];
            while !q.is_empty() && q.back().unwrap().0 <= ans {
                q.pop_back();
            }
            q.push_back((ans, i));
        }
        ans
    }
}
// @lc code=end
