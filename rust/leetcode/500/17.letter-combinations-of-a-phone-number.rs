/*
 * @lc app=leetcode id=17 lang=rust
 *
 * [17] Letter Combinations of a Phone Number
 */

// @lc code=start
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return Vec::new();
        }
        let mut ans = Vec::new();
        for d in digits.bytes() {
            let i = d - '0' as u8;
            let al=if i==7||i==9{4}else{3};
            if ans.is_empty() {
                for j in 0..al {
                    let c = ('a' as u8 + (i - 2) * 3+if i>7 {1}else{0} + j) as char;
                    ans.push(c.to_string());
                }
            } else {
                let a = ans.clone();
                let c = ('a' as u8 + (i - 2) * 3+if i>7 {1}else{0}) as char;
                for k in 0..ans.len() {
                    ans[k].push(c);
                }
                for j in 1..al {
                    for k in 0..a.len() {
                        let c = ('a' as u8 + (i - 2) * 3 +if i>7 {1}else{0}+ j) as char;
                        ans.push(a[k].clone() + &c.to_string());
                    }
                }
            }
        }
        ans
    }
}
// @lc code=end
