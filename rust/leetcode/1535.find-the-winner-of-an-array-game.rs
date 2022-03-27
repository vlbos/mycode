/*
 * @lc app=leetcode id=1535 lang=rust
 *
 * [1535] Find the Winner of an Array Game
 */

// @lc code=start
impl Solution {
    pub fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
        let mut prev = arr[0].max(arr[1]);
        if k == 1 {
            return prev;
        }
        let mut max = prev;
        let mut consecutive = 1;

        for &curr in arr.iter().skip(2) {
            if prev > curr {
                consecutive += 1;
                if consecutive == k {
                    return prev;
                }
            } else {
                prev = curr;
                consecutive = 1;
            }
            max=max.max(prev);
        }
        max
    }
}
// @lc code=end
