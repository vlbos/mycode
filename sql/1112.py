# Medium

# Topics

# Companies

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Enrollments`

# +---------------+---------+
# | Column Name   | Type    |
# +---------------+---------+
# | student\_id    | int     |
# | course\_id     | int     |
# | grade         | int     |
# +---------------+---------+
# (student\_id, course\_id) is the primary key (combination of columns with unique values) of this table.
# grade is never NULL.

# Write a solution to find the highest grade with its corresponding course for each student. In case of a tie, you should find the course with the smallest `course_id`.

# Return the result table ordered by `student_id` in **ascending order**.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Enrollments table:
# +------------+-------------------+
# | student\_id | course\_id | grade |
# +------------+-----------+-------+
# | 2          | 2         | 95    |
# | 2          | 3         | 95    |
# | 1          | 1         | 90    |
# | 1          | 2         | 99    |
# | 3          | 1         | 80    |
# | 3          | 2         | 75    |
# | 3          | 3         | 82    |
# +------------+-----------+-------+
# **Output:** 
# +------------+-------------------+
# | student\_id | course\_id | grade |
# +------------+-----------+-------+
# | 1          | 2         | 99    |
# | 2          | 2         | 95    |
# | 3          | 3         | 82    |
# +------------+-----------+-------+



import pandas as pd

def highest_grade(enrollments: pd.DataFrame) -> pd.DataFrame:
    return enrollments.sort_values(["student_id","grade","course_id"],ascending=[1,0,1]).groupby("student_id").head(1)


