// 1826\. Faulty Sensor
// ====================

// An experiment is being conducted in a lab. To ensure accuracy, there are **two** sensors collecting data simultaneously. You are given 2 arrays `sensor1` and `sensor2`, where `sensor1[i]` and `sensor2[i]` are the `ith` data points collected by the two sensors.

// However, this type of sensor has a chance of being defective, which causes **exactly one** data point to be dropped. After the data is dropped, all the data points to the **right** of the dropped data are **shifted** one place to the left, and the last data point is replaced with some **random value**. It is guaranteed that this random value will **not** be equal to the dropped value.

// *   For example, if the correct data is `[1,2,**3**,4,5]` and `3` is dropped, the sensor could return `[1,2,4,5,**7**]` (the last position can be **any** value, not just `7`).

// We know that there is a defect in **at most one** of the sensors. Return _the sensor number (_`1` _or_ `2`_) with the defect. If there is **no defect** in either sensor or if it is **impossible** to determine the defective sensor, return_ `-1`_._

// **Example 1:**

// **Input:** sensor1 = \[2,3,4,5\], sensor2 = \[2,1,3,4\]
// **Output:** 1
// **Explanation:** Sensor 2 has the correct values.
// The second data point from sensor 2 is dropped, and the last value of sensor 1 is replaced by a 5.

// **Example 2:**

// **Input:** sensor1 = \[2,2,2,2,2\], sensor2 = \[2,2,2,2,5\]
// **Output:** -1
// **Explanation:** It is impossible to determine which sensor has a defect.
// Dropping the last value for either sensor could produce the output for the other sensor.

// **Example 3:**

// **Input:** sensor1 = \[2,3,2,2,3,2\], sensor2 = \[2,3,2,3,2,7\]
// **Output:** 2
// **Explanation:** Sensor 1 has the correct values.
// The fourth data point from sensor 1 is dropped, and the last value of sensor 1 is replaced by a 7.

// **Constraints:**

// *   `sensor1.length == sensor2.length`
// *   `1 <= sensor1.length <= 100`
// *   `1 <= sensor1[i], sensor2[i] <= 100`

// ### Difficulty:

// Easy

// ### Lock:

// Prime

// ### Company:

// [Facebook](https://leetcode.ca/tags/#Facebook)

use super::util::tree::TreeNode;

#[allow(dead_code)]
pub struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn check_equivalence(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        use std::collections::HashMap;
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, v: i32, freq: &mut HashMap<i32, i32>) {
            if root.is_none() {
                return;
            }
            let node = root.as_ref().unwrap().borrow();
            if (node.val as u8 as char).is_ascii_alphabetic() {
                *freq.entry(node.val).or_insert(0) += v;
            } else {
                dfs(&node.left, v, freq);
                dfs(&node.right, v, freq);
            }
        }
        let mut freq = HashMap::new();
        dfs(&root1, 1, &mut freq);
        dfs(&root2, -1, &mut freq);
        if freq.values().any(|v| *v > 0) {
            false
        } else {
            true
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    // use crate::tree;
    use super::super::util::tree::to_tree;
    fn to_exp_tree(s: &str) -> Option<Rc<RefCell<TreeNode>>> {
        to_tree(
            s.split(',')
                .map(|x| {
                    if x == "null" {
                        None
                    } else {
                        Some(x.as_bytes()[0] as i32)
                    }
                })
                .collect::<Vec<Option<i32>>>(),
        )
    }
    #[test]
    pub fn test_check_equivalence_1() {
        assert!(Solution::check_equivalence(
            to_exp_tree("x"),
            to_exp_tree("x")
        ));
    }
    #[test]
    pub fn test_check_equivalence_2() {
        assert!(Solution::check_equivalence(
            to_exp_tree("+,a,+,null,null,b,c"),
            to_exp_tree("+,+,b,c,a")
        ));
    }
    #[test]
    pub fn test_check_equivalence_3() {
        assert!(!Solution::check_equivalence(
            to_exp_tree("+,a,+,null,null,b,c"),
            to_exp_tree("+,+,b,d,a")
        ));
    }
}
