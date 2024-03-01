// 1580\. Put Boxes Into the Warehouse II
// ======================================

// You are given two arrays of positive integers, `boxes` and `warehouse`,
// representing the heights of some boxes of unit width and the heights of `n` rooms in a warehouse respectively.
// The warehouse's rooms are labeled from `0` to `n - 1` from left to right where `warehouse[i]` (0-indexed) is the height of the `ith` room.

// Boxes are put into the warehouse by the following rules:

// *   Boxes cannot be stacked.
// *   You can rearrange the insertion order of the boxes.
// *   Boxes can be pushed into the warehouse from **either side** (left or right)
// *   If the height of some room in the warehouse is less than the height of a box, then that box and all other boxes behind it will be stopped before that room.

// Return _the maximum number of boxes you can put into the warehouse._

// **Example 1:**

// ![](https://assets.leetcode.com/uploads/2020/08/30/22.png)

// **Input:** boxes = \[1,2,2,3,4\], warehouse = \[3,4,1,2\]
// **Output:** 4
// **Explanation:
// ![](https://assets.leetcode.com/uploads/2020/08/30/22-1.png)** We can store the boxes in the following order:
// 1- Put the yellow box in room 2 from either the left or right side.
// 2- Put the orange box in room 3 from the right side.
// 3- Put the green box in room 1 from the left side.
// 4- Put the red box in room 0 from the left side.
// Notice that there are other valid ways to put 4 boxes such as swapping the red and green boxes or the red and orange boxes.

// **Example 2:**

// ![](https://assets.leetcode.com/uploads/2020/08/30/22-2.png)

// **Input:** boxes = \[3,5,5,2\], warehouse = \[2,1,3,4,5\]
// **Output:** 3
// **Explanation:
// ![](https://assets.leetcode.com/uploads/2020/08/30/22-3.png)** It's not possible to put the two boxes of height 5 in the warehouse since there's only 1 room of height >= 5.
// Other valid solutions are to put the green box in room 2 or to put the orange box first in room 2 before putting the green and red boxes.

// **Example 3:**

// **Input:** boxes = \[1,2,3\], warehouse = \[1,2,3,4\]
// **Output:** 3

// **Example 4:**

// **Input:** boxes = \[4,5,6\], warehouse = \[3,3,3,3,3\]
// **Output:** 0

// **Constraints:**

// *   `n == warehouse.length`
// *   `1 <= boxes.length, warehouse.length <= 105`
// *   `1 <= boxes[i], warehouse[i] <= 109`

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Unknown](https://leetcode.ca/tags/#Unknown)

#[allow(dead_code)]
pub struct Solution;
impl Solution {
    pub fn max_boxes_in_warehouse(boxes: Vec<i32>, warehouse: Vec<i32>) -> i32 {
        let n = warehouse.len();
        let mut boxes = boxes;
        boxes.sort();
        let mut ans = 0;
        let (mut l, mut r) = (0, n - 1);
        for &b in boxes.iter().rev() {
            if l > r {
                break;
            }
            if b <= warehouse[l] {
                ans += 1;
                l += 1;
            } else if b <= warehouse[r] {
                ans += 1;
                r -= 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_max_boxes_in_warehouse_1() {
        assert_eq!(
            4,
            Solution::max_boxes_in_warehouse(vec![1, 2, 2, 3, 4], vec![3, 4, 1, 2])
        );
    }
    #[test]
    pub fn test_max_boxes_in_warehouse_2() {
        assert_eq!(
            3,
            Solution::max_boxes_in_warehouse(vec![3, 5, 5, 2], vec![2, 1, 3, 4, 5])
        );
    }
    #[test]
    pub fn test_max_boxes_in_warehouse_3() {
        assert_eq!(
            3,
            Solution::max_boxes_in_warehouse(vec![1, 2, 3], vec![1, 2, 3, 4])
        );
    }

    #[test]
    pub fn test_max_boxes_in_warehouse_4() {
        assert_eq!(
            0,
            Solution::max_boxes_in_warehouse(vec![4, 5, 6], vec![3, 3, 3, 3, 3])
        );
    }
}
