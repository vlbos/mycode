// # [2728. Count Houses in a Circular Street](https://leetcode.com/problems/count-houses-in-a-circular-street)

// ## Description

//  You are given an object  street  of class  Street  that represents a circular street and a positive integer  k
// which represents a maximum bound for the number of houses in that street
// (in other words, the number of houses is less than or equal to  k ). Houses'doors could be open or closed initially.

//  Initially, you are standing in front of a door to a house on this street.
// Your task is to count the number of houses in the street.

//  The class  Street  contains the following functions which may help you:

// 	  void openDoor() : Open the door of the house you are in front of.
// 	  void closeDoor() : Close the door of the house you are in front of.
// 	  boolean isDoorOpen() : Returns  true  if the door of the current house is open and  false  otherwise.
// 	  void moveRight() : Move to the right house.
// 	  void moveLeft() : Move to the left house.

//  Return  ans   which represents the number of houses on this street.

//   ### Example 1:

//  Input:  street = [0,0,0,0], k = 10
//  Output:  4
//  Explanation:  There are 4 houses, and all their doors are closed.
// The number of houses is less than k, which is 10.

//   ### Example 2:

//  Input:  street = [1,0,1,1,0], k = 5
//  Output:  5
//  Explanation:  There are 5 houses, and the doors of the 1st, 3rd, and 4th house
// (moving in the right direction) are open, and the rest are closed.
// The number of houses is equal to k, which is 5.

//   Constraints:

// 	  n == number of houses
// 	  1  <= n  <= k  <= 10^3

// ## Solutions

// ### **C++**

// ```cpp
// /**
//  * Definition for a street.
//  * class Street {
//  * public:
//  *     Street(vector  doors);
//  *     void openDoor();
//  *     void closeDoor();
//  *     bool isDoorOpen();
//  *     void moveRight();
//  *     void moveLeft();
//  * };
//  */
// class Solution {
// public:
//     int house_count(Street* street, int k) {
#[allow(dead_code)]
pub struct Street {
    doors: Vec<i32>,
    index: usize,
}
impl Street {
    pub fn new(doors: Vec<i32>) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..doors.len());
        Self { doors, index }
    }
    fn open_door(&mut self) {
        self.doors[self.index] = 1;
    }
    fn close_door(&mut self) {
        self.doors[self.index] = 0;
    }
    fn is_door_open(&self) -> bool {
        self.doors[self.index] == 1
    }
    pub fn move_right(&mut self) {
        self.index = (self.index + 1) % self.doors.len();
    }
    fn move_left(&mut self) {
        self.index = if self.index == 0 {
            self.doors.len() - 1
        } else {
            self.index - 1
        };
    }
}

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn house_count(mut street: Option<Box<Street>>, k: i32) -> i32 {
        for _ in 0..k {
            street.as_mut().unwrap().open_door();
            street.as_mut().unwrap().move_left();
        }
        let mut ans = 0;
        while street.as_ref().unwrap().is_door_open() {
            street.as_mut().unwrap().close_door();
            street.as_mut().unwrap().move_left();
            ans += 1;
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_house_count_1() {
        let street = Some(Box::new(Street::new(vec![0, 0, 0, 0])));
        assert_eq!(4, Solution::house_count(street, 10));
    }
    #[test]
    pub fn test_house_count_2() {
        let street = Some(Box::new(Street::new(vec![1, 0, 1, 1, 0])));
        assert_eq!(5, Solution::house_count(street, 5));
    }
}
