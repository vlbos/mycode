/*
 * @lc app=leetcode id=1862 lang=rust
 *
 * [1862] Sum of Floored Pairs
 */

// @lc code=start
impl Solution {
    pub fn sum_of_floored_pairs(nums: Vec<i32>) -> i32 {
        let upper = *nums.iter().max().unwrap() as usize;
        let mut cnt = vec![0; upper + 1];
        for &num in &nums {
            cnt[num as usize] += 1;
        }
        let mut pre = vec![0; upper + 1];
        for i in 1..=upper {
            pre[i] = pre[i - 1] + cnt[i];
        }
        let mut ans = 0;
        for y in 1..=upper {
            if cnt[y] == 0 {
                continue;
            }
            let mut d = 1;
            while d * y <= upper {
                ans += (cnt[y] as i64)
                    * (d as i64)
                    * (pre[upper.min((d + 1) * y - 1)] - pre[d * y - 1]);
                d += 1;
            }
        }
        ( ans % 1_000_000_007) as _
    }
}
// @lc code=end
