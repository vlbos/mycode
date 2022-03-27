/*
 * @lc app=leetcode id=1488 lang=rust
 *
 * [1488] Avoid Flood in The City
 */

// @lc code=start
impl Solution {
    pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![1; rains.len()];
        let mut water = std::collections::HashMap::new();
        let mut zero = std::collections::BTreeSet::new();
        for (i, r) in rains.iter().enumerate() {
            if *r == 0 {
                zero.insert(i);
                continue;
            }
            if let Some(&j) = water.get(r) {
                let mut jday = zero.range(j..);
                let mut rj=-1;
                if let Some(jj) = jday.next() {
                    rj =(*jj) as i32;
                    ans[rj as usize] = *r ;
                } else {
                    return Vec::new();
                }
                if  rj!=-1{
                    zero.remove(&(rj as usize));
                }
            }
            water.insert(r, i);
            ans[i]=-1;
        }
        ans
    }
}
// @lc code=end
