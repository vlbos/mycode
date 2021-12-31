/*
 * @lc app=leetcode id=605 lang=rust
 *
 * [605] Can Place Flowers
 */

// @lc code=start
impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        if flowerbed.len() == 1 && flowerbed[0] == 0 {
            return true;
        }
        let mut r = 0;
        let mut i = 0;
        while i < flowerbed.len() {
            let mut step = 2;
            if flowerbed[i] == 0 {
                if i == 0 {
                    if flowerbed[i + 1] == 0 {
                        r += 1;
                    }
                } else if i == flowerbed.len() - 1 {
                    if flowerbed[i - 1] == 0 {
                        r += 1;
                    }
                } else if flowerbed[i - 1] == 0 && flowerbed[i + 1] == 0 {
                    r += 1;
                } else {
                    step -= 1;
                }
                if r >= n {
                    return true;
                }
            }
            i += step;
        }
        r >= n
    }
}
// @lc code=end

