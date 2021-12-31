/*
 * @lc app=leetcode id=1475 lang=rust
 *
 * [1475] Final Prices With a Special Discount in a Shop
 */

// @lc code=start
impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let mut ans =Vec::new();    
        let mut max = 0;
        for i  in 0..prices.len()-1{
            ans.push(prices[i]);
            for j in i+1..prices.len(){
                if prices[j]<=prices[i]{
                    ans[i]-=prices[j];
                    break;
                }
            }
        }
        ans.push(prices[prices.len()-1]);
        ans
    }
}
// @lc code=end

