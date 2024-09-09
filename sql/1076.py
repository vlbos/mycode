# Easy

# Topics

# Companies

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Project`

# +-------------+---------+
# | Column Name | Type    |
# +-------------+---------+
# | project\_id  | int     |
# | employee\_id | int     |
# +-------------+---------+
# (project\_id, employee\_id) is the primary key (combination of columns with unique values) of this table.
# employee\_id is a foreign key (reference column) to `Employee` table.
# Each row of this table indicates that the employee with employee\_id is working on the project with project\_id.

# Table: `Employee`

# +------------------+---------+
# | Column Name      | Type    |
# +------------------+---------+
# | employee\_id      | int     |
# | name             | varchar |
# | experience\_years | int     |
# +------------------+---------+
# employee\_id is the primary key (column with unique values) of this table.
# Each row of this table contains information about one employee.

# Write a solution to report all the **projects** that have the most employees.

# Return the result table in **any order**.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Project table:
# +-------------+-------------+
# | project\_id  | employee\_id |
# +-------------+-------------+
# | 1           | 1           |
# | 1           | 2           |
# | 1           | 3           |
# | 2           | 1           |
# | 2           | 4           |
# +-------------+-------------+
# Employee table:
# +-------------+--------+------------------+
# | employee\_id | name   | experience\_years |
# +-------------+--------+------------------+
# | 1           | Khaled | 3                |
# | 2           | Ali    | 2                |
# | 3           | John   | 1                |
# | 4           | Doe    | 2                |
# +-------------+--------+------------------+
# **Output:** 
# +-------------+
# | project\_id  |
# +-------------+
# | 1           |
# +-------------+
# **Explanation:** The first project has 3 employees while the second one has 2.



import pandas as pd

def project_employees_ii(project: pd.DataFrame, employee: pd.DataFrame) -> pd.DataFrame:
    df = project.groupby('project_id').size().reset_index(name='cnt')
    max_cnt = df['cnt'].max()
 
    return df[df['cnt'] == max_cnt][['project_id']]