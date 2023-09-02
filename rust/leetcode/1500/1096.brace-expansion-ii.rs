/*
 * @lc app=leetcode id=1096 lang=rust
 *
 * [1096] Brace Expansion II
 */

// @lc code=start
impl Solution {
    pub fn brace_expansion_ii(expression: String) -> Vec<String> {
        let (mut stack, mut ans, mut cur) = (Vec::new(), Vec::new(), vec![String::new()]);
        for c in expression.chars() {
            if c.is_ascii_lowercase() {
                cur = cur.into_iter().map(|a| a + c.to_string().as_str()).collect();
            } else if c == '{' {
                stack.push((ans, cur));
                ans = Vec::new();
                cur = vec![String::new()];
            } else if c == '}' {
                let (pre_ans, pre) = stack.pop().unwrap();
                cur = ans
                    .into_iter()
                    .chain(cur.into_iter())
                    .map(|c| {
                        pre.clone().into_iter()
                            .map(|p| p + c.as_str())
                            .collect::<Vec<String>>()
                    })
                    .flatten()
                    .collect();
                ans = pre_ans;
            } else if c == ',' {
                ans.extend(cur);
                cur = vec![String::new()];
            }
        }
         ans.extend(cur);
        ans.sort();
        ans.dedup();
        ans
    }
}
// @lc code=end
impl Solution {
    pub fn brace_expansion_ii(expression: String) -> Vec<String> {
        use std::collections::HashSet;
        fn dfs(expression: String,ans:&mut HashSet<String>){
            if let Some(j)=expression.find('}'){
                 let i=expression[..j].rfind('{').unwrap();
                 let (a,c)=(&expression[..i],&expression[j+1..]);
                 for b in expression[i+1..j].split(','){
                     dfs(a.to_string()+b+c,ans);
                 }
            }else{
                ans.insert(expression);
            }

        }
        let mut ans=HashSet::new();
        dfs(expression,&mut ans);
        let mut ans:Vec<String>=ans.into_iter().collect();
        ans.sort();
        ans
    }
}