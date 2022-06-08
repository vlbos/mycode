/*
 * @lc app=leetcode id=2171 lang=rust
 *
 * [2171] Removing Minimum Number of Magic Beans
 */

// @lc code=start
impl Solution {
    pub fn minimum_removal(beans: Vec<i32>) -> i64 {
        let mut beans = beans;
        beans.sort();
        let total = beans.iter().map(|&x| x as i64).sum::<i64>();
        let mut ans = total;
        let n = beans.len() as i64;
        for (i, &v) in beans.iter().enumerate() {
            ans = ans.min(total - (v as i64) * (n - i as i64));
        }
        ans
    }
}
// @lc code=end
