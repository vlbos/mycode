/*
 * @lc app=leetcode id=71 lang=rust
 *
 * [71] Simplify Path
 */

// @lc code=start
impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut p = path.split("/").collect::<Vec<&str>>();
        let mut ans = Vec::new();
        for s  in &p{
            if !s.is_empty() && *s!="."{
                if *s==".."{
                    if !ans.is_empty(){
                        ans.remove(ans.len()-1);
                    }
                }else{
                    ans.push(*s);
                }
            }
        }
        let mut a = "/".to_string();
        a+=&ans.join("/");
        a
    }
}
// @lc code=end

