/*
 * @lc app=leetcode id=2075 lang=rust
 *
 * [2075] Decode the Slanted Ciphertext
 */

// @lc code=start
impl Solution {
    pub fn decode_ciphertext(encoded_text: String, rows: i32) -> String {
         if rows == 1 {
            return encoded_text;
        }
        let rows = rows as usize;
        let cols = encoded_text.len() / rows;
        let e = encoded_text.as_bytes();
        let mut ans = Vec::new();
        for i in 0..cols {
            let (mut r, mut c) = (0, i);
            while r < rows && c < cols {
                ans.push(e[r * cols + c]);
                r += 1;
                c += 1;
            }
        }
        while !ans.is_empty() && ans[ans.len() - 1] == b' ' {
            ans.pop();
        }
        String::from_utf8(ans).unwrap()
    }
}
// @lc code=end
