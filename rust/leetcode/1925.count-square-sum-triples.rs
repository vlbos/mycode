/*
 * @lc app=leetcode id=1925 lang=rust
 *
 * [1925] Count Square Sum Triples
 */

// @lc code=start
impl Solution {
    pub fn count_triples(n: i32) -> i32 {
        let mut ans = 0;
        for k in 1..=n {
            for i in 1..k {
                for j in 1..k {
                    if i * i + j * j == k * k {
                        ans += 1;
                    }
                }
            }
        }
        ans
    }
}
// @lc code=end
