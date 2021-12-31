/*
 * @lc app=leetcode id=167 lang=rust
 *
 * [167] Two Sum II - Input array is sorted
 */

// @lc code=start
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i = 0;
        let mut j = numbers.len() - 1;
        while i < j {
            let v = numbers[i] + numbers[j];
            if v > target {
                j -= 1;
            } else if v < target {
                i += 1;
            } else {
                return vec![(i + 1) as i32, (j + 1) as i32].to_vec();
            }
        }
        vec![].to_vec()
    }
}
// @lc code=end

