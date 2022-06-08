/*
 * @lc app=leetcode id=1893 lang=rust
 *
 * [1893] Check if All the Integers in a Range Are Covered
 */

// @lc code=start
impl Solution {
    pub fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
        let mut f = vec![0;51];
        for r in &ranges{
            for i in r[0]..=r[1]{
                f[i as usize]=1;
            }
        }
        for i in left..=right{
            if f[i as usize]==0{
            return false;
            }
        }
        true
    }
}
// @lc code=end

