/*
 * @lc app=leetcode id=1560 lang=rust
 *
 * [1560] Most Visited Sector in  a Circular Track
 */

// @lc code=start
impl Solution {
    pub fn most_visited(n: i32, rounds: Vec<i32>) -> Vec<i32> {
        let n1 = n  + 1;
        let mut v = vec![0; n1 as usize];
        v[rounds[0] as usize] += 1;
        for i in 0..rounds.len() - 1 {
            let mut j = if rounds[i] + 1==n1 {1} else {rounds[i] + 1};
            let e = if rounds[i+1] + 1==n1 {1} else {rounds[i+1] + 1};
            while j != e {
                v[j as usize] += 1;
                j = if j + 1==n1 {1} else {j + 1};
            }
        }
        let mut ans = Vec::new();
        let mut m = 0;
        for (i, vi) in v.iter().enumerate() {
            if *vi > m {
                m = *vi;
                ans.clear();
                ans.push(i as i32);
            } else if *vi == m {
                ans.push(i as i32);
            }
        }
        ans
    }
}
// @lc code=end
