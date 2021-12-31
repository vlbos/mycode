/*
 * @lc app=leetcode id=1854 lang=rust
 *
 * [1854] Maximum Population Year
 */

// @lc code=start
impl Solution {
    pub fn maximum_population(logs: Vec<Vec<i32>>) -> i32 {
        let mut  y = vec![0;101];
        for l in &logs{
            for i in l[0]..l[1]{
                y[i as usize-1950]+=1;
            }
        }
        let mut max = 0;
        let mut ans = 0;
        for (i,n) in y.iter().enumerate(){
            if *n>max{
                max = *n;
                ans = i;
            }
        }
        ans as i32+1950
    }
}
// @lc code=end

