/*
 * @lc app=leetcode id=78 lang=rust
 *
 * [78] Subsets
 */

// @lc code=start
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();
        fn back_track(nums: &Vec<i32>, mut ans:&mut Vec<Vec<i32>>, mut seq:&mut Vec<i32>, idx: usize) {
            if idx == nums.len() {
                ans.push(seq.clone());
                return;
            }
            back_track(nums, ans, seq, idx + 1);
            seq.push(nums[idx]);
            back_track(nums, ans, seq, idx + 1);
            seq.pop();
        }
        let mut seq = Vec::new();
        back_track(&nums, &mut ans, &mut seq, 0);
        ans
    }
}
// @lc code=end
