/*
 * @lc app=leetcode id=1567 lang=rust
 *
 * [1567] Maximum Length of Subarray With Positive Product
 */

// @lc code=start
impl Solution {
    pub fn get_max_len(nums: Vec<i32>) -> i32 {
        let (mut positive, mut negative) = (0, 0);
        let mut ans = 0;
        for &v in &nums {
            if v > 0 {
                positive += 1;
                negative = if negative > 0 { negative + 1 } else { 0 };
            } else if v < 0 {
                let newpositive = if negative > 0 { negative + 1 } else { 0 };
                let newnegative = positive + 1;
                positive = newpositive;
                negative = newnegative;
            } else {
                positive = 0;
                negative = 0;
            }
            ans = ans.max(positive);
        }
        ans
    }
}
// @lc code=end
