/*
 * @lc app=leetcode id=470 lang=rust
 *
 * [470] Implement Rand10() Using Rand7()
 */

// @lc code=start
/**
 * The rand7() API is already defined for you.
 * @return a random integer in the range 1 to 7
 * fn rand7() -> i32;
 */

impl Solution {
    pub fn rand10() -> i32 {
        let mut a = 0;
        let mut b = 0;
        let mut idx = 0;
        loop {
            a = rand7();
            b = rand7();
            idx = b + (a - 1) * 7;
            if idx <= 40 {
                return 1 + (idx - 1) % 10;
            }
            a = idx - 40;
            b = rand7();
            idx = b + (a - 1) * 7;
            if idx <= 60 {
                return 1 + (idx - 1) % 10;
            }
            a = idx - 60;
            b = rand7();
            idx = b + (a - 1) * 7;
            if idx <= 20 {
                return 1 + (idx - 1) % 10;
            }
        }
        0
    }
}
// @lc code=end
