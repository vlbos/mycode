/*
 * @lc app=leetcode id=798 lang=rust
 *
 * [798] Smallest Rotation with Highest Score
 */

// @lc code=start
impl Solution {
    pub fn best_rotation(nums: Vec<i32>) -> i32 {
         let n = nums.len();
        let mut diffs = vec![0; n];
        for i in 0..n {
            let low = (i + 1)%n;
            let high = (i + 1 + n - nums[i] as usize) % n;
            diffs[low] += 1;
            diffs[high] -= 1;
            if low >= high {
                diffs[0] += 1;
            }
        }
        let mut best_index = 0;
        let mut max_score = 0;
        let mut score = 0;
        for i in 0..n {
            score += diffs[i];
            if score > max_score {
                best_index = i;
                max_score = score;
            }
        }
        best_index as _
    }
}
// @lc code=end
