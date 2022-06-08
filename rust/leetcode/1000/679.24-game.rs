/*
 * @lc app=leetcode id=679 lang=rust
 *
 * [679] 24 Game
 */

// @lc code=start
impl Solution {
    pub fn judge_point24(cards: Vec<i32>) -> bool {
       fn solve(nums: &Vec<f32>) -> bool {
            if nums.is_empty() {
                return false;
            }
            if nums.len() == 1 {
                return (nums[0] - 24.0).abs() < 0.000006;
            }
            for (i, &x) in nums.iter().enumerate() {
                for (j, &y) in nums.iter().enumerate() {
                    if i == j {
                        continue;
                    }
                    let mut new_nums = Vec::new();
                    for (k, &z) in nums.iter().enumerate() {
                        if k == i || k == j {
                            continue;
                        }
                        new_nums.push(z);
                    }
                    for k in 0..4 {
                        if k < 2 && i > j {
                            continue;
                        }
                        match k {
                            0 => new_nums.push(x + y),
                            1 => new_nums.push(x * y),
                            2 => new_nums.push(x - y),
                            _ => {
                                if y.abs() < 0.000001 {
                                    continue;
                                }
                                new_nums.push(x / y);
                            }
                        }
                        if solve(&new_nums) {
                            return true;
                        }
                        new_nums.pop();
                    }
                }

            }
            false
        }
        let nums = cards.iter().map(|x|*x as f32).collect();
        solve(&nums)
    }
}
// @lc code=end
