/*
 * @lc app=leetcode id=537 lang=rust
 *
 * [537] Complex Number Multiplication
 */

// @lc code=start
impl Solution {
    pub fn complex_number_multiply(num1: String, num2: String) -> String {
        let mut n1 = num1
            .split('+')
            .map(|x| x.trim_end_matches('i').parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let mut n2 = num2
            .split('+')
            .map(|x| x.trim_end_matches('i').parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let mut ans = vec![0; 2];
        let mut t = vec![0; 3];
        for i in 0..2 {
            for j in 0..2 {
                t[i + j] += n1[i] * n2[j];
            }
        }
        ans[0]=t[0]-t[2];
        ans[1]=t[1];
        format!("{}+{}i",ans[0],ans[1])
    }
}
// @lc code=end
