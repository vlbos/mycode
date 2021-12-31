/*
 * @lc app=leetcode id=1592 lang=rust
 *
 * [1592] Rearrange Spaces Between Words
 */

// @lc code=start
impl Solution {
    pub fn reorder_spaces(text: String) -> String {
        let ss = text.chars().filter(|x| x.is_ascii_whitespace()).count();
        if ss == 0 {
            return text;
        }
        let sv = text.split_ascii_whitespace().collect::<Vec<&str>>();
        if sv.len()==1{
             return sv[0].to_string()+ &(" ".repeat(ss));
        }
        let sc = ss / (sv.len() - 1);
        let mut ans = sv.join(&(" ".repeat(sc)));
        let r = ss % (sv.len() - 1);
        if r != 0 {
            ans += &(" ".repeat(r));
        }
        ans
    }
}
// @lc code=end
