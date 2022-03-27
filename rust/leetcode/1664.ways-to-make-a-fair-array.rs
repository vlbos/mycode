/*
 * @lc app=leetcode id=1664 lang=rust
 *
 * [1664] Ways to Make a Fair Array
 */

// @lc code=start
impl Solution {
    pub fn ways_to_make_fair(nums: Vec<i32>) -> i32 {
        let mut sum = vec![0, 0];
        for (i, &v) in nums.iter().enumerate() {
            sum[(i % 2) as usize] += v;
        }
        let mut cur = vec![0, 0];
        let mut ans = 0;
        for (i, &v) in nums.iter().enumerate() {
            cur[(i % 2) as usize] += v;

            let rest = [sum[0] - cur[0], sum[1] - cur[1]];
            let ret = [rest[1] + cur[0], rest[0] + cur[1]];
            if ret[(i % 2) as usize] - v == ret[(1 - i % 2) as usize] {
                ans+=1;
            }
        }
        ans
    }
}
// @lc code=end
