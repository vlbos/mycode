/*
 * @lc app=leetcode.cn id=937 lang=rust
 *
 * [937] 重新排列日志文件
 */

// @lc code=start
impl Solution {
    pub fn reorder_log_files(logs: Vec<String>) -> Vec<String> {
        let mut lv = Vec::<String>::new();
        let mut dv = Vec::<String>::new();
        for s in &logs {
            if s.split_ascii_whitespace()
                .nth(1)
                .unwrap()
                .chars()
                .nth(0)
                .unwrap()
                .is_digit(10)
            {
                dv.push((*s).clone());
            } else {
                let si = s.find(" ").unwrap();
                let si0 = &s[..si];
                let s1 = &s[si..];
                if lv.is_empty() || s1 >= &lv[lv.len() - 1][lv[lv.len() - 1].find(" ").unwrap()..] {
                    lv.push((*s).clone());
                } else {
                    for i in 0..lv.len() {
                        let lvii = lv[i].find(" ").unwrap();
                        let lvi0 = &lv[i][..lvii];
                        let lvi = &lv[i][lvii..];
                        if s1 < lvi || (s1 == lvi && si0 < lvi0) {
                            lv.insert(i, (*s).clone());
                            break;
                        } else if i == lv.len() {
                            lv.push((*s).clone());
                            break;
                        }
                    }
                }
            }
        }

        lv.extend(dv);
        lv
    }
}
// @lc code=end
