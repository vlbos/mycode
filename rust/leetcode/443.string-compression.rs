/*
 * @lc app=leetcode id=443 lang=rust
 *
 * [443] String Compression
 */

// @lc code=start
impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        if chars.len() == 1 {
            return chars.len() as i32;
        }
        let mut pre = chars[0];
        let mut preindex = 0;
        let mut l = 0;
        for i in 1..chars.len() {
            if pre != chars[i] {
                let cnt = i - preindex;
                if cnt > 1 {
                    chars[l] = chars[preindex];
                    l += 1;
                    if cnt > 9 {
                        let cnts = cnt.to_string().chars().collect::<Vec<char>>();
                        for c in cnts {
                            chars[l] = c;
                            l += 1;
                        }
                    } else {
                        chars[l] = (b'0' + cnt as u8) as char;
                        l += 1;
                    }
                } else {
                    chars[l] = chars[preindex];
                    l += 1;
                }
                pre = chars[i];
                preindex = i;
            }
        }
        let cnt = chars.len() - preindex;
        if cnt > 1 {
            chars[l] = chars[preindex];
            l += 1;
            if cnt > 9 {
                let cnts = cnt.to_string().chars().collect::<Vec<char>>();
                for c in cnts {
                    chars[l] = c;
                    l += 1;
                }
            } else {
                chars[l] = (b'0' + cnt as u8) as char;
                l += 1;
            }
        } else {
            chars[l] = chars[preindex];
            l += 1;
        }
        l as i32
    }
}
// @lc code=end
