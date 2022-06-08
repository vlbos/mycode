/*
 * @lc app=leetcode id=1675 lang=rust
 *
 * [1675] Minimize Deviation in Array
 */

// @lc code=start
impl Solution {
    pub fn minimum_deviation(nums: Vec<i32>) -> i32 {
        let mut set: std::collections::BTreeSet<i32> = nums
            .into_iter()
            .map(|x| if x % 2 > 0 { x * 2 } else { x })
            .collect();
        let mut ans = *set.iter().last().unwrap() - *set.iter().nth(0).unwrap();
        while ans > 0 && *set.iter().last().unwrap() % 2 == 0 {
            let max = *set.iter().last().unwrap();
            set.remove(&max);
            set.insert(max / 2);
            ans = ans.min(*set.iter().last().unwrap() - *set.iter().nth(0).unwrap());
        }
        ans
    }
}
// @lc code=end
