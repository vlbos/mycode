/*
 * @lc app=leetcode.cn id=804 lang=rust
 *
 * [804] 唯一摩尔斯密码词
 */

// @lc code=start
impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        let abc = vec![".-","-...","-.-.","-..",".","..-.","--.","....","..",".---","-.-",".-..","--","-.","---",".--.","--.-",".-.","...","-","..-","...-",".--","-..-","-.--","--.."];
        let mut result =Vec::<String>::new();
        let s = |w:&String|->String{
            let mut r = String::new();
            for c in w.chars(){
                r += abc[(c as u8 - 'a' as u8) as usize];
            }
            r
        };
        for w in &words{
            let ww = s(w);
            if !result.contains(&ww){
            result.push(ww);
            }
        }

        result.len() as i32
    }
}
// @lc code=end

