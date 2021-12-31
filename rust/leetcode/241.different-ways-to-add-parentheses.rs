/*
 * @lc app=leetcode id=241 lang=rust
 *
 * [241] Different Ways to Add Parentheses
 */

// @lc code=start
impl Solution {
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        fn dac(exp:&[char])-> Vec<i32>{
             if exp.len()<=2{
                if exp.iter().filter(|x|x.is_ascii_digit()).count()==exp.len(){
                    return vec![exp.iter().collect::<String>().parse::<i32>().unwrap()];
                }
             }
             let mut res = Vec::new();
             for (i,c) in exp.iter().enumerate(){
                if !c.is_ascii_digit(){
                    let l = dac(&exp[..i]);
                    let r = dac(&exp[i+1..]);
                    for li in &l{
                        for ri in &r{
                             res.push(match c {'+'=>li+ri,'-'=>li-ri,_=>li*ri});
                        }
                    }
                }
             }
            res
        }
        dac(&expression.chars().collect::<Vec<char>>())
    }
}
// @lc code=end

