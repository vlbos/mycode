/*
 * @lc app=leetcode id=2070 lang=rust
 *
 * [2070] Most Beautiful Item for Each Query
 */

// @lc code=start
impl Solution {
    pub fn maximum_beauty(items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut items = items;
        items.sort_by_key(|x| x[0]);
        let n = items.len();
        for i in 1..n {
            items[i][1] = items[i][1].max(items[i - 1][1]);
        }
        let query = |q: i32| -> i32 {
            let (mut l, mut r) = (0, n);
            while l < r {
                let mid = (l + r) / 2;
                if items[mid][0] > q {
                    r = mid;
                } else {
                    l = mid + 1;
                }
            }
            if l == 0 {
                0
            } else {
                items[l - 1][1]
            }
        };
        let mut ans = Vec::new();
        for &q in &queries {
            ans.push(query(q));
        }
        ans
    }
}
// @lc code=end
