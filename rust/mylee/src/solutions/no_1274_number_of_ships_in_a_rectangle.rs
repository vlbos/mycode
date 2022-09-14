/*
(This problem is an interactive problem.)

On the sea represented by a cartesian plane, each ship is located at an integer point, and each integer point may contain at most 1 ship.

You have a function Sea.hasShips(topRight, bottomLeft) which takes two points as arguments and returns true if and only if there is at least one ship in the rectangle represented by the two points, including on the boundary.

Given two points, which are the top right and bottom left corners of a rectangle, return the number of ships present in that rectangle.  It is guaranteed that there are at most 10 ships in that rectangle.

Submissions making more than 400 calls to hasShips will be judged Wrong Answer.  Also, any solutions that attempt to circumvent the judge will result in disqualification.


Example :

Input:
ships = [[1,1],[2,2],[3,3],[5,5]], topRight = [4,4], bottomLeft = [0,0]
Output: 3
Explanation: From [0,0] to [4,4] we can count 3 ships within the range.


Constraints:
    On the input ships is only given to initialize the map internally. You must solve this problem "blindfolded". In other words, you must find the answer using the given hasShips API, without knowing the ships position.
    0 <= bottomLeft[0] <= topRight[0] <= 1000
    0 <= bottomLeft[1] <= topRight[1] <= 1000


*/
/**
 * // This is Sea's API interface.
 * // You should not implement it, or speculate about its implementation
 * struct Sea;
 * impl Sea {
 *    pub fn hasShips(topRight: Vec<i32>,bottomLeft: Vec<i32>)->bool{}
 * }
 */
 pub struct Sea;
 impl Sea {
    pub fn hasShips(&self,topRight: Vec<i32>,bottomLeft: Vec<i32>)->bool{true}
 }
pub struct Solution ;
impl Solution {
    pub fn count_ships(sea: &Sea, topRight: Vec<i32>, bottomLeft: Vec<i32>) -> i32 {
let x1 = topRight[0];
        let y1 = topRight[1];
        let x2 = bottomLeft[0];
        let y2 = bottomLeft[1];

        if x1 < x2 || y1 < y2 || !sea.hasShips(topRight, bottomLeft) {
            return 0
        }

        if x1 == x2 || y1 == y2 {
            return 1
        }

        let mx = (x1 + x2) / 2;
        let my = (y1 + y2) / 2;

        Self::count_ships(sea, vec![mx, my], vec![x2, y2]) + Self::count_ships(sea, vec![mx, y1], vec![x2, my + 1]) + Self::count_ships(sea, vec![x1, my], vec![mx + 1, y2]) + Self::count_ships(sea, vec![x1, y1], vec![mx + 1, my + 1])
    }
}

