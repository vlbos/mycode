/*
 * @lc app=leetcode id=825 lang=rust
 *
 * [825] Friends Of Appropriate Ages
 */

// @lc code=start
impl Solution {
    pub fn num_friend_requests(ages: Vec<i32>) -> i32 {
        let ac = ages.iter().fold(std::collections::HashMap::new(), |mut m, a| {
            *m.entry(*a).or_insert(0) += 1;
            m
        });
        let mut ans = 0;
        for i in 1..=120 {
            let ca = ac.get(&i).unwrap_or(&0);
            for j in 1..=120 {
                let cb = ac.get(&j).unwrap_or(&0);
                if i as f64 * 0.5 + 7.0 >= j as f64 || i < j || i < 100 && j > 100 {
                    continue;
                }
                ans += ca * cb;
                if i == j {
                    ans -= ca;
                }
            }
        }
        ans
    }
}
// @lc code=end
