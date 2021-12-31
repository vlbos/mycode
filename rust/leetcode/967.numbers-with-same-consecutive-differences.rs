/*
 * @lc app=leetcode id=967 lang=rust
 *
 * [967] Numbers With Same Consecutive Differences
 */

// @lc code=start
impl Solution {
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        use std::collections::HashSet;
        let mut cur = (1..10).collect::<HashSet<i32>>();
        for _ in 1..n{
            let mut cur2 = HashSet::new();
            for x in &cur{
                let d = x%10;
                if d-k>=0{
                cur2.insert(x*10+d-k);
                }
                if d+k<=9{
                cur2.insert(x*10+d+k);
                }
            }
            cur=cur2;
        }
        if n==1{
        cur.insert(0);
        }
        cur.iter().cloned().collect::<Vec<i32>>()
    }
}
// @lc code=end

