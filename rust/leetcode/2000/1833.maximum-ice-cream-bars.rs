/*
 * @lc app=leetcode id=1833 lang=rust
 *
 * [1833] Maximum Ice Cream Bars
 */

// @lc code=start
impl Solution {
    pub fn max_ice_cream(costs: Vec<i32>, coins: i32) -> i32 {
        let mut f = vec![0; 1000_001];
        for &c in &costs {
            f[c as usize] += 1;
        }
        let mut coins = coins;
        let mut ans = 0;
        for i in 1..f.len() {
            let ii = i as i32;
            if  coins >= ii {
                let cc = f[i].min(coins / ii);
                ans += cc;
                coins -= ii * cc;
            } else {
                break;
            }
        }
        ans
    }
}
// @lc code=end
