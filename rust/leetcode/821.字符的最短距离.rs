/*
 * @lc app=leetcode.cn id=821 lang=rust
 *
 * [821] 字符的最短距离
 */

// @lc code=start
impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let mut r = Vec::<i32>::new();
        let mut index = Vec::<usize>::new();
        for (i, cs) in s.chars().enumerate() {
            if cs == c {
                index.push(i);
            }
        }
        let dis = |i: usize| -> i32 {
            let mut r = usize::MAX;
            for j in &index {
                let n = if i > *j { i - *j } else { *j - i };
                if n < r {
                    r = n;
                }
            }
            r as i32
        };
        for (i, cs) in s.chars().enumerate() {
            r.push(dis(i));
        }
        r
    }
}
// @lc code=end
