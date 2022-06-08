/*
 * @lc app=leetcode id=491 lang=rust
 *
 * [491] Increasing Subsequences
 */

// @lc code=start
impl Solution {
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn dfs(nums: &Vec<i32>, idx: usize, last: i32, t: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
            if idx == nums.len() {
                if t.len() >= 2 {
                    ans.push(t.clone());
                }
                return;
            }
            if last <= nums[idx] {
                t.push(nums[idx]);
                dfs(nums, idx + 1, nums[idx], t, ans);
                t.pop();
            }
            if last != nums[idx] {
                dfs(nums, idx + 1, last, t, ans);
            }
        }
        let mut ans: Vec<Vec<i32>> = Vec::new();
        let mut t: Vec<i32> = Vec::new();
        dfs(&nums, 0, i32::MIN, &mut t, &mut ans);
        ans
    }
}
// @lc code=end
