/*
 * @lc app=leetcode id=2170 lang=rust
 *
 * [2170] Minimum Operations to Make the Array Alternating
 */

// @lc code=start
impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 0;
        }
        use std::collections::HashMap;
        let mut m = vec![HashMap::new(); 2];
        for (i, &v) in nums.iter().enumerate() {
            *m[i % 2].entry(v).or_insert(0) += 1;
        }
        let mut mm = vec![vec![0; 4]; 2];
        for (i, n) in m.iter().enumerate() {
            for (&k, &v) in n {
                if v > mm[i][1] {
                    for j in 0..2 {
                        mm[i].swap(j, j + 2);
                    }
                    mm[i][0] = k;
                    mm[i][1] = v;
                } else if v > mm[i][3] {
                    mm[i][2] = k;
                    mm[i][3] = v;
                }
            }
        }
        let n = nums.len() as i32;
        n - if mm[0][0] != mm[1][0] {
            mm[0][1] + mm[1][1]
        } else {
            (mm[0][1] + mm[1][3]).max(mm[1][1] + mm[0][3])
        }
    }
}
// @lc code=end
