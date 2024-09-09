# Hard

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: Heights

# +-------------+------+
# | Column Name | Type |
# +-------------+------+
# | id          | int  |
# | height      | int  |
# +-------------+------+
# id is the primary key (column with unique values) for this table, and it is guaranteed to be in sequential order.
# Each row of this table contains an id and height.

# Write a solution to calculate the amount of rainwater can be **trapped between the bars** in the landscape, considering that each bar has a **width** of `1` unit.

# Return _the result table in_ **any** _order._

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Heights table:
# +-----+--------+
# | id  | height |
# +-----+--------+
# | 1   | 0      |
# | 2   | 1      |
# | 3   | 0      |
# | 4   | 2      |
# | 5   | 1      |
# | 6   | 0      |
# | 7   | 1      |
# | 8   | 3      |
# | 9   | 2      |
# | 10  | 1      |
# | 11  | 2      |
# | 12  | 1      |
# +-----+--------+
# **Output:** 
# +---------------------+
# | total\_trapped\_water | 
# +---------------------+
# | 6                   | 
# +---------------------+
# **Explanation:** 
# ![](https://assets.leetcode.com/uploads/2024/02/26/trapping_rain_water.png)

# The elevation map depicted above (in the black section) is graphically represented with the x-axis denoting the id and the y-axis representing the heights \[0,1,0,2,1,0,1,3,2,1,2,1\]. In this scenario, 6 units of rainwater are trapped within the blue section.



import pandas as pd

def calculate_trapped_rain_water(heights: pd.DataFrame) -> pd.DataFrame:
    df = heights.assign(ones = heights["height"].apply(lambda x: x*[1]))["ones"].apply(pd.Series)
    df2 = (df.ffill() == 1) & (df.bfill() == 1)
    return pd.DataFrame({"total_trapped_water": [((df == 1)^(df2 == 1)).sum().sum()]})