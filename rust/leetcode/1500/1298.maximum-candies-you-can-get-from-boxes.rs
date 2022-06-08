/*
 * @lc app=leetcode id=1298 lang=rust
 *
 * [1298] Maximum Candies You Can Get from Boxes
 */

// @lc code=start
impl Solution {
    pub fn max_candies(
        status: Vec<i32>,
        candies: Vec<i32>,
        keys: Vec<Vec<i32>>,
        contained_boxes: Vec<Vec<i32>>,
        initial_boxes: Vec<i32>,
    ) -> i32 {
       let n = status.len();
let mut can_open: Vec<bool> = status.into_iter().map(|x| x == 1).collect();
        let mut has_box = vec![false; n];
        let mut used = vec![false; n];
        let mut ans = 0;
        let mut q = std::collections::VecDeque::new();

        for &i in &initial_boxes {
            let i = i as usize;
            has_box[i] = true;
            if can_open[i] {
                q.push_back(i);
                used[i] = true;
                ans += candies[i];
            }
        }
        while let Some(i) = q.pop_front() {
            for &key in &keys[i] {
                let j = key as usize;
                can_open[j] = true;
                if !used[j] && has_box[j] {
                    q.push_back(j);
                    used[j] = true;
                    ans += candies[j];
                }
            }
            for &j in &contained_boxes[i] {
                let j = j as usize;
                has_box[j] = true;
                if !used[j] && can_open[j] {
                    q.push_back(j);
                    used[j] = true;
                    ans += candies[j];
                }
            }
        }
        ans
    }
}
// @lc code=end
