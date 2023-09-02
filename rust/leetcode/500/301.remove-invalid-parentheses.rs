/*
 * @lc app=leetcode id=301 lang=rust
 *
 * [301] Remove Invalid Parentheses
 */

// @lc code=start
impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        use std::collections::HashSet;
        let mut ans = Vec::new();
        let mut curr = HashSet::new();
        curr.insert(s.clone());
        let is_valid = |s: &String| -> bool {
            let mut count = 0;
            for c in s.chars() {
                if c == '(' {
                    count += 1;
                } else if c == ')' {
                    if count == 0 {
                        return false;
                    }
                    count -= 1;
                }
            }
            count == 0
        };
        loop {
            for str in &curr {
                if is_valid(str) {
                    ans.push(str.clone());
                }
            }
            if !ans.is_empty() {
                return ans;
            }
            let mut next = HashSet::new();
            for str in &curr {
                for (i, c) in str.chars().enumerate() {
                    if i > 0 && c == str.chars().nth(i - 1).unwrap() {
                        continue;
                    }
                    if c == '(' || c == ')' {
                        next.insert((str[..i].to_string() + &str[i + 1..]).clone());
                    }
                }
            }
            curr = next;
        }
        ans
    }
}
// @lc code=end
impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let (mut l,mut r)=(0,0);
        for c in s.chars(){
            if c=='('{
                l+=1;
            }else if c==')'{
                if l>0{
                    l-=1;
                }else{
                    r+=1;
                }
            }
        }
        fn helper(i:usize,s:String,l:i32,r:i32,ans:&mut Vec<String> ){
            if l==0 && r==0{
                let mut cnt=0;
                 for c in s.chars(){
                    if c=='('{
                cnt+=1;
            }else if c==')'{
                cnt-=1;
                if cnt<0{
                    break
                }
            }
                     }
                if cnt==0{
                    ans.push(s);
                }
                return
            }
            let bs=s.as_bytes();
            let n=s.len();
            for j in i..n{
                if j>i && bs[j]==bs[j-1]{
                    continue
                }
                if l+r>(n-j) as i32{
                    break
                }
                if l>0 && bs[j]==b'('{
                    helper(j,s[..j].to_string()+&s[j+1..],l-1,r,ans);
                }
                if r>0 && bs[j]==b')'{
                    helper(j,s[..j].to_string()+&s[j+1..],l,r-1,ans);
                }
            }
        }
        let mut ans=Vec::new();
        helper(0,s,l,r,&mut ans);
        ans
    }
}