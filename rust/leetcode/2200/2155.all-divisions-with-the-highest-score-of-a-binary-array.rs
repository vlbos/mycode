/*
 * @lc app=leetcode id=2155 lang=rust
 *
 * [2155] All Divisions With the Highest Score of a Binary Array
 */

// @lc code=start
impl Solution {
    pub fn max_score_indices(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut best = 0;
        let mut ans = vec![0];
        let mut sum = 0;
        for (i, &v) in nums.iter().enumerate() {
            if v == 0 {
                sum += 1;
                if sum > best {
                    best = sum;
                    ans = vec![i as i32 + 1];
                } else if sum == best {
                    ans.push(i as i32 + 1);
                }
            } else {
                sum -= 1;
            }
        }
        ans
    }
}
// @lc code=end
