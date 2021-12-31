/*
 * @lc app=leetcode id=227 lang=rust
 *
 * [227] Basic Calculator II
 */

// @lc code=start
impl Solution {
    pub fn calculate(s: String) -> i32 {
        let sv = s.chars().collect::<Vec<char>>();
        let mut stack = Vec::new();
        let mut d = String::new();
        let mut presign='+';
        for (i,c) in sv.iter().enumerate() {
            if c.is_ascii_digit() {
                d.push(*c);
            } 
            if  (!c.is_ascii_digit() && *c != ' ' ) || i==sv.len()-1{
                let n = d.parse::<i32>().unwrap();
                match presign  {
                   '+'=> stack.push(n),
                    '-'=> stack.push(-n),
                    '*'=> if let Some(mut l)=stack.last_mut(){*l*=n;},
                    _=> if let Some(mut l)=stack.last_mut(){*l/=n;},
                }
                d = String::new();
                presign=*c;
            }
        }
        stack.iter().sum()
    }
}
// @lc code=end
