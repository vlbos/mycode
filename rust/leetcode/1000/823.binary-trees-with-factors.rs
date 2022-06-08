/*
 * @lc app=leetcode id=823 lang=rust
 *
 * [823] Binary Trees With Factors
 */

// @lc code=start
impl Solution {
    pub fn num_factored_binary_trees(arr: Vec<i32>) -> i32 {
        let inf: i64 = 1_000_000_007;
        let mut arr = arr;
        arr.sort();
        let index = arr
            .iter()
            .enumerate()
            .map(|(i, v)| (*v , i))
            .collect::<std::collections::HashMap<i32, usize>>();
        let n = arr.len();
        let mut dp = vec![1; n];
        for i in 0..n {
            for j in 0..i {
                if arr[i]%arr[j]==0{
                    let r = arr[i]/arr[j];
                    if index.contains_key(&r){
                        dp[i]=(dp[i]+dp[j]*dp[index[&r]])%inf;
                    }
                }
            }
        }
        (dp.iter().sum::<i64>()%inf)  as i32
    }
}
// @lc code=end
