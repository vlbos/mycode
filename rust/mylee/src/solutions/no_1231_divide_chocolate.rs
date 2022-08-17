// 1231\. Divide Chocolate
// =======================

// You have one chocolate bar that consists of some chunks. Each chunk has its own sweetness given by the array `sweetness`.

// You want to share the chocolate with your `K` friends so you start cutting the chocolate bar into `K+1` pieces using `K` cuts, each piece consists of some **consecutive** chunks.

// Being generous, you will eat the piece with the **minimum total sweetness** and give the other pieces to your friends.

// Find the **maximum total sweetness** of the piece you can get by cutting the chocolate bar optimally.

// **Example 1:**

// **Input:** sweetness = \[1,2,3,4,5,6,7,8,9\], K = 5
// **Output:** 6
// **Explanation:** You can divide the chocolate to \[1,2,3\], \[4,5\], \[6\], \[7\], \[8\], \[9\]

// **Example 2:**

// **Input:** sweetness = \[5,6,7,8,9,1,2,3,4\], K = 8
// **Output:** 1
// **Explanation:** There is only one way to cut the bar into 9 pieces.

// **Example 3:**

// **Input:** sweetness = \[1,2,2,1,2,2,1,2,2\], K = 2
// **Output:** 5
// **Explanation:** You can divide the chocolate to \[1,2,2\], \[1,2,2\], \[1,2,2\]

// **Constraints:**

// *   `0 <= K < sweetness.length <= 10^4`
// *   `1 <= sweetness[i] <= 10^5`

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn maximize_sweetness(sweetness: Vec<i32>, k: i32) -> i32 {
        let (mut l, mut r) = (
            *sweetness.iter().min().unwrap(),
            sweetness.iter().sum::<i32>(),
        );
        let mut pre = 0;
        while l < r {
            let mid = (l + r + 1) / 2;
            let (mut sum, mut cnt) = (0, 0);
            for &s in &sweetness {
                sum += s;
                if sum >= mid {
                    sum = 0;
                    cnt += 1;
                }
            }

            if cnt > k {
                l = mid;
            } else {
                r = mid - 1;
            }
            if pre == mid {
                break;
            }
            pre = mid;
        }
        l
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_maximize_sweetness_1() {
        assert_eq!(
            6,
            Solution::maximize_sweetness(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 5)
        );
    }
    #[test]
    pub fn test_maximize_sweetness_2() {
        assert_eq!(
            1,
            Solution::maximize_sweetness(vec![5, 6, 7, 8, 9, 1, 2, 3, 4], 8)
        );
    }
    #[test]
    pub fn test_maximize_sweetness_3() {
        assert_eq!(
            5,
            Solution::maximize_sweetness(vec![1, 2, 2, 1, 2, 2, 1, 2, 2], 2)
        );
    }
}
