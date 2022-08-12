// 1151\. Minimum Swaps to Group All 1's Together
// ==============================================

// Given a binary array `data`, return the minimum number of swaps required to group all `1`’s present in the array together in **any place** in the array.

// **Example 1:**

// **Input:** \[1,0,1,0,1\]
// **Output:** 1
// **Explanation:** 
// There are 3 ways to group all 1's together:
// \[1,1,1,0,0\] using 1 swap.
// \[0,1,1,1,0\] using 2 swaps.
// \[0,0,1,1,1\] using 1 swap.
// The minimum is 1.

// **Example 2:**

// **Input:** \[0,0,0,1,0\]
// **Output:** 0
// **Explanation:** 
// Since there is only one 1 in the array, no swaps needed.

// **Example 3:**

// **Input:** \[1,0,1,0,1,0,0,1,1,0,1\]
// **Output:** 3
// **Explanation:** 
// One possible solution that uses 3 swaps is \[0,0,0,0,0,1,1,1,1,1,1\].

// **Note****:**

// 1.  `1 <= data.length <= 10^5`
// 2.  `0 <= data[i] <= 1`

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Expedia](https://leetcode.ca/tags/#Expedia)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn   min_swaps(data: Vec<i32>) -> i32 {
        let count = data.iter().filter(|&x|*x==1).count() as usize;
        let mut ans=i32::MAX;
        let mut start=0;
        let mut zeros=0;
        for  (end,&v) in data.iter().enumerate(){
            if v==0{
            zeros+=1;
            }
            if end-start+1>count {
                 if data[start]==0{
                        zeros-=1;
                    } 
                start+=1;
            }
            if end-start+1==count {
                 ans=ans.min(zeros);
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
   pub fn  test_min_swaps_1() {
        assert_eq!(1, Solution::min_swaps(vec![1,0,1,0,1]));
    }
  #[test]
   pub fn  test_min_swaps_2() {
        assert_eq!(0, Solution::min_swaps(vec![0,0,0,1,0]));
    }
  #[test]
   pub fn  test_min_swaps_3() {
        assert_eq!(3, Solution::min_swaps(vec![1,0,1,0,1,0,0,1,1,0,1]));
    }
}
