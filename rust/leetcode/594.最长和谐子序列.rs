/*
 * @lc app=leetcode.cn id=594 lang=rust
 *
 * [594] 最长和谐子序列
 */

// @lc code=start
impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut m = std::collections::HashMap::<i32, i32>::new();

        for n in &nums {
            let c = m.entry(*n).or_insert(0);
            *c += 1;
        }
        let mut k = m.keys().cloned().collect::<Vec<i32>>();
        k.sort();
        let mut maxcount = 0;
        for i in 1..k.len() {
            if k[i] - k[i - 1] == 1 {
                let count = m.get(&k[i]).unwrap() + m.get(&k[i - 1]).unwrap();
                if count > maxcount {
                    maxcount = count;
                }
            }
        }
        maxcount
    }
}
// @lc code=end
