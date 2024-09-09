# Medium

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Students`

# +---------------+------+
# | Column Name   | Type |
# +---------------+------+
# | student\_id    | int  |
# | department\_id | int  |
# | mark          | int  |
# +---------------+------+
# student\_id contains unique values.
# Each row of this table indicates a student's ID, the ID of the department in which the student enrolled, and their mark in the exam.

# Write a solution to report the rank of each student in their department as a percentage, where the rank as a percentage is computed using the following formula: `(student_rank_in_the_department - 1) * 100 / (the_number_of_students_in_the_department - 1)`. The `percentage` should be **rounded to 2 decimal places**. `student_rank_in_the_department` is determined by **descending** `mark`, such that the student with the highest `mark` is `rank 1`. If two students get the same mark, they also get the same rank.

# Return the result table in **any order**.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Students table:
# +------------+---------------+------+
# | student\_id | department\_id | mark |
# +------------+---------------+------+
# | 2          | 2             | 650  |
# | 8          | 2             | 650  |
# | 7          | 1             | 920  |
# | 1          | 1             | 610  |
# | 3          | 1             | 530  |
# +------------+---------------+------+
# **Output:** 
# +------------+---------------+------------+
# | student\_id | department\_id | percentage |
# +------------+---------------+------------+
# | 7          | 1             | 0.0        |
# | 1          | 1             | 50.0       |
# | 3          | 1             | 100.0      |
# | 2          | 2             | 0.0        |
# | 8          | 2             | 0.0        |
# +------------+---------------+------------+
# **Explanation:** 
# For Department 1:
#  - Student 7: percentage = (1 - 1) \* 100 / (3 - 1) = 0.0
#  - Student 1: percentage = (2 - 1) \* 100 / (3 - 1) = 50.0
#  - Student 3: percentage = (3 - 1) \* 100 / (3 - 1) = 100.0
# For Department 2:
#  - Student 2: percentage = (1 - 1) \* 100 / (2 - 1) = 0.0
#  - Student 8: percentage = (1 - 1) \* 100 / (2 - 1) = 0.0



import pandas as pd

def compute_rating(students: pd.DataFrame) -> pd.DataFrame:
    return students.groupby('department_id').apply(lambda g: ((g.set_index('student_id')['mark'].rank(method='min', ascending=False)-1)*100/(len(g)-1)).round(2)).reset_index().rename(columns={'mark': 'percentage'})[['student_id', 'department_id', 'percentage']].fillna(0.).sort_values(['department_id', 'percentage'])