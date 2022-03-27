/*
 * @lc app=leetcode.cn id=414 lang=rust
 *
 * [414] 第三大的数
 */

// @lc code=start
impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut r = Vec::<i32>::new();
        for n in &nums {
            if r.contains(n) {
                continue;
            }
            if r.is_empty() {
                r.push(*n);
                continue;
            }
            for i in 0..r.len() {
                let j = r.len() - i - 1;
                if *n > r[j] {
                    if r.len() < 3 {
                        r.push(r[j]);
                    } else if i > 0 {
                        r[j + 1] = r[j];
                    }

                    r[j] = *n;
                } else if r.len() < 3 {
                    r.push(*n);
                }
            }
        }
        // println!("{:?}",r);
        if r.len() < 3 {
            r[0]
        } else {
            r[2]
        }
    }
}
// @lc code=end
