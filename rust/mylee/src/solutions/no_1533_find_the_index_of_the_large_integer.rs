/*
We have an integer array arr, where all the integers in arr are equal except for one integer which is larger than the rest of the integers. You will not be given direct access to the array, instead, you will have an API ArrayReader which have the following functions:

    int compareSub(int l, int r, int x, int y): where 0 <= l, r, x, y < ArrayReader.length(), l <= r and x <= y. The function compares the sum of sub-array arr[l..r] with the sum of the sub-array arr[x..y] and returns:


        1 if arr[l]+arr[l+1]+...+arr[r] > arr[x]+arr[x+1]+...+arr[y].
        0 if arr[l]+arr[l+1]+...+arr[r] == arr[x]+arr[x+1]+...+arr[y].
        -1 if arr[l]+arr[l+1]+...+arr[r] < arr[x]+arr[x+1]+...+arr[y].


    int length(): Returns the size of the array.

You are allowed to call compareSub() 20 times at most. You can assume both functions work in O(1) time.

Return the index of the array arr which has the largest integer.

Follow-up:

    What if there are two numbers in arr that are bigger than all other numbers?
    What if there is one number that is bigger than other numbers and one number that is smaller than other numbers?


Example 1:
Input: arr = [7,7,7,7,10,7,7,7]
Output: 4
Explanation: The following calls to the API
reader.compareSub(0, 0, 1, 1) // returns 0 this is a query comparing the sub-array (0, 0) with the sub array (1, 1), (i.e. compares arr[0] with arr[1]).
Thus we know that arr[0] and arr[1] doesn't contain the largest element.
reader.compareSub(2, 2, 3, 3) // returns 0, we can exclude arr[2] and arr[3].
reader.compareSub(4, 4, 5, 5) // returns 1, thus for sure arr[4] is the largest element in the array.
Notice that we made only 3 calls, so the answer is valid.

Example 2:
Input: nums = [6,6,12]
Output: 2


Constraints:
    2 <= arr.length <= 5 * 10^5
    1 <= arr[i] <= 100
    All elements of arr are equal except for one element which is larger than all other elements.


*/
/**
 * // This is the ArrayReader's API interface.
 * // You should not implement it, or speculate about its implementation
 * type ArrayReader struct {
 * }
 * // Compares the sum of arr[l..r] with the sum of arr[x..y]
 * // return 1 if sum(arr[l..r]) > sum(arr[x..y])
 * // return 0 if sum(arr[l..r]) == sum(arr[x..y])
 * // return -1 if sum(arr[l..r]) < sum(arr[x..y])
 * func (this *ArrayReader) compareSub(l, r, x, y int) int {}
 *
 * // Returns the length of the array
 * func (this *ArrayReader) length() int {}
 */

/**
 * // This is the ArrayReader's API interface.
 * // You should not implement it, or speculate about its implementation
 * struct ArrayReader;
 * impl Array Reader {
 *     pub fn compareSub(l: i32, r: i32, x: i32, y: i32) -> i32 {}
 *     // Compares the sum of arr[l..r] with the sum of arr[x..y]
 *     // return 1 if sum(arr[l..r]) > sum(arr[x..y])
 *     // return 0 if sum(arr[l..r]) == sum(arr[x..y])
 *     // return -1 if sum(arr[l..r]) < sum(arr[x..y])
 *     // Returns the length of the array
 * }
 */

pub struct ArrayReader;
impl ArrayReader {
    pub fn compareSub(&self, l: i32, r: i32, x: i32, y: i32) -> i32 {
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
    pub fn get_index(reader: &ArrayReader) -> i32 {
        let n = reader.length();
        let mut lo = 0;
        let mut hi = n - 1;

        while lo < hi {
            let mid_lo = lo + (hi - lo) / 2;
            let mid_hi = lo + (hi - lo) / 2 + (hi - lo) % 2;
            let r = reader.compareSub(lo, mid_lo, mid_hi, hi);

            if r == 0 {
                return mid_lo;
            }

            if r > 0 {
                hi = mid_lo;
            } else {
                lo = mid_hi;
            }
        }

        lo
    }
}

// func getIndex(reader *ArrayReader) int {
//     n := reader.length()
//     l, y := 0, n-1
//     var tmp int
//     for l < y {
//         m := l + (y - l) / 2
//         // 二分查找，将原数组分成两部分，这里要注意
//         // 如果是 [l...y] 是奇数长度，那么中间那个数(即 m 所指向的位置)不在比较范围内
//         if (y - l + 1) & 1 == 1 {
//             tmp = reader.compareSub(l, m-1, m+1, y)
//         } else {
//             tmp = reader.compareSub(l, m, m+1, y)
//         }
//         // 左半部分大于右半部分，说明待求数字肯定在左半部分，注意这里不减 1 是考虑 [l..y] 长度为偶数、m 值在左半部分的情况
//         if tmp > 0 {
//             y = m
//         } else if tmp < 0 {
//             // 左半部分小于右半部分，说明待求数字肯定在右半部分
//             l = m+1
//         } else {
//             // 左右两部分相等，那么大数肯定就在中间了
//             return m
//         }
//     }
//     return l

// }
