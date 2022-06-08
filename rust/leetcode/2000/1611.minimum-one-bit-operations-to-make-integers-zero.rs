/*
 * @lc app=leetcode id=1611 lang=rust
 *
 * [1611] Minimum One Bit Operations to Make Integers Zero
 */

// @lc code=start
impl Solution {
    pub fn minimum_one_bit_operations(n: i32) -> i32 {
       let mut rs = vec![0; 32];
        rs[0] = 1i64;
        for i in 1..32 {
            rs[i] = rs[i - 1] * 2 + 1;
        }
        let mut ans = 0;
        let vt: Vec<i32> = (0..32).filter(|x| ((1<< x) & n )> 0).collect();
        let mut f = 1;
        for &v in vt.iter().rev() {
            ans += rs[v as usize] * f;
            f *= -1;
        }
        ans as _
    }
}
// @lc code=end
