/*
 * @lc app=leetcode id=165 lang=rust
 *
 * [165] Compare Version Numbers
 */

// @lc code=start
impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let v1 = version1
            .split(".")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let v2 = version2
            .split(".")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let l = v1.len().min(v2.len());
        for i in 0..l {
            if v1[i] < v2[i] {
                return -1;
            } else if v1[i] > v2[i] {
                return 1;
            }
        }
        if v1.len() > v2.len() {
            for i in v2.len()..v1.len() {
                if v1[i] > 0 {
                    return 1;
                }
            }
        } else if v1.len() < v2.len() {
            for i in v1.len()..v2.len() {
                if v2[i] > 0 {
                    return -1;
                }
            }
        }
        0
    }
}
// @lc code=end
