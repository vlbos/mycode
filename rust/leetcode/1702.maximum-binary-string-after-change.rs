/*
 * @lc app=leetcode id=1702 lang=rust
 *
 * [1702] Maximum Binary String After Change
 */

// @lc code=start
impl Solution {
    pub fn maximum_binary_string(binary: String) -> String {
        let (mut l, mut r) = (0, 0);
        let mut f = true;
        for c in binary.chars() {
            f &= (c == '1');
            if c == '1' {
                if f {
                    l += 1;
                } else {
                    r += 1;
                }
            }
        }
        if l + r < binary.len() - 1 {
            let mut ans = vec!['1';binary.len()];
            let k = binary.len() - r - 1;
            ans[k]='0';
            return ans.iter().collect::<String>();
        }
        binary
    }
}
// @lc code=end
