/*
 * @lc app=leetcode id=6 lang=rust
 *
 * [6] ZigZag Conversion
 */

// @lc code=start
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows < 2 {
            return s;
        }
        let num_rows=num_rows as usize;
        let len = s.len();
        let rows = 2 * num_rows- 2;
        let getcols = |len: usize| -> usize {
            (len / rows)*(num_rows- 1)
                + if len % rows  >= num_rows {
                    len % rows - num_rows+1
                } else {
                    0
                }
        };
        let vl = getcols(len)+1;
        let mut v = vec![vec![' ';vl];num_rows];
        for (i, c) in s.chars().enumerate() {
            let row = if i%rows>=num_rows {num_rows - i % rows%num_rows - 2}else{i% rows};
            let col = getcols(i);
            v[row][col] = c;
        }
        let mut ans = Vec::new();
        for rv in &v {
            for n in rv {
                if *n != ' ' {
                    ans.push(*n);
                }
            }
        }
        ans.iter().collect()
    }
}
// @lc code=end
