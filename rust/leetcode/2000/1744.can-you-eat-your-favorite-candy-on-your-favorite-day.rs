/*
 * @lc app=leetcode id=1744 lang=rust
 *
 * [1744] Can You Eat Your Favorite Candy on Your Favorite Day?
 */

// @lc code=start
impl Solution {
    pub fn can_eat(candies_count: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let n = candies_count.len();
        let mut sum = vec![0; n];
        sum[0] = candies_count[0] as i64;
        for i in 1..n {
            sum[i] = sum[i - 1] + candies_count[i] as i64;
        }
        let mut ans = Vec::new();
        for q in &queries {
            let d = q[1] as i64 + 1;
            let (x1, y1) = (d, d * q[2] as i64);
            let t = q[0] as usize;
            let (x2, y2) = (
                if t == 0 { 0 } else { sum[t - 1] as i64 } + 1,
                sum[t] as i64,
            );
            ans.push(!(x1 > y2 || x2 > y1));
        }
        ans
    }
}
// @lc code=end
