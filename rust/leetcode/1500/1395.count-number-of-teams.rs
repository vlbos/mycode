/*
 * @lc app=leetcode id=1395 lang=rust
 *
 * [1395] Count Number of Teams
 */

// @lc code=start
impl Solution {
    pub fn num_teams(rating: Vec<i32>) -> i32 {
        let n = rating.len();
        let mut v = vec![vec![0;2];n];
        let mut ans = 0;
        for i in 1..n{
            for j in 0..i{
                if rating[i]<rating[j]{
                    ans+=v[j][0];
                    v[i][0]+=1;
                }
                if rating[i]>rating[j]{
                    ans+=v[j][1];
                    v[i][1]+=1;
                }
            }
        }
        ans
    }
}
// @lc code=end

