// 370\. Range Addition
// ====================

// Assume you have an array of length **_n_** initialized with all **0**'s and are given **_k_** update operations.

// Each operation is represented as a triplet:
// **\[startIndex, endIndex, inc\]** which increments each element of subarray **A\[startIndex ... endIndex\]** (startIndex and endIndex inclusive) with **inc**.

// Return the modified array after all **_k_** operations were executed.

// **Example:**

// **Input:** length = 5, updates = \[\[1,3,2\],\[2,4,3\],\[0,2,-2\]\]
// **Output:** \[-2,0,3,5,3\]

// **Explanation:**

// Initial state:
// \[0,0,0,0,0\]

// After applying operation \[1,3,2\]:
// \[0,2,2,2,0\]

// After applying operation \[2,4,3\]:
// \[0,2,5,5,3\]

// After applying operation \[0,2,-2\]:
// \[-2,0,3,5,3\]

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)

// @lc code=start
impl Solution {
    pub fn get_modified_array(length: i32, updates: Vec<Vec<i32>>) -> Vec<i32> {
        // let mut res = vec![0; length as usize];
        // for u in updates {
        //     res[u[0] as usize] += u[2];
        //     if res.len() > u[1] as usize + 1 {
        //         res[u[1] as usize + 1] += 0 - u[2];
        //     }
        // }
        // res.into_iter()
        //     .scan(0, |acc, curr| {
        //         *acc += curr;
        //         Some(*acc)
        //     })
        //     .collect()
        let mut ans = vec![0; length as usize + 1];
        for u in updates {
            let (a, b) = (u[0] as usize, u[1] as usize + 1);
            ans[a] += u[2];
            ans[b] -= u[2];
        }
        ans.pop();
        ans.into_iter()
            .scan(0, |acc, x| {
                *acc += x;
                Some(*acc)
            })
            .collect()
    }
}
// @lc code=end

#[allow(dead_code)]
pub  struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_range_addition() {
        assert_eq!(
            Solution::get_modified_array(5, vec![vec![1, 3, 2], vec![2, 4, 3], vec![0, 2, -2]]),
            vec![-2, 0, 3, 5, 3]
        );
    }
}
