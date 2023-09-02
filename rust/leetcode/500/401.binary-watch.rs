/*
 * @lc app=leetcode id=401 lang=rust
 *
 * [401] Binary Watch
 */

// @lc code=start
impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
         if turned_on > 8 {
            return vec![].to_vec();
        }
        let mut r = Vec::<String>::new();
        let mut hb = turned_on.min(3);
        let cb = |v| -> Vec<i32> {
            let mut v = v;
            let mut b = vec![0; v + 1];
            for i in 1..=v {
                b[i] = b[i & (i - 1)] + 1;
            }
            b
        };
        let hc = cb(11);
        let mc = cb(59);

        for i in 0..=hb {
            for k in 0..12 {
                for l in 0..60 {
                    if hc[k] == i && mc[l] == turned_on - i {
                        r.push(k.to_string() + if l < 10 { ":0" } else { ":" } + &l.to_string());
                    }
                }
            }
        }
        r
    }
}
// @lc code=end

impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let nums=[1,2,4,8,1,2,4,8,16,32];
        let mut ans=Vec::new();
        let mut visited=vec![false;10];
        fn dfs(start:usize,step:i32,turned_on: i32,nums:&[i32],visited:&mut Vec<bool>,ans:&mut Vec<String>){
            let handle_date=|visited:&Vec<bool>|{
                let mut ans=vec![0;2];
                for (i,&v) in nums.iter().enumerate(){
                    if visited[i]{
                        ans[if i<4{0}else{1}]+=v;
                    }
                }
                ans
            };
            if step==turned_on{
                let date=handle_date(&visited);
                ans.push(format!("{}:{:02}",date[0],date[1]));
                return 
            }
            for i  in start..nums.len(){
                    visited[i]=true;
                    let date=handle_date(&visited);
                    if date[0]>=0 && date[0]<=11 && date[1]>=0 && date[1]<=59{
                        dfs(i+1,step+1,turned_on,nums,visited,ans);
                    }
                    visited[i]=false;
            }
        }
        dfs(0,0,turned_on,&nums,&mut visited,&mut ans);
        ans
    }
}