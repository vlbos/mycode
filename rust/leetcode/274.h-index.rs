/*
 * @lc app=leetcode id=274 lang=rust
 *
 * [274] H-Index
 */

// @lc code=start
impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut citations=citations;
        citations.sort_by(|a,b|b.cmp(a));
        let mut ans = 0;
        let mut cnt = 0;
        for c in citations{
            cnt+=1;
            if cnt<=c{
                ans=cnt;
            }else{
                break;
            }
        }
        ans
    }
}
// @lc code=end

