/*
 * @lc app=leetcode id=1326 lang=rust
 *
 * [1326] Minimum Number of Taps to Open to Water a Garden
 */

// @lc code=start
impl Solution {
    pub fn min_taps(n: i32, ranges: Vec<i32>) -> i32 {
        let mut prev: Vec<i32> = (0..=n).collect();
        for i in 0..=n {
            let (l, r) = (
                0i32.max(i - ranges[i as usize]),
                n.min(i + ranges[i as usize]),
            );
            prev[r as usize] = l.min(prev[r as usize]);
        }
        let (mut breakpoint, mut furthest) = (n, i32::MAX);
        let mut ans = 0;
        for i in (1..=n).rev() {
            furthest = furthest.min(prev[i as usize]);
            if i == breakpoint {
                if furthest >= i {
                    return -1;
                }
                breakpoint = furthest;
                ans += 1;
            }
        }
        ans
    }
}
// @lc code=end
impl Solution {
    pub fn min_taps(n: i32, ranges: Vec<i32>) -> i32 {
        let mut intervals=Vec::new();
        for (i,&v) in ranges.iter().enumerate(){
            let i=i as i32;
            intervals.push((0i32.max(i-v),n.min(i+v)));
        }
        intervals.sort();
        let mut dp=vec![i32::MAX;n as usize+1];
        dp[0]=0;
        for &(start,end) in &intervals{
            if dp[start as usize]==i32::MAX{
                return -1
            }
            for j in start..=end{
                dp[j as usize]=dp[j as usize].min(dp[start as usize]+1);
            }
        }
        dp[n as usize]
    }
}