/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

// @lc code=start
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nm = std::collections::HashMap::new();
        for (i, n) in nums.iter().enumerate() {
            // for j in i+1..nums.len(){
            //     if n+nums[j] == target{
            if let Some(j) = nm.get(&(target - n)) {
                return vec![i as i32, *j as i32];
            }
            nm.insert(n, i);
        }
        vec![0]
    }
}
// @lc code=end
