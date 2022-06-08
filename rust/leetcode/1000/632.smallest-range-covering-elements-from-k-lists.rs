/*
 * @lc app=leetcode id=632 lang=rust
 *
 * [632] Smallest Range Covering Elements from K Lists
 */

// @lc code=start
impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let n = nums.len();
        let mut indices = std::collections::HashMap::new();
        let (mut min, mut max) = (i32::MAX, i32::MIN);
        for (i, list) in nums.iter().enumerate() {
            for &num in list {
                indices.entry(num).or_insert(Vec::new()).push(i);
                min = min.min(num);
                max = max.max(num);
            }
        }
        let mut freq = vec![0; n];
        let mut inside = 0;
        let (mut left, mut right) = (min, min - 1);
        let (mut best_left, mut best_right) = (min, max);
        while right < max {
            right += 1;
            if indices.contains_key(&right) {
                for &x in indices.get(&right).unwrap() {
                    freq[x] += 1;
                    if freq[x] == 1 {
                        inside += 1;
                    }
                }
                while inside == n {
                    if right - left < best_right - best_left {
                        best_left = left;
                        best_right = right;
                    }
                    if indices.contains_key(&left) {
                        for &x in indices.get(&left).unwrap() {
                            freq[x] -= 1;
                            if freq[x] == 0 {
                                inside -= 1;
                            }
                        }
                    }
                    left += 1;
                }
            }
        }
        vec![best_left, best_right]
    }
}
// @lc code=end
