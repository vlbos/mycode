/*
 * @lc app=leetcode id=845 lang=rust
 *
 * [845] Longest Mountain in Array
 */

// @lc code=start
impl Solution {
    pub fn longest_mountain(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut ans = 0;
        let mut l = 0;
        let mut r = 0;
        while l + 2 < n {
            r = l + 1;
            if l + 1 < n && arr[l] < arr[l + 1] {
                while r + 1 < n && arr[r] < arr[r + 1] {
                    r += 1;
                }
                if r + 1 < n && arr[r] > arr[r + 1] {
                    while r + 1 < n && arr[r] > arr[r + 1] {
                        r += 1;
                    }
                    ans = ans.max(r-l+1);
                } else {
                    r += 1;
                }
            }
            l = r;
        }
        ans as i32
    }
}
// @lc code=end
