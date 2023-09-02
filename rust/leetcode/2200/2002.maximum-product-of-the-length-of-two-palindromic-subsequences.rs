/*
 * @lc app=leetcode id=2002 lang=rust
 *
 * [2002] Maximum Product of the Length of Two Palindromic Subsequences
 */

// @lc code=start
impl Solution {
    pub fn max_product(s: String) -> i32 {
        let n = s.len();
        let bs = s.as_bytes();
        let mut total = Vec::new();
        for i in 1..(1 << n) {
            let mut temp = Vec::new();
            for j in 0..n {
                if (1 << j) & i > 0 {
                    temp.push(bs[j]);
                }
            }
            let mut rtemp=temp.clone();
            rtemp.reverse();
            if temp == rtemp {
                total.push(i as i32);
            }
        }
        let mut ans = 0;
        for i in 0..total.len() {
            for j in i + 1..total.len() {
                if total[i] & total[j] == 0 {
                    ans = ans.max((total[i].count_ones()) * (total[j].count_ones()));
                }
            }
        }
        ans as _
    }
}
// @lc code=end
impl Solution {
    pub fn max_product(s: String) -> i32 {
        fn dfs(index:usize,s:&[u8],s1:&mut String,s2:&mut String,ans:&mut i32){
            if check(s1.as_bytes())&&check(s2.as_bytes()){
                *ans=(*ans).max((s1.len()*s2.len()) as i32);
            }
            if index==s.len(){
                return
            }
            s1.push(s[index] as char);
            dfs(index+1,s,s1,s2,ans);
            s1.pop();
             s2.push(s[index] as char);
            dfs(index+1,s,s1,s2,ans);
            s2.pop();
            dfs(index+1,s,s1,s2,ans);
        }
        fn check(s:&[u8])->bool{
            if s.len()<2{
                return true
            }
            let (mut l,mut r)=(0,s.len()-1);
                while l<r{
                    if s[l]!=s[r]{
                        return false
                    }
                    l+=1;
                    r-=1;
                }
                true
            
        }
        let mut ans=0;
        dfs(0,s.as_bytes(),&mut String::new(),&mut String::new(),&mut ans);
        ans
    }
}