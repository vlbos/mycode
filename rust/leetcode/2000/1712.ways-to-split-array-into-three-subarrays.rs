/*
 * @lc app=leetcode id=1712 lang=rust
 *
 * [1712] Ways to Split Array Into Three Subarrays
 */

// @lc code=start
impl Solution {
    pub fn ways_to_split(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut presum = vec![0; n + 1];
        for (i, &v) in nums.iter().enumerate() {
            presum[i + 1] = presum[i] + v;
        }
        let (mut l, mut r) = (2, 2);
        let mut ans = 0;
        for m in 2..=n {
            l = l.max(m);
            r = r.max(m);
            while r + 1 < n && presum[n] - presum[r + 1] >= presum[r + 1] - presum[m - 1] {
                r += 1;
            }
            while l < n && presum[l] - presum[m - 1] < presum[m - 1] {
                l += 1;
            }
            if r < n
                && l <= r
                && presum[l] - presum[m - 1] >= presum[m - 1]
                && presum[n] - presum[r] >= presum[r] - presum[m - 1]
            {
                ans += (r - l + 1) as i64;
            }
        }
        (ans%1000_000_007) as _
    }
}
// @lc code=end
