/*
 * @lc app=leetcode id=1024 lang=rust
 *
 * [1024] Video Stitching
 */

// @lc code=start
impl Solution {
    pub fn video_stitching(clips: Vec<Vec<i32>>, time: i32) -> i32 {
        let mut maxn = vec![0; time as usize];
        for c in &clips {
            if c[0] < time {
                let i = c[0] as usize;
                maxn[i] = maxn[i].max(c[1]);
            }
        }
        let mut ans = 0;
        let mut pre = 0;
        let mut last = 0;
        for i in 0..time {
            last = last.max(maxn[i as usize]);
            if i == last {
                return -1;
            }
            if i == pre {
                ans += 1;
                pre = last;
            }
        }
        ans
    }
}
// @lc code=end
impl Solution {
    pub fn video_stitching(clips: Vec<Vec<i32>>, time: i32) -> i32 {
        let t =time as usize;
        let mut dp=vec![i32::MAX/2;t+1];
        dp[0]=0;
        for i in 1..=t{
            for c in &clips{
                let c0=c[0] as usize;
                if c0<i && i<=c[1] as  usize{
                    dp[i]=dp[i].min(dp[c0]+1);
                }
            }
        }
        if dp[t]==i32::MAX/2{-1}else{dp[t]}
    }
}