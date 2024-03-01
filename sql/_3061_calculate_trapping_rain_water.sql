# [3061. Calculate Trapping Rain Water](https://leetcode.com/problems/calculate-trapping-rain-water)

[中文文档](/solution/3000-3099/3061.Calculate%20Trapping%20Rain%20Water/README.md)



## Description

Table: Heights


+-------------+------+
| Column Name | Type |
+-------------+------+
| id          | int  |
| height      | int  |
+-------------+------+
id is the primary key (column with unique values) for this table, and it is guaranteed to be in sequential order.
Each row of this table contains an id and height.


Write a solution to calculate the amount of rainwater can be trapped between the bars in the landscape, considering that each bar has a width of 1 unit.

Return the result table in any order.

The result format is in the following example.

 
Example 1:


Input: 
Heights table:
+-----+--------+
| id  | height |
+-----+--------+
| 1   | 0      |
| 2   | 1      |
| 3   | 0      |
| 4   | 2      |
| 5   | 1      |
| 6   | 0      |
| 7   | 1      |
| 8   | 3      |
| 9   | 2      |
| 10  | 1      |
| 11  | 2      |
| 12  | 1      |
+-----+--------+
Output: 
+---------------------+
| total_trapped_water | 
+---------------------+
| 6                   | 
+---------------------+
Explanation: 


The elevation map depicted above (in the black section) is graphically represented with the x-axis denoting the id and the y-axis representing the heights [0,1,0,2,1,0,1,3,2,1,2,1]. In this scenario, 6 units of rainwater are trapped within the blue section.


## Solutions

### Solution 1: Window Function + Summation

We use the window function `MAX(height) OVER (ORDER BY id)` to calculate the maximum height for each position and its left side, and use `MAX(height) OVER (ORDER BY id DESC)` to calculate the maximum height for each position and its right side, denoted as `l` and `r` respectively. Then, the amount of water stored at each position is `min(l, r) - height`. Finally, we sum them up.



```sql
# Write your MySQL query statement below
WITH
    T AS (
        SELECT
            *,
            MAX(height) OVER (ORDER BY id) AS l,
            MAX(height) OVER (ORDER BY id DESC) AS r
        FROM Heights
    )
SELECT SUM(LEAST(l, r) - height) AS total_trapped_water
FROM T;
```

```python
import pandas as pd


def calculate_trapped_rain_water(heights: pd.DataFrame) -> pd.DataFrame:
    heights["l"] = heights["height"].cummax()
    heights["r"] = heights["height"][::-1].cummax()[::-1]
    heights["trapped_water"] = heights[["l", "r"]].min(axis=1) - heights["height"]
    return pd.DataFrame({"total_trapped_water": [heights["trapped_water"].sum()]})
```




