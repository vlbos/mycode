/*
 * @lc app=leetcode id=768 lang=rust
 *
 * [768] Max Chunks To Make Sorted II
 */

// @lc code=start
impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut count = std::collections::HashMap::new();
        let mut ans = 0;
        let mut nonzero = 0;
        let mut sorted_arr = arr.clone();
        sorted_arr.sort();
        for (&x, y) in arr.iter().zip(sorted_arr) {
            *count.entry(x).or_insert(0) += 1;
            if *count.get(&x).unwrap() == 0 {
                nonzero -= 1;
            }
            if *count.get(&x).unwrap() == 1 {
                nonzero += 1;
            }
            *count.entry(y).or_insert(0) -= 1;
            if *count.get(&y).unwrap() == -1 {
                nonzero += 1;
            }
            if *count.get(&y).unwrap() == 0 {
                nonzero -= 1;
            }
            if nonzero == 0 {
                ans += 1;
            }
        }
        ans
    }
}
// @lc code=end
