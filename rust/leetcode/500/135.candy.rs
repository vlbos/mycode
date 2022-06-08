/*
 * @lc app=leetcode id=135 lang=rust
 *
 * [135] Candy
 */

// @lc code=start
impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut n = ratings.len();
        let mut ans = 1;
        let (mut inc, mut dec) = (1, 0);
        let mut pre = 1;
        for i in 1..n {
            if ratings[i] >= ratings[i - 1] {
                dec = 0;
                pre = if ratings[i] == ratings[i - 1] {
                    1
                } else {
                    pre + 1
                };
                ans += pre;
                inc = pre;
            } else {
                dec += 1;
                if dec == inc {
                    dec += 1;
                }
                ans += dec;
                pre = 1;
            }
        }
        ans
    }
}
// @lc code=end
