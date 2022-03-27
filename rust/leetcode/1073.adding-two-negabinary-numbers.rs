/*
 * @lc app=leetcode id=1073 lang=rust
 *
 * [1073] Adding Two Negabinary Numbers
 */

// @lc code=start
impl Solution {
    pub fn add_negabinary(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let n1 = arr1.len();
        let n2 = arr2.len();
        let n = n1.max(n2) + 4;
        let mut ans = vec![0; n];
        for i in (0..n1).rev() {
            ans[n1 - 1 - i] += arr1[i];
        }
        for i in (0..n2).rev() {
            ans[n2 - 1 - i] += arr2[i];
        }
        for i in 0..n - 2 {
            let c = ans[i] >> 1;
            ans[i] &= 1;
            ans[i + 1] += c;
            ans[i + 2] += c;
        }
        let mut k = n - 3;
        while k > 0 && ans[k] == 0 {
            k -= 1;
        }
        ans.truncate(k + 1);
        ans.reverse();
        ans
    }
}
// @lc code=end
