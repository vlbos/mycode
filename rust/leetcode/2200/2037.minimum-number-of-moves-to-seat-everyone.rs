/*
 * @lc app=leetcode id=2037 lang=rust
 *
 * [2037] Minimum Number of Moves to Seat Everyone
 */

// @lc code=start
impl Solution {
    pub fn min_moves_to_seat(seats: Vec<i32>, students: Vec<i32>) -> i32 {
        let mut seats = seats;
        let mut students = students;
        seats.sort();
        students.sort();
        students
            .into_iter()
            .enumerate()
            .map(|(i, v)| (seats[i] - v).abs())
            .sum::<i32>()
    }
}
// @lc code=end
