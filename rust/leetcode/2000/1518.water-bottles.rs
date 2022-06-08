/*
 * @lc app=leetcode id=1518 lang=rust
 *
 * [1518] Water Bottles
 */

// @lc code=start
impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut nb = num_bottles;
        let mut ans = nb;
        while nb>=num_exchange{
            ans +=nb/num_exchange;
            nb =  nb/num_exchange+nb%num_exchange;
        }
        ans
    }
}
// @lc code=end

