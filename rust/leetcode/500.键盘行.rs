/*
 * @lc app=leetcode.cn id=500 lang=rust
 *
 * [500] 键盘行
 */

// @lc code=start
impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let v =vec!["qwertyuiop","asdfghjkl","zxcvbnm"];
        let mut s = String::new();
        let mut r = Vec::<String>::new();
        for w in words{
            for _v in &v{
                if _v.contains(w.chars().next().unwrap().to_ascii_lowercase()){
                    s = _v.to_string();
                    break;
                }
            }
            let mut b = true;
            for _w in w.chars(){
                if !s.contains(_w.to_ascii_lowercase()){
                    b=false;
                    break;
                }
            }
            if b{
                r.push(w);
            }
        }
        r
    }
}
// @lc code=end

