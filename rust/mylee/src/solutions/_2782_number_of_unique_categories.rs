// # [2782. number of unique categories](https://leetcode.com/problems/number-of-unique-categories)

// ## Description

// You are given an integer n and an object category_handler of class CategoryHandler.

// There are n elements, numbered from 0 to n - 1. Each element has a category,
// and your task is to find the number of unique categories.

// The class CategoryHandler contains the following function, which may help you:

//
// 	boolean haveSameCategory(integer a, integer b): Returns true if a and b are in the same category and false otherwise.
// Also, if either a or b is not a valid number (i.e. it's greater than or equal to nor less than 0), it returns false.
//

// Return the number of unique categories.

//
// ### Example 1:

//
// Input: n = 6, catagoryHandler = [1,1,2,2,3,3]
// Output: 3
// Explanation: There are 6 elements in this example. The first two elements belong to category 1,
// the second two belong to category 2, and the last two elements belong to category 3. So there are 3 unique categories.
//

// ### Example 2:

//
// Input: n = 5, catagoryHandler = [1,2,3,4,5]
// Output: 5
// Explanation: There are 5 elements in this example. Each element belongs to a unique category.
// So there are 5 unique categories.
//

// ### Example 3:

//
// Input: n = 3, catagoryHandler = [1,1,1]
// Output: 1
// Explanation: There are 3 elements in this example. All of them belong to one category. So there is only 1 unique category.
//

//
// Constraints:

//
// 	1 <= n <= 100
//

// ## Solutions

// ### **C++**

// ```cpp
// /**
//  * Definition for a category handler.
//  * class CategoryHandler {
//  * public:
//  *     CategoryHandler(vector<int> categories);
//  *     bool haveSameCategory(int a, int b);
//  * };
//  */
// class Solution {
// public:
//     int number_of_categories(int n, CategoryHandler* category_handler) {

#[allow(dead_code)]
pub struct CategoryHandler {
    categories: Vec<i32>,
}
impl CategoryHandler {
    pub fn new(categories: Vec<i32>) -> Self {
        Self { categories }
    }
    fn have_same_category(&self, a: i32, b: i32) -> bool {
        self.categories[a as usize] == self.categories[b as usize]
    }
}

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn number_of_categories(n: i32, category_handler: Option<Box<CategoryHandler>>) -> i32 {
        let mut parents: Vec<i32> = (0..n).collect();
        fn find(x: i32, parents: &mut Vec<i32>) -> i32 {
            let px = parents[x as usize];
            if px != x {
                parents[x as usize] = find(px, parents);
            }
            parents[x as usize]
        }
        for a in 0..n {
            for b in a + 1..n {
                if category_handler.as_ref().unwrap().have_same_category(a, b) {
                    let (pa, pb) = (find(a, &mut parents), find(b, &mut parents));
                    parents[pa as usize] = pb;
                }
            }
        }
        parents
            .into_iter()
            .enumerate()
            .filter(|(i, x)| *i == *x as usize)
            .count() as _
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_number_of_categories_1() {
        let category_handler = Some(Box::new(CategoryHandler::new(vec![1, 1, 2, 2, 3, 3])));
        assert_eq!(3, Solution::number_of_categories(6, category_handler));
    }
    #[test]
    pub fn test_number_of_categories_2() {
        let category_handler = Some(Box::new(CategoryHandler::new(vec![1, 2, 3, 4, 5])));
        assert_eq!(5, Solution::number_of_categories(5, category_handler));
    }
    #[test]
    pub fn test_number_of_categories_3() {
        let category_handler = Some(Box::new(CategoryHandler::new(vec![1, 1, 1])));
        assert_eq!(1, Solution::number_of_categories(3, category_handler));
    }
}
