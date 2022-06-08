/*
 * @lc app=leetcode id=1553 lang=rust
 *
 * [1553] Minimum Number of Days to Eat N Oranges
 */

// @lc code=start
impl Solution {
    pub fn min_days(n: i32) -> i32 {
        use std::collections::HashMap;
        let mut memo = HashMap::new();
        fn _min_days(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
            if n <= 1 {
                return n;
            }
            if let Some(&m) = memo.get(&n) {
                return m;
            }
            let v = (n % 2 + 1 + _min_days(n / 2, memo)).min(n % 3 + 1 + _min_days(n / 3, memo));
            memo.insert(n, v);
            v
        }
        _min_days(n, &mut memo)
    }
}
// @lc code=end
