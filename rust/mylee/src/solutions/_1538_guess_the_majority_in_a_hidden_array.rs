/*
We have an integer array nums, where all the integers in nums are 0 or 1. You will not be given direct access to the array, instead, you will have an API ArrayReader which have the following functions:

    int query(int a, int b, int c, int d): where 0 <= a < b < c < d < ArrayReader.length(). The function returns the distribution of the value of the 4 elements and returns:


        4 : if the values of the 4 elements are the same (0 or 1).
        2 : if three elements have a value equal to 0 and one element has value equal to 1 or vice versa.
        0 : if two element have a value equal to 0 and two elements have a value equal to 1.


    int length(): Returns the size of the array.

You are allowed to call query() 2 * n times at most where n is equal to ArrayReader.length().

Return any index of the most frequent value in nums, in case of tie, return -1.

Follow up: What is the minimum number of calls needed to find the majority element?


Example 1:
Input: nums = [0,0,1,0,1,1,1,1]
Output: 5
Explanation: The following calls to the API
reader.length() // returns 8 because there are 8 elements in the hidden array.
reader.query(0,1,2,3) // returns 2 this is a query that compares the elements nums[0], nums[1], nums[2], nums[3]
// Three elements have a value equal to 0 and one element has value equal to 1 or viceversa.
reader.query(4,5,6,7) // returns 4 because nums[4], nums[5], nums[6], nums[7] have the same value.
we can infer that the most frequent value is found in the last 4 elements.
Index 2, 4, 6, 7 is also a correct answer.

Example 2:
Input: nums = [0,0,1,1,0]
Output: 0

Example 3:
Input: nums = [1,0,1,0,1,0,1,0]
Output: -1


Constraints:
    5 <= nums.length <= 10^5
    0 <= nums[i] <= 1


*/
/**
 * // This is the ArrayReader's API interface.
 * // You should not implement it, or speculate about its implementation
 * struct ArrayReader;
 * impl ArrayReader {
 *     // Compares 4 different elements in the array
 *     // return 4 if the values of the 4 elements are the same (0 or 1).
 *     // return 2 if three elements have a value equal to 0 and one element has value equal to 1 or vice versa.
 *     // return 0 : if two element have a value equal to 0 and two elements have a value equal to 1.
 *     pub fn query(a: i32, b: i32, c: i32, d: i32) -> i32 {}
 *
 *     // Returns the length of the array
 *     pub fn length() -> i32 {}
 * };
 */
/**
 * // This is the ArrayReader's API interface.
 * // You should not implement it, or speculate about its implementation
 * struct ArrayReader;
 * impl ArrayReader {
 *     // Compares 4 different elements in the array
 *     // return 4 if the values of the 4 elements are the same (0 or 1).
 *     // return 2 if three elements have a value equal to 0 and one element has value equal to 1 or vice versa.
 *     // return 0 : if two element have a value equal to 0 and two elements have a value equal to 1.
 *     pub fn query(a: i32, b: i32, c: i32, d: i32) -> i32 {}
 *
 *     // Returns the length of the array
 *     pub fn length() -> i32 {}
 * };
 */
pub struct ArrayReader;
impl ArrayReader {
    pub fn query(&self, _a: i32, _b: i32, _c: i32, _d: i32) -> i32 {
        0
    }
    // Compares the sum of arr[l..r] with the sum of arr[x..y]
    // return 1 if sum(arr[l..r]) > sum(arr[x..y])
    // return 0 if sum(arr[l..r]) == sum(arr[x..y])
    // return -1 if sum(arr[l..r]) < sum(arr[x..y])
    // Returns the length of the array
    pub fn length(&self) -> i32 {
        0
    }
}

pub struct Solution;
impl Solution {
    pub fn get_majority(reader: &ArrayReader) -> i32 {
        let n = reader.length();
        let mut v = vec![0; n as usize];
        v[0] = 1;
        let q0123 = reader.query(0, 1, 2, 3);
        let q0124 = reader.query(0, 1, 2, 4);
        let q0134 = reader.query(0, 1, 3, 4);
        let q0234 = reader.query(0, 2, 3, 4);
        let q1234 = reader.query(1, 2, 3, 4);
        if q0234 == q1234 {
            v[1] = 1;
        }
        if q0134 == q1234 {
            v[2] = 1;
        }
        if q0124 == q1234 {
            v[3] = 1;
        }
        if q0123 == q1234 {
            v[4] = 1;
        }
        let mut prev = q1234;
        for i in 5..n {
            let curr = reader.query(i - 3, i - 2, i - 1, i);
            v[i as usize] = if prev == curr {
                v[i as usize - 4]
            } else {
                1 - v[i as usize - 4]
            };
            prev = curr;
        }
        let sum = v.iter().sum::<i32>();
        if sum * 2 == n as i32 {
            return -1;
        }
        let j = if sum * 2 < n as i32 { 0 } else { 1 };
        if let Some(i) = v.into_iter().position(|x| x == j) {
            i as i32
        } else {
            -1
        }
    }
}
