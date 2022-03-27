/*
 * @lc app=leetcode id=1574 lang=rust
 *
 * [1574] Shortest Subarray to be Removed to Make Array Sorted
 */

// @lc code=start
impl Solution {
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut l = 0;
        while l + 1 < n && arr[l] <= arr[l + 1] {
            l += 1;
        }
        let mut r = n - 1;
        while r > 0 && arr[r] >= arr[r - 1] {
            r -= 1;
        }
        let mut ans = r.min(n - l - 1);
        let (mut i, mut j) = (0, r);
        while i <= l && j < n {
            if arr[i] <= arr[j] {
                ans = ans.min(j - i - 1);
                i += 1;
            } else {
                j += 1;
            }
        }
        ans as _
    }
}
// @lc code=end
