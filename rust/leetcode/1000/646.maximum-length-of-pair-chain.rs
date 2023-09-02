/*
 * @lc app=leetcode id=646 lang=rust
 *
 * [646] Maximum Length of Pair Chain
 */

// @lc code=start
impl Solution {
    pub fn find_longest_chain(pairs: Vec<Vec<i32>>) -> i32 {
        let mut pairs=pairs;
        // pairs.sort_by(|x,y|x[0].cmp(&y[0]));
        // let mut dp =vec![1;pairs.len()];
        // for i in 1..pairs.len(){
        //     for j in 0..i{
        //         if pairs[j][1]<pairs[i][0]{
        //             dp[i]=dp[i].max(dp[j]+1);
        //         }
        //     }
        // }
        // *dp.iter().max().unwrap()
        pairs.sort_by(|x,y|x[1].cmp(&y[1]));
        let mut cur = i32::MIN;
        let mut ans= 0;
        for p in &pairs{
                if cur<p[0]{
                    cur=p[1];
                    ans+=1;
                }
        }
        ans
    }
}
// @lc code=end

impl Solution {
    pub fn find_longest_chain(mut pairs: Vec<Vec<i32>>) -> i32 {
        pairs.sort();
        let n=pairs.len();
        let mut dp=vec![1;n];
        for i in 0..n{
            for j in 0..i{
                if pairs[i][0]>pairs[j][1]{
                    dp[i]=dp[i].max(dp[j]+1);
                }
            }
        }
        dp[dp.len()-1]
    }
}