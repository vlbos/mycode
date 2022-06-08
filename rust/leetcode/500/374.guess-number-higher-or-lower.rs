/*
 * @lc app=leetcode id=374 lang=rust
 *
 * [374] Guess Number Higher or Lower
 */

// @lc code=start
/** 
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is lower than the guess number
 *			      1 if num is higher than the guess number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        let mut i = 1 as i64;
        let mut j = n as i64;
        while i <= j {
            let mut mid = (i + j) / 2;
            let r = guess(mid as i32);
            if r < 0 {
                j = mid - 1;
            } else if r > 0 {
                i = mid + 1;
            } else {
                return mid as i32;
            }
        }
        0
    }
}
// @lc code=end

