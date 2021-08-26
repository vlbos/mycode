/*
 * @lc app=leetcode.cn id=680 lang=rust
 *
 * [680] 验证回文字符串 Ⅱ
 */

// @lc code=start
impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let sb = s.into_bytes();
        let mut i = 0;
        let mut j = sb.len() - 1;
        let mut b = true;
        while i < j {
            if sb[i] != sb[j] {
                if !b || (i + 1 < sb.len() && j > 0 && sb[i + 1] != sb[j] && sb[i] != sb[j - 1]) {
                    return false;
                }
                if b && i + 1 < sb.len() && sb[i + 1] == sb[j] && j > 0 && sb[i] == sb[j - 1] {
                    let ii = i;
                    let jj = j;
                    i += 1;
                    while i < j {
                        if sb[i] != sb[j] {
                            break;
                        }
                        i += 1;
                        j -= 1;
                    }
                    if i >= j || j - i < 2 {
                        return true;
                    }
                    i = ii;
                    j = jj;
                    j -= 1;
                    while i < j {
                        if sb[i] != sb[j] {
                            return false;
                        }
                        i += 1;
                        j -= 1;
                    }
                } else {
                    if b && i + 1 < sb.len() && sb[i + 1] == sb[j] {
                        i += 1;
                        b = false;
                    }
                    if b && j > 0 && sb[i] == sb[j - 1] {
                        j -= 1;
                        b = false;
                    }
                }
            }
            i += 1;
            j -= 1;
        }
        i >= j || j - i < 2
    }
}
// @lc code=end
