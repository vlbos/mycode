// 1570\. dot product of two sparse vectors
// ========================================

// Given two sparse vectors, compute their dot product.

// Implement class `SparseVector`:

// *   `SparseVector(nums)` Initializes the object with the vector `nums`
// *   `dotProduct(vec)` Compute the dot product between the instance of _SparseVector_ and `vec`

// A **sparse vector** is a vector that has mostly zero values, you should store the sparse vector **efficiently** and compute the dot product between two _SparseVector_.

// **Follow up:** What if only one of the vectors is sparse?

// **Example 1:**

// **Input:** nums1 = \[1,0,0,2,3\], nums2 = \[0,3,0,4,0\]
// **Output:** 8
// **Explanation:** v1 = SparseVector(nums1) , v2 = SparseVector(nums2)
// v1.dotProduct(v2) = 1\*0 + 0\*3 + 0\*0 + 2\*4 + 3\*0 = 8

// **Example 2:**

// **Input:** nums1 = \[0,1,0,0,0\], nums2 = \[0,0,0,0,2\]
// **Output:** 0
// **Explanation:** v1 = SparseVector(nums1) , v2 = SparseVector(nums2)
// v1.dotProduct(v2) = 0\*0 + 1\*0 + 0\*0 + 0\*0 + 0\*2 = 0

// **Example 3:**

// **Input:** nums1 = \[0,1,0,0,2,0,0\], nums2 = \[1,0,0,0,3,0,4\]
// **Output:** 6

// **Constraints:**

// *   `n == nums1.length == nums2.length`
// *   `1 <= n <= 10^5`
// *   `0 <= nums1[i], nums2[i] <= 100`

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Unknown](https://leetcode.ca/tags/#Unknown)

struct SparseVector {
    nums: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl SparseVector {
    fn new(nums: Vec<i32>) -> Self {
        Self { nums }
    }

    // Return the dotProduct of two sparse vectors
    pub fn dot_product(&self, vec: SparseVector) -> i32 {
        self.nums
            .iter()
            .zip(&vec.nums)
            .fold(0, |c, (a, b)| c + a * b)
    }
}

/**
 * Your SparseVector object will be instantiated and called as such:
 * let v1 = SparseVector::new(nums1);
 * let v2 = SparseVector::new(nums2);
 * let ans = v1.dot_product(v2);
 */
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_dot_product_1() {
        assert_eq!(
            8,
            SparseVector::new(vec![1, 0, 0, 2, 3])
                .dot_product(SparseVector::new(vec![0, 3, 0, 4, 0]))
        );
    }
    #[test]
    pub fn test_dot_product_2() {
        assert_eq!(
            0,
            SparseVector::new(vec![0, 1, 0, 0, 0])
                .dot_product(SparseVector::new(vec![0, 0, 0, 0, 2]))
        );
    }
    #[test]
    pub fn test_dot_product_3() {
        assert_eq!(
            6,
            SparseVector::new(vec![0, 1, 0, 0, 2, 0, 0])
                .dot_product(SparseVector::new(vec![1, 0, 0, 0, 3, 0, 4]))
        );
    }
}
