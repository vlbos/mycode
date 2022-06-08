/*
 * @lc app=leetcode id=525 lang=rust
 *
 * [525] Contiguous Array
 */

// @lc code=start
impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut m = std::collections::HashMap::new();
        let mut counter = 0;
        let mut max = 0;
        m.insert(counter, -1);
        for (i, v) in nums.iter().enumerate() {
            counter += if *v == 0 { -1 } else { 1 };
            if let Some(j) = m.get(&counter) {
                let ii = i as i32 - *j;
                if ii > max {
                    max = ii;
                }
            } else {
                m.insert(counter, i as i32);
            }
        }
        max
    }
}
// @lc code=end
