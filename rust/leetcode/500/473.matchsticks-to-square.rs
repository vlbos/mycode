/*
 * @lc app=leetcode id=473 lang=rust
 *
 * [473] Matchsticks to Square
 */

// @lc code=start
impl Solution {
    pub fn makesquare(matchsticks: Vec<i32>) -> bool {
        if matchsticks.len() < 4 {
            return false;
        }

        let s = matchsticks.iter().sum::<i32>();
        if s % 4 != 0 {
            return false;
        }
        let ss = s / 4;
        let f = matchsticks.iter().filter(|x| **x > ss).count();
        if f > 0 {
            return false;
        }
        let mut matchsticks = matchsticks;
        matchsticks.sort_by(|a, b| b.cmp(a));

        let mut sums = vec![0; 4];
        fn dfs(idx: usize, sl: i32, matchsticks: &Vec<i32>, sums: &mut Vec<i32>) -> bool {
            if idx == matchsticks.len() {
                for n in sums.iter().skip(1) {
                    if *sums.first().unwrap() != *n {
                        return false;
                    }
                }
                return true;
            }
            for i in 0..sums.len() {
                if sums[i] + matchsticks[idx] <= sl {
                    sums[i] += matchsticks[idx];
                    if dfs(idx + 1, sl, matchsticks, sums) {
                        return true;
                    }
                    sums[i] -= matchsticks[idx];
                }
            }
            false
        }

        dfs(0, ss, &matchsticks, &mut sums)
    }
}
// @lc code=end
impl Solution {
    pub fn makesquare(matchsticks: Vec<i32>) -> bool {
        let sum=matchsticks.iter().sum::<i32>();
        if sum%4>0{
            return false
        }
        let side=sum/4;
        let mut dp=vec![-1;1<<matchsticks.len()];
        dp[0]=0;
        for s in 1..dp.len(){
            for (k,&v) in matchsticks.iter().enumerate(){
                if s&(1<<k)==0{
                    continue
                }
                let s1=s&(!(1<<k));
                if dp[s1]>=0 && dp[s1]+v <=side{
                    dp[s]=(dp[s1]+v)%side;
                    break
                }
            }
        }
        dp[dp.len()-1]==0
    }
}

impl Solution {
    pub fn makesquare(mut matchsticks: Vec<i32>) -> bool {
        let mut len=matchsticks.iter().sum::<i32>();
        if len%4>0{
            return false
        }
        len/=4;
        matchsticks.sort_by_key(|x|-x);
        fn dfs(idx:usize,len:i32,matchsticks: &Vec<i32>,edges:&mut Vec<i32>)->bool{
            if idx==matchsticks.len(){
                return true
            }
            for i in 0..4{
                edges[i]+=matchsticks[idx];
                if edges[i]<=len && dfs(idx+1,len,matchsticks,edges){
                    return true
                }
                edges[i]-=matchsticks[idx];
            }
            false
        }
        let mut edges=vec![0;4];
        dfs(0,len,&matchsticks,&mut edges)
    }
}