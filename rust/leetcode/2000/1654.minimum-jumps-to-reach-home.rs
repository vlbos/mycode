/*
 * @lc app=leetcode id=1654 lang=rust
 *
 * [1654] Minimum Jumps to Reach Home
 */

// @lc code=start
impl Solution {
    pub fn minimum_jumps(forbidden: Vec<i32>, a: i32, b: i32, x: i32) -> i32 {
        use std::collections::HashSet;
        let mut f: HashSet<i32> = forbidden.iter().cloned().collect();
        let mut q = std::collections::VecDeque::new();
        q.push_back((0, 0, false));
        while let Some((cur, cnt, used)) = q.pop_front() {
            if cur == x {
                return cnt;
            }
            if cur + a < 6000 && !f.contains(&(cur + a)) {
                f.insert(cur + a);
                q.push_back((cur + a, cnt + 1, false));
            }
            if !used && cur > b && !f.contains(&(cur - b)) {
                q.push_back((cur - b, cnt + 1, true));
            }
        }
        -1
    }
}
// @lc code=end
impl Solution {
    pub fn minimum_jumps(forbidden: Vec<i32>, a: i32, b: i32, x: i32) -> i32 {
        if x==0{
            return 0
        }
        let forb:std::collections::HashSet<i32>=forbidden.into_iter().collect();
        let mut dp=vec![0;6000i32.min(x+(a+b)*3) as usize];
        let n=dp.len();
        let mut i=0;
        while i<n{
            let ii=i as i32;
            if forb.contains(&ii) || (i>0 && dp[i]==0){
                i+=1;
                continue
            }
            let ia=i+a as usize;
            if !forb.contains(&(ii+a)) && ia<n{
                if dp[ia]==0||dp[i]+1<dp[ia]{
                    dp[ia]=dp[i]+1;
                }
            }
            if !forb.contains(&(ii+a)) && !forb.contains(&(ii+a-b)) &&  ii+a>=b && ii+a-b <n as i32{
                let iab=(ii+a-b) as usize;
                if dp[iab]==0||dp[i]+2<dp[iab]{
                    dp[iab]=dp[i]+2;
                    if a<b{
                        i=(ii+a-b-1) as usize;
                    }
                }
            }
            i+=1;
        }
        if dp[x as usize]==0{-1}else{dp[x as usize]}
    }
}