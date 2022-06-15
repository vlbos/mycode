/*
 * @lc app=leetcode id=2073 lang=rust
 *
 * [2073] Time Needed to Buy Tickets
 */

// @lc code=start
impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let tk = tickets[k as usize];
        for (i, &t) in tickets.iter().enumerate() {
            ans += t.min(tk - if i > k as usize { 1 } else { 0 });
        }
        ans
    }
}
// @lc code=end
