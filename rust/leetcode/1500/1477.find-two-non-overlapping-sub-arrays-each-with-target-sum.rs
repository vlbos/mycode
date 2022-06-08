/*
 * @lc app=leetcode id=1477 lang=rust
 *
 * [1477] Find Two Non-overlapping Sub-arrays Each With Target Sum
 */

// @lc code=start
impl Solution {
    pub fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
        let n = arr.len();
        let mut len = vec![100001; n + 1];
        let mut ans = 100001;
        let mut r = n - 1;
        let mut sum = 0;
        for l in (0..n).rev() {
            sum += arr[l];
            while sum > target {
                sum -= arr[r];
                r -= 1;
            }
            if sum == target {
                let curlen = r - l + 1;
                ans = ans.min(curlen + len[r + 1]);
                len[l] = curlen.min(len[l + 1]);
            } else {
                len[l] = len[l + 1];
            }
        }
        if ans == 100001 {
            -1
        } else {
            ans as i32
        }
    }
}
// @lc code=end
