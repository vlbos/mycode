/*
 * @lc app=leetcode id=1187 lang=rust
 *
 * [1187] Make Array Strictly Increasing
 */

// @lc code=start
impl Solution {
    pub fn make_array_increasing(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let n = arr1.len();
        let mut dp = vec![vec![i32::MAX; n + 1]; n + 1];
        dp[0][0] = -1;
        let mut arr2 = arr2;
        arr2.sort();
        let k = arr2[..].partition_point(|&x| x<=3);

        for i in 1..=n {
            for j in 0..=i {
                if arr1[i - 1] > dp[j][i - 1] {
                    dp[j][i] = arr1[i - 1];
                }
                if j > 0 {
                    let k = arr2.partition_point(|x| *x <= dp[j - 1][i-1]);
                    if k >= 0 && k<arr2.len() {
                        dp[j][i] = dp[j][i].min(arr2[k]);
                    }
                }
                if i == n && dp[j][i] != i32::MAX {
                    return j as _;
                }
            }
        }
        -1
    }
}
// @lc code=end
