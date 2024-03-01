# [3053. Classifying Triangles by Lengths](https://leetcode.com/problems/classifying-triangles-by-lengths)

[中文文档](/solution/3000-3099/3053.Classifying%20Triangles%20by%20Lengths/README.md)



## Description

Table: Triangles


+-------------+------+ 
| Column Name | Type | 
+-------------+------+ 
| A           | int  | 
| B           | int  |
| C           | int  |
+-------------+------+
(A, B, C) is the primary key for this table.
Each row include the lengths of each of a triangle&#39;s three sides.


Write a query to find the type of triangle. Output one of the following for each row:


	Equilateral: It&#39;s a triangle with 3 sides of equal length.
	Isosceles: It&#39;s a triangle with 2 sides of equal length.
	Scalene: It&#39;s a triangle with 3 sides of differing lengths.
	Not A Triangle: The given values of A, B, and C don&#39;t form a triangle.


Return the result table in any order.

The result format is in the following example.

&nbsp;
Example 1:


Input: 
Triangles table:
+----+----+----+
| A  | B  | C  |
+----+----+----+
| 20 | 20 | 23 |
| 20 | 20 | 20 |
| 20 | 21 | 22 |
| 13 | 14 | 30 |
+----+----+----+
Output: 
+----------------+
| triangle_type  | 
+----------------+
| Isosceles      | 
| Equilateral    |
| Scalene        |
| Not A Triangle |
+----------------+
Explanation: 
- Values in the first row from an Isosceles triangle, because A = B.
- Values in the second row from an Equilateral triangle, because A = B = C.
- Values in the third row from an Scalene triangle, because A != B != C.
- Values in the fourth row cannot form a triangle, because the combined value of sides A and B is not larger than that of side C.

## Solutions

### Solution 1: Using CASE WHEN Statement

We can use the `CASE WHEN` statement to determine the type of the triangle.

First, we need to determine whether the three sides can form a triangle. If not, we return `Not A Triangle`.

Then, we check if the lengths of the three sides are equal. If they are, we return `Equilateral`.

Next, we check if there are two sides with equal length. If there are, we return `Isosceles`.

Otherwise, it means that the lengths of the three sides are all different, so we return `Scalene`.



```sql
# Write your MySQL query statement below
SELECT
    CASE
        WHEN A + B <= C
        OR A + C <= B
        OR B + C <= A THEN 'Not A Triangle'
        WHEN A = B
        AND B = c THEN 'Equilateral'
        WHEN (A = B) + (B = C) + (A = C) = 1 THEN 'Isosceles'
        ELSE 'Scalene'
    END AS triangle_type
FROM Triangles;
```




