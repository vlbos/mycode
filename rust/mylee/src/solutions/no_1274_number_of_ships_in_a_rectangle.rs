/*
(This problem is an interactive problem.)

On the sea represented by a cartesian plane, each ship is located at an integer point, and each integer point may contain at most 1 ship.

You have a function Sea.has_ships(top_right, bottom_left) which takes two points as arguments and returns true if and only if there is at least one ship in the rectangle represented by the two points, including on the boundary.

Given two points, which are the top right and bottom left corners of a rectangle, return the number of ships present in that rectangle.  It is guaranteed that there are at most 10 ships in that rectangle.

Submissions making more than 400 calls to has_ships will be judged Wrong Answer.  Also, any solutions that attempt to circumvent the judge will result in disqualification.


Example :

Input:
ships = [[1,1],[2,2],[3,3],[5,5]], top_right = [4,4], bottom_left = [0,0]
Output: 3
Explanation: From [0,0] to [4,4] we can count 3 ships within the range.


Constraints:
    On the input ships is only given to initialize the map internally. You must solve this problem "blindfolded". In other words, you must find the answer using the given has_ships API, without knowing the ships position.
    0 <= bottom_left[0] <= top_right[0] <= 1000
    0 <= bottom_left[1] <= top_right[1] <= 1000


*/
/**
 * // This is Sea's API interface.
 * // You should not implement it, or speculate about its implementation
 * struct Sea;
 * impl Sea {
 *    pub fn has_ships(top_right: Vec<i32>,bottom_left: Vec<i32>)->bool{}
 * }
 */
pub struct Sea;
impl Sea {
    pub fn has_ships(&self, _top_right: Vec<i32>, _bottom_left: Vec<i32>) -> bool {
        true
    }
}
pub struct Solution;
impl Solution {
    pub fn count_ships(sea: &Sea, top_right: Vec<i32>, bottom_left: Vec<i32>) -> i32 {
        let x1 = top_right[0];
        let y1 = top_right[1];
        let x2 = bottom_left[0];
        let y2 = bottom_left[1];

        if x1 < x2 || y1 < y2 || !sea.has_ships(top_right, bottom_left) {
            return 0;
        }

        if x1 == x2 || y1 == y2 {
            return 1;
        }

        let mx = (x1 + x2) / 2;
        let my = (y1 + y2) / 2;

        Self::count_ships(sea, vec![mx, my], vec![x2, y2])
            + Self::count_ships(sea, vec![mx, y1], vec![x2, my + 1])
            + Self::count_ships(sea, vec![x1, my], vec![mx + 1, y2])
            + Self::count_ships(sea, vec![x1, y1], vec![mx + 1, my + 1])
    }
}
