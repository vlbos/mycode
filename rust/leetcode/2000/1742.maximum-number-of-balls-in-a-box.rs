/*
 * @lc app=leetcode id=1742 lang=rust
 *
 * [1742] Maximum Number of Balls in a Box
 */

// @lc code=start
impl Solution {
    pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
        let sum = |x:i32|->i32{
            let mut ans = 0;
            let mut x = x;
            while x>0{
                ans+=x%10;
                x/=10;
            }
            ans
        };
        let mut d=[0;46];
        let mut max=0;
        for i  in  low_limit..=high_limit{
            let j = sum(i) as usize;
            d[j]+=1;
            max = max.max(d[j]);
        }
        max
    }
}
// @lc code=end

