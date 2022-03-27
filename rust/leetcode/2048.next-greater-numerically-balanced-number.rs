/*
 * @lc app=leetcode id=2048 lang=rust
 *
 * [2048] Next Greater Numerically Balanced Number
 */

// @lc code=start
impl Solution {
    pub fn next_beautiful_number(n: i32) -> i32 {
        let check = |mut i: i32| -> bool {
            let mut cnt = std::collections::HashMap::new();
            while i > 0 {
                *cnt.entry(i % 10).or_insert(0) += 1;
                i /= 10;
            }
            cnt.iter().all(|(&k, &v)| k == v)
        };
        for i in n + 1..=1224444 {
            if check(i) {
                return i;
            }
        }
        0
    }
}
// @lc code=end
