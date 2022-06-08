/*
 * @lc app=leetcode id=1942 lang=rust
 *
 * [1942] The Number of the Smallest Unoccupied Chair
 */

// @lc code=start
impl Solution {
    pub fn smallest_chair(times: Vec<Vec<i32>>, target_friend: i32) -> i32 {
        let n = times.len();
        let mut arrival: Vec<(i32, usize)> =
            times.iter().enumerate().map(|(i, x)| (x[0], i)).collect();
        let mut leaving: Vec<(i32, usize)> =
            times.iter().enumerate().map(|(i, x)| (x[1], i)).collect();
        arrival.sort();
        leaving.sort();
        let mut available = (0..n)
            .map(|x| -(x as i32))
            .collect::<std::collections::BinaryHeap<i32>>();
        let mut seats = std::collections::HashMap::new();
        let mut j = 0;
        for &(time, person) in &arrival {
            while j < n && leaving[j].0 <= time {
                available.push(-*seats.get(&leaving[j].1).unwrap_or(&0));
                j += 1;
            }
            if let Some(seat) = available.pop() {
                seats.insert(person, -seat);
                if person as i32== target_friend {
                    return -seat;
                }
            }
        }
        -1
    }
}
// @lc code=end
