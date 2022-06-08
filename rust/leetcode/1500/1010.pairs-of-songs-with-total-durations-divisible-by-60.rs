/*
 * @lc app=leetcode id=1010 lang=rust
 *
 * [1010] Pairs of Songs With Total Durations Divisible by 60
 */

// @lc code=start
impl Solution {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        let mut m = std::collections::HashMap::new();
        let mut ans = 0;
        for &k in &time{
               ans+= *m.entry((60-k%60)%60).or_insert(0);
               *m.entry(k%60).or_insert(0)+=1;
        }
        ans
    }
}
// @lc code=end

