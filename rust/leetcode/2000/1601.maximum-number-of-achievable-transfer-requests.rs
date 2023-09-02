/*
 * @lc app=leetcode id=1601 lang=rust
 *
 * [1601] Maximum Number of Achievable Transfer Requests
 */

// @lc code=start
impl Solution {
    pub fn maximum_requests(n: i32, requests: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        for mask in 0..(1 << requests.len()) {
            let cnt = (mask as i32).count_ones();
            if cnt <= ans {
                continue;
            }
            let mut delta = vec![0; n as usize];
            for (i, (x, y)) in requests
                .iter()
                .map(|x| (x[0] as usize, x[1] as usize))
                .enumerate()
            {
                if mask & (1 << i) > 0 {
                    delta[x] += 1;
                    delta[y] -= 1;
                }
            }
            if delta.iter().all(|x| *x == 0) {
                ans = cnt;
            }
        }

        ans as _
    }
}
// @lc code=end
impl Solution {
    pub fn maximum_requests(n: i32, requests: Vec<Vec<i32>>) -> i32 {
        fn dfs(pos:usize,zero:i32,cnt:i32,requests: &Vec<Vec<i32>>,delta:&mut Vec<i32>,ans:&mut i32){
            if pos==requests.len(){
                if zero==delta.len() as i32{
                    *ans=cnt.max(*ans);
                }
                return
            }
            dfs(pos+1,zero,cnt,requests,delta,ans);
            let mut z=0;
            let (x,y)=(requests[pos][0] as usize,requests[pos][1] as usize);
            if delta[x]==0{
                z-=1;
            }
            delta[x]-=1;
            if delta[x]==0{
                z+=1;
            }
            if delta[y]==0{
                z-=1;
            }
            delta[y]+=1;
            if delta[y]==0{
                z+=1;
            }
            dfs(pos+1,zero+z,cnt+1,requests,delta,ans);
            delta[x]+=1;
            delta[y]-=1;
        }
        let mut delta=vec![0;n as usize];
        let mut ans=0;
        dfs(0,n,0,&requests,&mut delta,&mut ans);
        ans
    }
}