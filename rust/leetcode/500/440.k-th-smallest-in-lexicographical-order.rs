/*
 * @lc app=leetcode id=440 lang=rust
 *
 * [440] K-th Smallest in Lexicographical Order
 */

// @lc code=start
impl Solution {
    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        let get_steps = |curr: i64, n: i64| -> i32 {
            let mut steps = 0;
            let mut first = curr;
            let mut last = curr;
            while first <= n {
                steps += last.min(n) - first + 1;
                first *= 10;
                last = last * 10 + 9;
            }
            steps as _
        };
        let mut curr = 1;
        let mut k = k-1;
        while k > 0 {
            let steps = get_steps(curr as i64, n as i64);
            if steps <= k {
                k -= steps;
                curr += 1;
            } else {
                curr *= 10;
                k -= 1;
            }
        }
        curr
    }
}
// @lc code=end
