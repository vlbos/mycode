/*
 * @lc app=leetcode id=1247 lang=rust
 *
 * [1247] Minimum Swaps to Make Strings Equal
 */

// @lc code=start
impl Solution {
    pub fn minimum_swap(s1: String, s2: String) -> i32 {
        let (xy, yx) = s1
            .chars()
            .zip(s2.chars())
            .fold((0, 0), |(xy, yx), (c1, c2)| {
                if c1 == c2 {
                    (xy, yx)
                } else if c1 == 'x' {
                    (xy + 1, yx)
                } else {
                    (xy, yx + 1)
                }
            });
          if (xy + yx) & 1 == 0 {((xy+1)>>1) + ((yx+1)>>1)}else{-1}
    }
}
// @lc code=end

