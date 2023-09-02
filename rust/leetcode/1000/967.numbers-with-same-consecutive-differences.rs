/*
 * @lc app=leetcode id=967 lang=rust
 *
 * [967] Numbers With Same Consecutive Differences
 */

// @lc code=start
impl Solution {
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        use std::collections::HashSet;
        let mut cur = (1..10).collect::<HashSet<i32>>();
        for _ in 1..n{
            let mut cur2 = HashSet::new();
            for x in &cur{
                let d = x%10;
                if d-k>=0{
                cur2.insert(x*10+d-k);
                }
                if d+k<=9{
                cur2.insert(x*10+d+k);
                }
            }
            cur=cur2;
        }
        if n==1{
        cur.insert(0);
        }
        cur.iter().cloned().collect::<Vec<i32>>()
    }
}
// @lc code=end

impl Solution {
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        use std::collections::HashSet;
        let mut ans=HashSet::new();
        fn dfs(num:i32,bit:i32,cur:i32,n: i32, k: i32,ans:&mut HashSet<i32>){
            if bit==n{
                ans.insert(num);
                return
            }
            if cur+k<=9{
                dfs(num*10+cur+k,bit+1,cur+k,n,k,ans);
            }
            if cur-k>=0{
                dfs(num*10+cur-k,bit+1,cur-k,n,k,ans);
            }
        }
        for i in 1..=9{
            dfs(i,1,i,n,k,&mut ans);
        }
        ans.into_iter().collect()
    }
}