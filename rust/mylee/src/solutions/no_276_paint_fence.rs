// 276\. Paint Fence
// =================

// There is a fence with n posts, each post can be painted with one of the k colors.

// You have to paint all the posts such that no more than two adjacent fence posts have the same color.

// Return the total number of ways you can paint the fence.

// **Note:**
// n and k are non-negative integers.

// **Example:**

// **Input:** n = 3, k = 2
// **Output:** 6
// **Explanation:** Take c1 as color 1, c2 as color 2. All possible ways are:

//             post1  post2  post3
//  -----      -----  -----  -----
//    1         c1     c1     c2
//    2         c1     c2     c1
//    3         c1     c2     c2
//    4         c2     c1     c1
//    5         c2     c1     c2
//    6         c2     c2     c1

// ### Difficulty:

// Easy

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)
// @lc code=start
impl Solution {
    pub fn num_ways(n: i32, k: i32) -> i32 {
        // per color
        if n == 0 || k == 0 {
            return 0;
        }
        let mut prev_same = 0;
        let mut prev_diff = 1;
        for _ in 1..n {
            let diff = (k - 1) * (prev_diff + prev_same);
            let same = prev_diff;
            prev_same = same;
            prev_diff = diff;
        }
        (prev_same + prev_diff) * k
    }
}
// @lc code=end
#[allow(dead_code)]
pub  struct  Solution;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_num_ways() {
        assert_eq!(Solution::num_ways(3, 3), 24);
    }
}
