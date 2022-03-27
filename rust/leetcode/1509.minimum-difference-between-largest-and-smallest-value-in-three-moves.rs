/*
 * @lc app=leetcode id=1509 lang=rust
 *
 * [1509] Minimum Difference Between Largest and Smallest Value in Three Moves
 */

// @lc code=start
impl Solution {
    pub fn min_difference(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 4 {
            return 0;
        }
        let p = 1000_000_000;
        let (mut max4, mut min4) = (vec![-p; 4], vec![p; 4]);
        for &v in &nums {
            let mut add = 0;
            while add < 4 && max4[add] > v {
                add += 1;
            }
            if add < 4 {
                for i in (add + 1..4).rev() {
                    max4[i] = max4[i - 1];
                }
                max4[add] = v;
            }
            add = 0;
            while add < 4 && min4[add] < v {
                add += 1;
            }
            if add < 4 {
                for i in (add + 1..4).rev() {
                    min4[i] = min4[i - 1];
                }
                min4[add] = v;
            }
        }
        let mut ans = 2 * p;
        for i in 0..4 {
            ans = ans.min(max4[i] - min4[3 - i]);
        }
        ans
    }
}
// @lc code=end
