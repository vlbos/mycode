/*
 * @lc app=leetcode id=47 lang=rust
 *
 * [47] Permutations II
 */

// @lc code=start
impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn back_tracking(
            nums: &Vec<i32>,
            mut vis: &mut Vec<bool>,
            mut ans: &mut Vec<Vec<i32>>,
            mut combine: &mut Vec<i32>,
            idx: usize,
        ) {
            if idx == nums.len() {
                ans.push(combine.clone());
            }
            for i in 0..nums.len() {
                if vis[i] || (i > 0 && nums[i] == nums[i - 1] && !vis[i - 1]) {
                    continue;
                }
                combine.push(nums[i]);
                vis[i] = true;
                back_tracking(&nums, &mut vis, &mut ans, &mut combine, idx + 1);
                combine.pop();
                vis[i] = false;
            }
        }
        let mut nums = nums;
        nums.sort();
        let mut combine = Vec::new();
        let mut vis = vec![false; nums.len()];
        let mut ans: Vec<Vec<i32>> = Vec::new();
        back_tracking(&nums, &mut vis, &mut ans, &mut combine, 0);
        ans
    }
}
// @lc code=end
