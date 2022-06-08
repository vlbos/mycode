/*
 * @lc app=leetcode id=1576 lang=rust
 *
 * [1576] Replace All ?'s to Avoid Consecutive Repeating Characters
 */

// @lc code=start
impl Solution {
    pub fn modify_string(s: String) -> String {
        if s.len() == 1 {
            if s == "?" {
                return "a".to_string();
            } else {
                return s;
            }
        }
        let mut ss = s.chars().collect::<Vec<char>>();
        let l = ss.len();
        let mut p = 'a' as u8;
        let q = 'z' as u8;
        for i in 1..l - 1 {
            let c = ss[i];
            if c == '?' {
                p = 'a' as u8;
                if i > 0 && i < ss.len() - 1 {
                    while p <= q && (p == ss[i - 1] as u8 || p == ss[i + 1] as u8) {
                        p += 1;
                    }
                    if p <= q {
                        ss[i] = p as char;
                    }
                }
            }
        }
        p = 'a' as u8;
        let mut i = 0;
        if ss[i] == '?' {
            while p <= q && p == ss[i + 1] as u8 {
                p += 1;
            }
            if p <= q {
                ss[i] = p as char;
            }
        }

        p = 'a' as u8;
        i = ss.len() - 1;
        if ss[i] == '?' {
            while p <= q && (p == ss[i - 1] as u8) {
                p += 1;
            }
            if p <= q {
                ss[i] = p as char;
            }
        }
        ss.iter().collect()
    }
}
// @lc code=end
