/*
 * @lc app=leetcode id=2094 lang=rust
 *
 * [2094] Finding 3-Digit Even Numbers
 */

// @lc code=start
impl Solution {
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
       let mut ans = Vec::new();
        use std::collections::HashMap;
        let mut freq = HashMap::new();
        for &d in &digits {
            *freq.entry(d).or_insert(0) += 1;
        }
        for i in (100..1000).step_by(2) {
            let mut freq1 = HashMap::new();
            let mut t = i;
            while t>0 {
                *freq1.entry(t%10).or_insert(0) += 1;
                t/=10;
            }
            if freq1.iter().all(|x| *x.1 <= *freq.get(x.0).unwrap_or(&0)) {
                ans.push(i);
            }
        }
        ans
    }
}
// @lc code=end
