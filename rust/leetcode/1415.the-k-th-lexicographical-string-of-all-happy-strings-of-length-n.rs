/*
 * @lc app=leetcode id=1415 lang=rust
 *
 * [1415] The k-th Lexicographical String of All Happy Strings of Length n
 */

// @lc code=start
impl Solution {
    pub fn get_happy_string(n: i32, k: i32) -> String {
        fn dfs(n: usize, v: &mut Vec<String>, curr: &mut String, s: String) {
            if curr.len() == n {
                v.push(curr.clone());
                return;
            }
            for c in s.chars() {
                let ss = "abc".chars().filter(|x| *x != c).collect();
                curr.push(c);
                dfs(n, v, curr, ss);
                curr.pop();
            }
        }
        let mut v = Vec::new();
        let mut curr = String::new();
        let s = "abc".to_string();
        dfs(n as usize, &mut v, &mut curr, s);
        let k = k as usize;
        if k <= v.len() {
            v[k - 1].clone()
        } else {
            String::new()
        }
    }
}
// @lc code=end
