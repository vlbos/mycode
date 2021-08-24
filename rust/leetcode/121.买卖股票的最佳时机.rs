/*
 * @lc app=leetcode.cn id=121 lang=rust
 *
 * [121] 买卖股票的最佳时机
 */

// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
            let mut max = 0;
            let mut min = i32::MAX;

            for i in 0..prices.len(){
                    let p = prices[i];
                    if (p<min){
                        min = p;
                    }else if (p-min)>max{
                        max =p-min;
                    }
            }
            max
    }
}
// @lc code=end

