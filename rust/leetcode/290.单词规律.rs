/*
 * @lc app=leetcode.cn id=290 lang=rust
 *
 * [290] 单词规律
 */

// @lc code=start
impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut c = Vec::<char>::new();
        let mut d = Vec::<String>::new();

        let mut ss = s.split_whitespace();
        let mut p = pattern.chars();
        loop {
            match (p.next(),ss.next()){
            (Some(_p),Some(_ss))=>{
                 match (c.iter().position(|&v| v==_p),d.iter().position(|v| v==_ss)){
                 (Some(_ci),Some(_di))=>{
                     if d[_ci]!=_ss||c[_di]!=_p{
                        return false;
                    }
                },
    (Some(_ci),None)=>if d[_ci]!=_ss{
                        return false;
                    },
 (None,Some(_di))=>if c[_di]!=_p{
                        return false;
                    },
                    _=>                 {  c.push(_p);
                    d.push(_ss.to_string());},
                }
                 },
            (Some(_),None)|(None,Some(_))=>return false,
            _=>break,
            }
            
        }
        true
    }
}
// @lc code=end

