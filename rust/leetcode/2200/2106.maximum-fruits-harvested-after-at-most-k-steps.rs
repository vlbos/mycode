/*
 * @lc app=leetcode id=2106 lang=rust
 *
 * [2106] Maximum Fruits Harvested After at Most K Steps
 */

// @lc code=start
impl Solution {
    pub fn max_total_fruits(fruits: Vec<Vec<i32>>, start_pos: i32, k: i32) -> i32 {
        let mut sum = Vec::new();
        sum.push(0);
        for fruit in &fruits {
            let last = sum[sum.len() - 1];
            sum.push(fruit[1] + last);
        }
        let pos: Vec<i32> = fruits.iter().map(|x| x[0]).collect();
        let mut ans = 0;
        for x in (0..=k).rev() {
            let y = (k - x) / 2;
            let (l, r) = (start_pos - x, start_pos + y);
            let (pl, pr) = (
                pos.partition_point(|&x| x < l),
                pos.partition_point(|&x| x <= r),
            );
            ans = ans.max(sum[pr] - sum[pl]);
            let (l, r) = (start_pos - y, start_pos + x);
            let (pl, pr) = (
                pos.partition_point(|&x| x < l),
                pos.partition_point(|&x| x <= r),
            );
            ans = ans.max(sum[pr] - sum[pl]);
        }
        ans
    }
}
// @lc code=end
