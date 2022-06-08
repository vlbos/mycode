/*
 * @lc app=leetcode id=757 lang=rust
 *
 * [757] Set Intersection Size At Least Two
 */

// @lc code=start
impl Solution {
    pub fn intersection_size_two(intervals: Vec<Vec<i32>>) -> i32 {
         let mut intervals = intervals;
        intervals.sort_by(|a, b| {
            if a[0] == b[0] {
                b[1].cmp(&a[1])
            } else {
                a[0].cmp(&b[0])
            }
        });
        let mut todo = vec![2; intervals.len()];
        let mut ans = 0;
        while !intervals.is_empty() {
            let se = intervals.pop().unwrap();
            let t = todo.pop().unwrap();
            for p in se[0]..(se[0] + t) {
                for (i, se0) in intervals.iter().enumerate() {
                    if todo[i] > 0 && p <= se0[1] {
                        todo[i] -= 1;
                    }
                }
                ans += 1;
            }
        }
        ans
    }
}
// @lc code=end
