/*
 * @lc app=leetcode id=131 lang=rust
 *
 * [131] Palindrome Partitioning
 */

// @lc code=start
impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        fn dfs(s:&Vec<char>,mut f:&mut Vec<Vec<i32>>,mut ans :&mut Vec<Vec<String>>,mut seq :&mut Vec<String>,idx:usize){
            fn is_palindrome(s:& Vec<char>,mut f:&mut Vec<Vec<i32>>,i:usize,j:usize)->i32{
                if f[i][j]!=0{
                return f[i][j];
                }
                if i>=j{
                    f[i][j]=1;
                    return f[i][j];
                }
                f[i][j]=if s[i]==s[j]{is_palindrome(s,f,i+1,j-1)}else{-1};
                return f[i][j]; 
            }
            if idx==s.len(){
                ans.push(seq.clone());
                return;
            }
            for j in idx..s.len(){
                if is_palindrome(s,f,idx,j)==1{
                        seq.push(s[idx..j+1].iter().collect());
                        dfs(s,f,ans,seq,j+1);
                        seq.pop();
                }
            }
        }
        let mut f = vec![vec![0;s.len()];s.len()];
        let s = s.chars().collect::<Vec<char>>();
        let mut ans :Vec<Vec<String>>= Vec::new();
        let mut seq= Vec::new();
        dfs(&s,&mut f ,&mut ans,&mut seq,0);
        ans
    }
}
// @lc code=end

