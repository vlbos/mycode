# Medium

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Schools`

# +-------------+------+
# | Column Name | Type |
# +-------------+------+
# | school\_id   | int  |
# | capacity    | int  |
# +-------------+------+
# school\_id is the column with unique values for this table.
# This table contains information about the capacity of some schools. The capacity is the maximum number of students the school can accept.

# Table: `Exam`

# +---------------+------+
# | Column Name   | Type |
# +---------------+------+
# | score         | int  |
# | student\_count | int  |
# +---------------+------+
# score is the column with unique values for this table.
# Each row in this table indicates that there are student\_count students that got at least score points in the exam.
# The data in this table will be logically correct, meaning a row recording a higher score will have the same or smaller student\_count compared to a row recording a lower score. More formally, for every two rows i and j in the table, if scorei > scorej then student\_counti <= student\_countj.

# Every year, each school announces a **minimum score requirement** that a student needs to apply to it. The school chooses the minimum score requirement based on the exam results of all the students:

# 1.  They want to ensure that even if **every** student meeting the requirement applies, the school can accept everyone.
# 2.  They also want to **maximize** the possible number of students that can apply.
# 3.  They **must** use a score that is in the `Exam` table.

# Write a solution to report the **minimum score requirement** for each school. If there are multiple score values satisfying the above conditions, choose the **smallest** one. If the input data is not enough to determine the score, report `-1`.

# Return the result table in **any order**.

# The result format is in the following example.

# **Example 1:**

# **Input:**
# Schools table:
# +-----------+----------+
# | school\_id | capacity |
# +-----------+----------+
# | 11        | 151      |
# | 5         | 48       |
# | 9         | 9        |
# | 10        | 99       |
# +-----------+----------+
# Exam table:
# +-------+---------------+
# | score | student\_count |
# +-------+---------------+
# | 975   | 10            |
# | 966   | 60            |
# | 844   | 76            |
# | 749   | 76            |
# | 744   | 100           |
# +-------+---------------+
# **Output:**
# +-----------+-------+
# | school\_id | score |
# +-----------+-------+
# | 5         | 975   |
# | 9         | -1    |
# | 10        | 749   |
# | 11        | 744   |
# +-----------+-------+
# **Explanation:** 
# - School 5: The school's capacity is 48. Choosing 975 as the min score requirement, the school will get at most 10 applications, which is within capacity.
# - School 10: The school's capacity is 99. Choosing 844 or 749 as the min score requirement, the school will get at most 76 applications, which is within capacity. We choose the smallest of them, which is 749.
# - School 11: The school's capacity is 151. Choosing 744 as the min score requirement, the school will get at most 100 applications, which is within capacity.
# - School 9: The data given is not enough to determine the min score requirement. Choosing 975 as the min score, the school may get 10 requests while its capacity is 9. We do not have information about higher scores, hence we report -1.

import pandas as pd

def find_cutoff_score(schools: pd.DataFrame, exam: pd.DataFrame) -> pd.DataFrame:

    df = schools.merge(exam, how = 'cross')            

    return (df[df.capacity >= df.student_count]         
              .groupby('school_id')['score'].min()      
              .reset_index()
              .merge(schools, how = 'right')         
              .iloc[:,[0,1]].fillna(-1))              