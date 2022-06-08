/*
 * @lc app=leetcode id=401 lang=rust
 *
 * [401] Binary Watch
 */

// @lc code=start
impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
         if turned_on > 8 {
            return vec![].to_vec();
        }
        let mut r = Vec::<String>::new();
        let mut hb = turned_on.min(3);
        let cb = |v| -> Vec<i32> {
            let mut v = v;
            let mut b = vec![0; v + 1];
            for i in 1..=v {
                b[i] = b[i & (i - 1)] + 1;
            }
            b
        };
        let hc = cb(11);
        let mc = cb(59);

        for i in 0..=hb {
            for k in 0..12 {
                for l in 0..60 {
                    if hc[k] == i && mc[l] == turned_on - i {
                        r.push(k.to_string() + if l < 10 { ":0" } else { ":" } + &l.to_string());
                    }
                }
            }
        }
        r
    }
}
// @lc code=end

