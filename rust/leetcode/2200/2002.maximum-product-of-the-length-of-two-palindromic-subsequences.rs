/*
 * @lc app=leetcode id=2002 lang=rust
 *
 * [2002] Maximum Product of the Length of Two Palindromic Subsequences
 */

// @lc code=start
impl Solution {
    pub fn max_product(s: String) -> i32 {
        let n = s.len();
        let bs = s.as_bytes();
        let mut total = Vec::new();
        for i in 1..(1 << n) {
            let mut temp = Vec::new();
            for j in 0..n {
                if (1 << j) & i > 0 {
                    temp.push(bs[j]);
                }
            }
            let mut rtemp=temp.clone();
            rtemp.reverse();
            if temp == rtemp {
                total.push(i as i32);
            }
        }
        let mut ans = 0;
        for i in 0..total.len() {
            for j in i + 1..total.len() {
                if total[i] & total[j] == 0 {
                    ans = ans.max((total[i].count_ones()) * (total[j].count_ones()));
                }
            }
        }
        ans as _
    }
}
// @lc code=end
