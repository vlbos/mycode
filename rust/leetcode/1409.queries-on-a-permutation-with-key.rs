/*
 * @lc app=leetcode id=1409 lang=rust
 *
 * [1409] Queries on a Permutation With Key
 */

// @lc code=start
impl Solution {
    pub fn process_queries(queries: Vec<i32>, m: i32) -> Vec<i32> {
        let mut v :Vec<i32>= (1..=m).collect();
        let mut ans = Vec::new();
        for &q in &queries{
            let p = v.iter().position(|x|*x==q);
            if let Some(i) = p{
                ans.push(i as i32);
                let iv = v.remove(i);
                v.insert(0,iv);
            }
        }
        ans
    }
}
// @lc code=end

