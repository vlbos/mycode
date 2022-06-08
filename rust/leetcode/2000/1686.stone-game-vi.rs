/*
 * @lc app=leetcode id=1686 lang=rust
 *
 * [1686] Stone Game VI
 */

// @lc code=start
impl Solution {
    pub fn stone_game_vi(alice_values: Vec<i32>, bob_values: Vec<i32>) -> i32 {
        let mut mp = alice_values
            .iter()
            .zip(&bob_values)
            .enumerate()
            .map(|(i, v)| (v.0 + v.1, i))
            .collect::<Vec<(i32, usize)>>();
        mp.sort_by(|a, b| b.cmp(&a));
        let mut sum = vec![0, 0];
        let values = [&alice_values, &bob_values];
      
        for i in 0..alice_values.len() {
            let j = (i % 2) as usize;
            sum[j] += values[j][mp[i].1];
        }
        let ans =sum[0]-sum[1];
        if ans==0 {
            return 0;
        }
        ans/ans.abs()
    }
}
// @lc code=end
