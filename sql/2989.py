# Medium

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Scores`

# +--------------+---------+
# | Column Name  | Type    |
# +--------------+---------+
# | student\_id   | int     |
# | student\_name | varchar |
# | assignment1  | int     |
# | assignment2  | int     |
# | assignment3  | int     |
# +--------------+---------+
# student\_id is column of unique values for this table.
# This table contains student\_id, student\_name, assignment1, assignment2, and assignment3.

# Write a solution to calculate the **difference** in the **total score** (sum of all `3` assignments) between the **highest score** obtained by students and the **lowest score** obtained by them.

# Return _the result table in **any** order__._

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Scores table:
# +------------+--------------+-------------+-------------+-------------+
# | student\_id | student\_name | assignment1 | assignment2 | assignment3 |
# +------------+--------------+-------------+-------------+-------------+
# | 309        | Owen         | 88          | 47          | 87          |
# | 321        | Claire       | 98          | 95          | 37          |     
# | 338        | Julian       | 100         | 64          | 43          |  
# | 423        | Peyton       | 60          | 44          | 47          |  
# | 896        | David        | 32          | 37          | 50          | 
# | 235        | Camila       | 31          | 53          | 69          | 
# +------------+--------------+-------------+-------------+-------------+
# **Output**
# +---------------------+
# | difference\_in\_score | 
# +---------------------+
# | 111                 | 
# +---------------------+
# **Explanation**
# - student\_id 309 has a total score of 88 + 47 + 87 = 222.
# - student\_id 321 has a total score of 98 + 95 + 37 = 230.
# - student\_id 338 has a total score of 100 + 64 + 43 = 207.
# - student\_id 423 has a total score of 60 + 44 + 47 = 151.
# - student\_id 896 has a total score of 32 + 37 + 50 = 119.
# - student\_id 235 has a total score of 31 + 53 + 69 = 153.
# student\_id 321 has the highest score of 230, while student\_id 896 has the lowest score of 119. Therefore, the difference between them is 111.



import pandas as pd

def class_performance(scores: pd.DataFrame) -> pd.DataFrame:
    total = scores.assignment1 + scores.assignment2 + scores.assignment3
    return pd.DataFrame({
        'difference_in_score': [total.max() - total.min()]
    })