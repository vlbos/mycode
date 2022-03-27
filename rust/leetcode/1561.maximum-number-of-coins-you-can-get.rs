/*
 * @lc app=leetcode id=1561 lang=rust
 *
 * [1561] Maximum Number of Coins You Can Get
 */

// @lc code=start
impl Solution {
    pub fn max_coins(piles: Vec<i32>) -> i32 {
        let mut piles=piles;
        piles.sort();
        let r = piles.len()/3;
        let mut i = piles.len()-2;
        let mut ans = 0;
        for _ in 0..r{
            ans+=piles[i];
            i-=2;
        }
       ans
    }
}
// @lc code=end

