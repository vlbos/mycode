# [3415\. Find Products with Three Consecutive Digits 🔒](https://leetcode.com/problems/find-products-with-three-consecutive-digits)
# ==================================================================================================================================

# [![](https://img.shields.io/badge/Difficulty-Easy-4051B5?style=flat-square)](https://img.shields.io/badge/Difficulty-Easy-4051B5?style=flat-square)

# Description
# -----------

# Table: `Products`

# +-------------+---------+
# | Column Name | Type    |
# +-------------+---------+
# | product\_id  | int     |
# | name        | varchar |
# +-------------+---------+
# product\_id is the unique key for this table.
# Each row of this table contains the ID and name of a product.

# Write a solution to find all **products** whose names contain a **sequence of exactly three digits in a row**. 

# Return _the result table ordered by_ `product_id` _in **ascending** order._

# The result format is in the following example.

# **Example:**

# **Input:**

# products table:

# +-------------+--------------------+
# | product\_id  | name               |
# +-------------+--------------------+
# | 1           | ABC123XYZ          |
# | 2           | A12B34C            |
# | 3           | Product56789       |
# | 4           | NoDigitsHere       |
# | 5           | 789Product         |
# | 6           | Item003Description |
# | 7           | Product12X34       |
# +-------------+--------------------+

# **Output:**

# +-------------+--------------------+
# | product\_id  | name               |
# +-------------+--------------------+
# | 1           | ABC123XYZ          |
# | 5           | 789Product         |
# | 6           | Item003Description |
# +-------------+--------------------+

# **Explanation:**

# *   Product 1: ABC123XYZ contains the digits 123.
# *   Product 5: 789Product contains the digits 789.
# *   Product 6: Item003Description contains 003, which is exactly three digits.

# **Note:**

# *   Results are ordered by `product_id` in ascending order.
# *   Only products with exactly three consecutive digits in their names are included in the result.



 import pandas as pd


def find_products(products: pd.DataFrame) -> pd.DataFrame:
    filtered = products[
        products["name"].str.contains(r"(^|[^0-9])[0-9]{3}([^0-9]|$)", regex=True)
    ]
    return filtered.sort_values(by="product_id")




# Write your MySQL query statement below
SELECT product_id, name
FROM Products
WHERE name REGEXP '(^|[^0-9])[0-9]{3}([^0-9]|$)'
ORDER BY 1;