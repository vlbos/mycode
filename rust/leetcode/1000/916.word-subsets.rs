/*
 * @lc app=leetcode id=916 lang=rust
 *
 * [916] Word Subsets
 */

// @lc code=start
impl Solution {
    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
         let count =|s:&String|->Vec<i32>{
                let mut v = vec![0;26];
                for c in s.bytes(){
                     v[(c-b'a') as usize]+=1;
                }
                v
            };
            let mut bmax = vec![0;26];
            for w in &words2{
                let c = count(w);
                for i in 0..26{
                    bmax[i]=bmax[i].max(c[i]);
                }
            }
            let mut ans =Vec::new();
            for w in &words1{
                   let c = count(w);
                    if !c.iter().enumerate().any(|(i,&x)|x<bmax[i]){
                        ans.push(w.clone());
                    }
            }
            ans
    }
}
// @lc code=end

