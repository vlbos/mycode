// 360\. Sort Transformed Array
// ============================

// Given a **sorted** array of integers _nums_ and integer values _a_, _b_ and _c_.
// Apply a quadratic function of the form f(_x_) = _ax_2 + _bx_ + _c_ to each element _x_ in the array.

// The returned array must be in **sorted order**.

// Expected time complexity: **O(_n_)**

// **Example 1:**

// **Input:** nums = \[-4,-2,2,4\], a = 1, b = 3, c = 5
// **Output:** \[3,9,15,33\]

// **Example 2:**

// **Input:** nums = \[-4,-2,2,4\], a = \-1, b = 3, c = 5
// **Output:** \[-23,-5,1,7\]

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Facebook](https://leetcode.ca/tags/#Facebook) [Google](https://leetcode.ca/tags/#Google)
// @lc code=start
impl Solution {
    pub fn sort_transformed_array(nums: Vec<i32>, a: i32, b: i32, c: i32) -> Vec<i32> {
        let mut ans: Vec<i32> = nums.into_iter().map(|x| a * x * x + b * x + c).collect();
        ans.sort();
        ans
        // if a == 0 {
        //     let med = nums.into_iter().map(|n| Solution::cal(n, a, b, c));
        //     if b >= 0 {
        //         med.collect()
        //     } else {
        //         med.rev().collect()
        //     }
        // } else {
        //     let len = nums.len();
        //     if len <= 0 {
        //         return vec![];
        //     }
        //     let mut sort_indcies: Vec<usize> = vec![];
        //     let mut left = 0isize;
        //     let mut right = (len - 1) as isize;
        //     while left <= right {
        //         if i32::abs(2 * a * nums[left as usize] + b)
        //             >= i32::abs(2 * a * nums[right as usize] + b)
        //         {
        //             sort_indcies.push(left as usize);
        //             left += 1;
        //         } else {
        //             sort_indcies.push(right as usize);
        //             right -= 1;
        //         }
        //     }
        //     let increase = a >= 0;
        //     let med = sort_indcies
        //         .into_iter()
        //         .map(|i| Solution::cal(nums[i], a, b, c));
        //     if increase {
        //         med.rev().collect()
        //     } else {
        //         med.collect()
        //     }
        // }
    }

    // fn cal(num: i32, a: i32, b: i32, c: i32) -> i32 {
    //     a * num * num + b * num + c
    // }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sort_transformed_array_1() {
        assert_eq!(
            Solution::sort_transformed_array(vec![-4, -2, 2, 4], 1, 3, 5),
            vec![3, 9, 15, 33]
        );
    }

    #[test]
    fn test_sort_transformed_array_2() {
        assert_eq!(
            Solution::sort_transformed_array(vec![-4, -2, 2, 4], -1, 3, 5),
            vec![-23, -5, 1, 7]
        );
    }

    #[test]
    fn test_sort_transformed_array_3() {
        assert_eq!(
            Solution::sort_transformed_array(vec![-4, -2, 2, 4], 0, 1, 0),
            vec![-4, -2, 2, 4]
        );
    }
}
