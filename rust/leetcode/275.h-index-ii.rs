/*
 * @lc app=leetcode id=275 lang=rust
 *
 * [275] H-Index II
 */

// @lc code=start
impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut i = 0;
        let mut j = citations.len() - 1;
        let mut mid = 0;
        let mut premid = citations.len();
        while i <= j {
            mid = (i + j) / 2;
            if (mid == 0 || citations.len() - mid + 1 > citations[mid - 1] as usize)
                && citations.len() - mid <= citations[mid] as usize
            {
                ans = citations.len() - mid;
                break;
            }
            if citations.len() - mid < citations[mid] as usize {
                j = mid;
            } else {
                i = mid+1;
            }
            if premid==mid{
            break;
            }
            premid=mid;
        }
        ans as i32
    }
}
// @lc code=end
