/*
 * @lc app=leetcode id=1748 lang=rust
 *
 * [1748] Sum of Unique Elements
 */

// @lc code=start
impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let mut cnt = vec![0; 101];
        for n in &nums {
            cnt[*n as usize] += 1;
        }
        let mut ans = 0;
        for (i, n) in cnt.iter().enumerate() {
            if *n == 1 {
                ans += i as i32;
            }
        }
        ans
    }
}
// @lc code=end
