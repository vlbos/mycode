/*
 * @lc app=leetcode id=306 lang=rust
 *
 * [306] Additive Number
 */

// @lc code=start
impl Solution {
    pub fn is_additive_number(num: String) -> bool {
        for i in 1..num.len() - 1 {
            if i > 1 && &num[0..1] == "0" {
                continue;
            }
            for j in i + 1..num.len() {
                if j - i > 1 && &num[i..i + 1] == "0" {
                    continue;
                }
                let mut n1 = (&num[0..i]).parse::<u64>().unwrap();
                let mut n2 = (&num[i..j]).parse::<u64>().unwrap();
                let mut s = (&num[0..j]).to_string();
                loop {
                    let n3 = n1 + n2;
                    n1 = n2;
                    n2 = n3;
                    s += n3.to_string().as_str();
                    if s == num {
                        return true;
                    }
                    if !num.starts_with(&s) {
                        break;
                    }
                }
            }
        }
        false
    }
}
// @lc code=end

