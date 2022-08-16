// 1229\. Meeting Scheduler
// ========================

// Given the availability time slots arrays `slots1` and `slots2` of two people and a meeting duration `duration`, return the **earliest time slot** that works for both of them and is of duration `duration`.

// If there is no common time slot that satisfies the requirements, return an **empty array**.

// The format of a time slot is an array of two elements `[start, end]` representing an inclusive time range from `start` to `end`.

// It is guaranteed that no two availability slots of the same person intersect with each other. That is, for any two time slots `[start1, end1]` and `[start2, end2]` of the same person, either `start1 > end2` or `start2 > end1`.

// **Example 1:**

// **Input:** slots1 = \[\[10,50\],\[60,120\],\[140,210\]\], slots2 = \[\[0,15\],\[60,70\]\], duration = 8
// **Output:** \[60,68\]

// **Example 2:**

// **Input:** slots1 = \[\[10,50\],\[60,120\],\[140,210\]\], slots2 = \[\[0,15\],\[60,70\]\], duration = 12
// **Output:** \[\]

// **Constraints:**

// *   `1 <= slots1.length, slots2.length <= 10^4`
// *   `slots1[i].length, slots2[i].length == 2`
// *   `slots1[i][0] < slots1[i][1]`
// *   `slots2[i][0] < slots2[i][1]`
// *   `0 <= slots1[i][j], slots2[i][j] <= 10^9`
// *   `1 <= duration <= 10^6`

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [DoorDash](https://leetcode.ca/tags/#DoorDash) [Paypal](https://leetcode.ca/tags/#Paypal) [pramp](https://leetcode.ca/tags/#pramp) [Uber](https://leetcode.ca/tags/#Uber)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn min_available_duration(
        slots1: Vec<Vec<i32>>,
        slots2: Vec<Vec<i32>>,
        duration: i32,
    ) -> Vec<i32> {
        let (mut i, mut j) = (0, 0);
        while i < slots1.len() && j < slots2.len() {
            while i < slots1.len() && slots1[i][1] < slots2[j][0] {
                i += 1;
            }
            if i == slots1.len() {
                break;
            }
            while j < slots2.len() && slots2[j][1] < slots1[i][0] {
                j += 1;
            }
            if j == slots2.len() {
                break;
            }
            let start = slots1[i][0].max(slots2[j][0]);
            let end = start + duration;
            if end <= slots1[i][1].min(slots2[j][1]) {
                return vec![start, end];
            }
        }

        Vec::new()
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_min_available_duration_1() {
        assert_eq!(
            vec![60, 68],
            Solution::min_available_duration(
                vec![vec![10, 50], vec![60, 120], vec![140, 210]],
                vec![vec![0, 15], vec![60, 70]],
                8
            )
        );
    }
    #[test]
    pub fn test_min_available_duration_2() {
        assert_eq!(
            Vec::<i32>::new(),
            Solution::min_available_duration(
                vec![vec![10, 50], vec![60, 120], vec![140, 210]],
                vec![vec![0, 15], vec![60, 70]],
                12
            )
        );
    }
}
