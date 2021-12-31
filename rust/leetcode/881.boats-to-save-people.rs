/*
 * @lc app=leetcode id=881 lang=rust
 *
 * [881] Boats to Save People
 */

// @lc code=start
impl Solution {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        let mut people = people;
        people.sort();
        let mut light = 0;
        let mut heavy = people.len() - 1;
        let mut ans = 0;
        while light <= heavy {
            if people[light] + people[heavy] > limit {
                heavy -= 1;
            } else {
                light += 1;
                heavy -= 1;
            }
            ans += 1;
            if heavy == usize::MAX {
                    break;
            }
        }
        ans
    }
}
// @lc code=end
