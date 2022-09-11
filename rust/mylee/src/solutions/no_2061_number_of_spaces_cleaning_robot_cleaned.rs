// # [2061. Number of Spaces Cleaning Robot Cleaned](https://leetcode.com/problems/number-of-spaces-cleaning-robot-cleaned)

// ## Description

//  A room is represented by a  0-indexed  2D binary matrix  room  where a  0  represents an  empty  space and a  1  represents a space with an  object .
//  The top left corner of the room will be empty in all test cases.

//  A cleaning robot starts at the top left corner of the room and is facing right.
// The robot will continue heading straight until it reaches the edge of the room or it hits an object,
// after which it will turn 90 degrees  clockwise  and repeat this process.
// The starting space and all spaces that the robot visits are  cleaned  by it.

//  Return  the number of  clean  spaces in the room if the robot runs indefinetely.

//   Example 1:
// <img src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2000-2099/2061.Number%20of%20Spaces%20Cleaning%20Robot%20Cleaned/images/image-20211101204703-1.png" style="width: 250px; height: 242px;" />

//  Input:  room = [[0,0,0],[1,1,0],[0,0,0]]
//  Output:  7
//  Explanation:
// The robot cleans the spaces at (0, 0), (0, 1), and (0, 2).
// The robot is at the edge of the room, so it turns 90 degrees clockwise and now faces down.
// The robot cleans the spaces at (1, 2), and (2, 2).
// The robot is at the edge of the room, so it turns 90 degrees clockwise and now faces left.
// The robot cleans the spaces at (2, 1), and (2, 0).
// The robot has cleaned all 7 empty spaces, so return 7.

//   Example 2:
// <img src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2000-2099/2061.Number%20of%20Spaces%20Cleaning%20Robot%20Cleaned/images/image-20211101204736-2.png" style="width: 250px; height: 245px;" />

//  Input:  room = [[0,1,0],[1,0,0],[0,0,0]]
//  Output:  1
//  Explanation:
// The robot cleans the space at (0, 0).
// The robot hits an object, so it turns 90 degrees clockwise and now faces down.
// The robot hits an object, so it turns 90 degrees clockwise and now faces left.
// The robot is at the edge of the room, so it turns 90 degrees clockwise and now faces up.
// The robot is at the edge of the room, so it turns 90 degrees clockwise and now faces right.
// The robot is back at its starting position.
// The robot has cleaned 1 space, so return 1.

//   Constraints:

// 	  m == room.length
// 	  n == room[r].length
// 	  1  <= m, n  <= 300
// 	  room[r][c]  is either  0  or  1 .
// 	  room[0][0] == 0

//  int number_of_clean_rooms(int[][] room) {

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn number_of_clean_rooms(room: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (room.len() as i32, room[0].len() as i32);
        let (mut i, mut j) = (0, 0);
        let mut state = 0;
        let mut seen = vec![vec![0; n as usize]; m as usize];
        seen[i as usize][j as usize] |= 1 << state;
        let mut room = room;
        room[i as usize][j as usize] = 2;
        let mut ans = 1;
        let dirs = [0, 1, 0, -1, 0];
        loop {
            let (dx, dy) = (dirs[state], dirs[state + 1]);
            let (x, y) = (i + dx, j + dy);
            if x < 0 || x == m || y < 0 || y == n || room[x as usize][y as usize] == 1 {
                state = (state + 1) % 4;
            } else {
                if room[x as usize][y as usize] == 0 {
                    ans += 1;
                    room[x as usize][y as usize] = 2;
                }
                i = x;
                j = y;
            }
            if seen[i as usize][j as usize] & (1 << state) > 0 {
                return ans;
            }
            seen[i as usize][j as usize] |= 1 << state;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_number_of_clean_rooms_1() {
        assert_eq!(
            7,
            Solution::number_of_clean_rooms(vec![vec![0, 0, 0], vec![1, 1, 0], vec![0, 0, 0]])
        );
    }
    #[test]
    pub fn test_number_of_clean_rooms_2() {
        assert_eq!(
            1,
            Solution::number_of_clean_rooms(vec![vec![0, 1, 0], vec![1, 0, 0], vec![0, 0, 0]])
        );
    }
}
