/*
 * @lc app=leetcode id=43 lang=rust
 *
 * [43] Multiply Strings
 */

// @lc code=start
impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let zero = "0".to_string();
        if num1 == zero || num2 == zero {
            return zero;
        }
        let n1 = num1
            .bytes()
            .rev()
            .map(|x| x - '0' as u8)
            .collect::<Vec<u8>>();
        let n2 = num2
            .bytes()
            .rev()
            .map(|x| x - '0' as u8)
            .collect::<Vec<u8>>();
        let mut ans = vec![0; n1.len() * n2.len() + 1];
        let mut carry = 0;
        let mut pre = 0;
        for i in 0..n1.len() {
            carry = 0;
            for j in 0..n2.len() {
                let t = ans[j + i] + n1[i] * n2[j] + carry;
                ans[j + i] = t % 10;
                carry = t / 10;
                pre = j + i;
            }
            if carry > 0 {
                ans[pre + 1] += carry;
                carry = 0;
            }
        }

        ans.iter()
            .rev()
            .map(|x| (x + '0' as u8) as char)
            .collect::<String>()
            .trim_start_matches('0')
            .to_string()
    }
}
// @lc code=end
