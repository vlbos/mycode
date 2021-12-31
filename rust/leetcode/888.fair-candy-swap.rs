/*
 * @lc app=leetcode id=888 lang=rust
 *
 * [888] Fair Candy Swap
 */

// @lc code=start
impl Solution {
    pub fn fair_candy_swap(alice_sizes: Vec<i32>, bob_sizes: Vec<i32>) -> Vec<i32> {
         let a = alice_sizes.iter().sum::<i32>();
        let b = bob_sizes.iter().sum::<i32>();
        let d = (a - b).abs();
        let mut aa = if a > b {
            alice_sizes.clone()
        } else {
            bob_sizes.clone()
        };
        let bb = if a <= b {
            alice_sizes.clone()
        } else {
            bob_sizes.clone()
        };
        aa.sort();
        aa.dedup();
        for i in &aa {
            if *i > d / 2 {
                let j = *i - d / 2;
                if bb.contains(&j) {
                    if a > b {
                        return vec![*i, j].to_vec();
                    } else {
                        return vec![j, *i].to_vec();
                    }
                }
            }
        }
        vec![].to_vec()
    }
}
// @lc code=end

