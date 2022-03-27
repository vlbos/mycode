/*
 * @lc app=leetcode id=1169 lang=rust
 *
 * [1169] Invalid Transactions
 */

// @lc code=start
impl Solution {
    pub fn invalid_transactions(transactions: Vec<String>) -> Vec<String> {
        let tx = transactions
            .iter()
            .map(|x| x.split(',').collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>();
        let mut ans = Vec::new();
        for (i, v) in tx.iter().enumerate() {
            if v[2].parse::<i32>().unwrap() > 1000 {
                ans.push(v.join(","));
                continue;
            }
            for (j, u) in tx.iter().enumerate() {
                if i == j {
                    continue;
                }

                if v[0] == u[0]
                    && v[3] != u[3]
                    && (v[1].parse::<i32>().unwrap() - u[1].parse::<i32>().unwrap()).abs() <= 60
                {
                    ans.push(v.join(","));
                    break;
                }
            }
        }
        ans
    }
}
// @lc code=end
