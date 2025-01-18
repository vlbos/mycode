# Medium

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

# Write a solution to report the **most experienced** employees in each project. In case of a tie, report all employees with the maximum number of experience years.

# Return the result table in **any order**.

# The result format is in the following example.

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
# | 3           | John   | 3                |
# | 4           | Doe    | 2                |
# +-------------+--------+------------------+
# **Output:** 
# +-------------+---------------+
# | project\_id  | employee\_id   |
# +-------------+---------------+
# | 1           | 1             |
# | 1           | 3             |
# | 2           | 1             |
# +-------------+---------------+
# **Explanation:** Both employees with id 1 and 3 have the most experience among the employees of the first project. For the second project, the employee with id 1 has the most experience.


import pandas as pd

def project_employees(project: pd.DataFrame, employee: pd.DataFrame) -> pd.DataFrame:
    df = employee.merge(project, how = 'left', on = 'employee_id')
    df['rank'] = df.groupby('project_id')['experience_years'].transform('rank', 'dense', ascending = False)
    return df[df['rank']==1][['project_id','employee_id']] 