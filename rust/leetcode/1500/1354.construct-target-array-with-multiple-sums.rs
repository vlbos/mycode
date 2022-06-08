/*
 * @lc app=leetcode id=1354 lang=rust
 *
 * [1354] Construct Target Array With Multiple Sums
 */

// @lc code=start
impl Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        if target.len() == 1 {
            return target[0] == 1;
        }
        let mut sum = target.iter().map(|x|*x as i64).sum::<i64>();
        let mut q: std::collections::BinaryHeap<i32> = target.into_iter().collect();
        while let Some(x) = q.pop() {
            let mut x = x as i64;
            if x == 1 {
                break;
            }
            if x * 2 - sum < 1 {
                return false;
            }
            let left = sum - x;
            let y = *q.peek().unwrap() as i64;
            let mut k = 0;
            if y == 1 {
                k = (x - y + left - 1) / left;
            } else {
                k = (x - y) / left + 1;
            }
            x -= k * left;
            if x <= 0 {
                return false;
            }
            sum -= k * left;
            q.push(x as i32);
        }
        true   
    }
}
// @lc code=end
