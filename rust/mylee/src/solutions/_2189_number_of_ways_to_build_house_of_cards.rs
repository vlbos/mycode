// [2189\. Number of Ways to Build House of Cards (Medium)](https://leetcode.com/problems/number-of-ways-to-build-house-of-cards/)[](https://leetcode.ca/2022-03-22-2189-Number-of-Ways-to-Build-House-of-Cards/#2189-number-of-ways-to-build-house-of-cards-medium)
// =================================================================================================================================================================================================================================================================

// You are given an integer `n` representing the number of playing cards you have. A **house of cards** meets the following conditions:

// *   A **house of cards** consists of one or more rows of **triangles** and horizontal cards.
// *   **Triangles** are created by leaning two cards against each other.
// *   One card must be placed horizontally between **all adjacent** triangles in a row.
// *   Any triangle on a row higher than the first must be placed on a horizontal card from the previous row.
// *   Each triangle is placed in the **leftmost** available spot in the row.

// Return _the number of **distinct** **house of cards** you can build using **all**_ `n` _cards._ Two houses of cards are considered distinct if there exists a row where the two houses contain a different number of cards.

// **Example 1:**

// ![](https://assets.leetcode.com/uploads/2022/02/27/image-20220227213243-1.png)

// **Input:** n = 16
// **Output:** 2
// **Explanation:** The two valid houses of cards are shown.
// The third house of cards in the diagram is not valid because the rightmost triangle on the top row is not placed on top of a horizontal card.

// **Example 2:**

// ![](https://assets.leetcode.com/uploads/2022/02/27/image-20220227213306-2.png)

// **Input:** n = 2
// **Output:** 1
// **Explanation:** The one valid house of cards is shown.

// **Example 3:**

// ![](https://assets.leetcode.com/uploads/2022/02/27/image-20220227213331-3.png)

// **Input:** n = 4
// **Output:** 0
// **Explanation:** The three houses of cards in the diagram are not valid.
// The first house of cards needs a horizontal card placed between the two triangles.
// The second house of cards uses 5 cards.
// The third house of cards uses 2 cards.

// **Constraints:**

// *   `1 <= n <= 500`

// **Companies**:
// [Airbnb](https://leetcode.com/company/airbnb)

// **Related Topics**:
// [Math](https://leetcode.com/tag/math/), [Dynamic Programming](https://leetcode.com/tag/dynamic-programming/)

// **Similar Questions**:

// *   [Champagne Tower (Medium)](https://leetcode.com/problems/champagne-tower/)

// Solution 1. Top-down DP[](https://leetcode.ca/2022-03-22-2189-Number-of-Ways-to-Build-House-of-Cards/#solution-1-top-down-dp)
// -----------------------------------------------------------------------------------------------------------------------------

// **Intuition**: Try using `1, 2, 3, ...` triangles in the first row, then count how many different houses we can build on top of this first row.

// **Algorithm**:

// If we build `j` houses in the first row, then we can at most build `j - 1` houses in the second row.

// Let `dp[i][j]` be the number of different houses we can build given `i` cards and `j` houses allowed in the current row.

// For `dp[i][j]`, we can try building `1, 2, 3, ..., j` houses in the current row.

//     dp[i][j] = SUM( dp[i - usedCards][housesInCurrentRow - 1] )
//                     where usedCards = 3 * housesInCurrentRow - 1

// The trivial case is `dp[0][j] = 1` – we add one to the answer once we used all the cards.

//     // OJ: https://leetcode.com/problems/number-of-ways-to-build-house-of-cards/
//     // Time: O(N^2)
//     // Space: O(N^2)
//     class Solution {
//     public:
//         int house_of_cards(int n) {

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn house_of_cards(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        dp[0] = 1;
        for h in 1..=n / 3 + 1 {
            let used = 3 * h - 1;
            for i in (used..=n).rev() {
                dp[i] += dp[i - used];
            }
        }
        dp[n]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_house_of_cards_1() {
        assert_eq!(2, Solution::house_of_cards(16));
    }
    #[test]
    pub fn test_house_of_cards_2() {
        assert_eq!(1, Solution::house_of_cards(2));
    }
    #[test]
    pub fn test_house_of_cards_3() {
        assert_eq!(0, Solution::house_of_cards(4));
    }
}
