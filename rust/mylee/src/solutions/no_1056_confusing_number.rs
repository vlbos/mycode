// 1056\. Confusing Number
// =======================

// Given a number `N`, return `true` if and only if it is a _confusing number_, which satisfies the following condition:

// We can rotate digits by 180 degrees to form new digits. When 0, 1, 6, 8, 9 are rotated 180 degrees, they become 0, 1, 9, 8, 6 respectively.
// When 2, 3, 4, 5 and 7 are rotated 180 degrees, they become invalid. A _confusing number_ is a number that when rotated 180 degrees becomes a **different** number with each digit valid.

// **Example 1:**

// ![](https://assets.leetcode.com/uploads/2019/03/23/1268_1.png)

// **Input:** 6
// **Output:** true
// **Explanation:**
// We get `9` after rotating `6`, `9` is a valid number and `9!=6`.

// **Example 2:**

// ![](https://assets.leetcode.com/uploads/2019/03/23/1268_2.png)

// **Input:** 89
// **Output:** true
// **Explanation:**
// We get `68` after rotating `89`, `86` is a valid number and `86!=89`.

// **Example 3:**

// ![](https://assets.leetcode.com/uploads/2019/03/26/1268_3.png)

// **Input:** 11
// **Output:** false
// **Explanation:**
// We get `11` after rotating `11`, `11` is a valid number but the value remains the same, thus `11` is not a confusing number.

// **Example 4:**

// ![](https://assets.leetcode.com/uploads/2019/03/23/1268_4.png)

// **Input:** 25
// **Output:** false
// **Explanation:**
// We get an invalid number after rotating `25`.

// **Note:**

// 1.  `0 <= N <= 10^9`
// 2.  After the rotation we can ignore leading zeros, for example if after rotation we have `0008` then this number is considered as just `8`.

// ### Difficulty:

// Easy

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)
 
#[allow(dead_code)] 
 pub struct Solution {}
impl Solution {
    pub fn confusing_number(n: i32) -> bool {
        let ns = n.to_string();
        ns.chars()
            .filter(|x| "01869".chars().all(|c| c != *x))
            .count()
            == 0
            && ns.chars().filter(|x| "69".chars().any(|c| c == *x)).count() > 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_confusing_number_1() {
        assert!(Solution::confusing_number(6));
    }
    #[test]
    fn test_confusing_number_2() {
        assert!(Solution::confusing_number(89));
    }
    #[test]
    fn test_confusing_number_3() {
        assert!(!Solution::confusing_number(11));
    }
    #[test]
    fn test_confusing_number_4() {
        assert!(!Solution::confusing_number(25));
    }
}
