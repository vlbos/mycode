/*
 * @lc app=leetcode id=1224 lang=rust
 *
 * [1224] Maximum Equal Frequency
 */

// @lc code=start
impl Solution {
    pub fn max_equal_freq(nums: Vec<i32>) -> i32 {
        let mut cnt = std::collections::HashMap::new();
        let mut ans = 0;
        let (mut m1, mut m2) = (0, 0);
        let mut l2 = 0;
        for (i, &v) in nums.iter().enumerate() {
            *cnt.entry(v).or_insert(0) += 1;
            let c = *cnt.get(&v).unwrap();
            if c > m1 {
                m2 = m1;
                m1 = c;
            } else if c > m2 {
                m2 = c;
            }
            if c == 2 {
                l2 += 1;
            }
            let l = cnt.len();
            if l == 1
                ||( m1 == m2 + 1 && m2 as usize * l == i)
                || (m1 as usize * (l - 1) == i && l2 < l)
            {
                ans = ans.max(i as i32 + 1);
            }
        }
        ans
    }
}
// @lc code=end
