# Medium

# Topics

# Companies

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Employees`

# +---------------+---------+
# | Column Name   | Type    |
# +---------------+---------+
# | employee\_id   | int     |
# | employee\_name | varchar |
# | manager\_id    | int     |
# +---------------+---------+
# employee\_id is the column of unique values for this table.
# Each row of this table indicates that the employee with ID employee\_id and name employee\_name reports his work to his/her direct manager with manager\_id
# The head of the company is the employee with employee\_id = 1.

# Write a solution to find `employee_id` of all employees that directly or indirectly report their work to the head of the company.

# The indirect relation between managers **will not exceed three managers** as the company is small.

# Return the result table in **any order**.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Employees table:
# +-------------+---------------+------------+
# | employee\_id | employee\_name | manager\_id |
# +-------------+---------------+------------+
# | 1           | Boss          | 1          |
# | 3           | Alice         | 3          |
# | 2           | Bob           | 1          |
# | 4           | Daniel        | 2          |
# | 7           | Luis          | 4          |
# | 8           | Jhon          | 3          |
# | 9           | Angela        | 8          |
# | 77          | Robert        | 1          |
# +-------------+---------------+------------+
# **Output:** 
# +-------------+
# | employee\_id |
# +-------------+
# | 2           |
# | 77          |
# | 4           |
# | 7           |
# +-------------+
# **Explanation:** 
# The head of the company is the employee with employee\_id 1.
# The employees with employee\_id 2 and 77 report their work directly to the head of the company.
# The employee with employee\_id 4 reports their work indirectly to the head of the company 4 --> 2 --> 1. 
# The employee with employee\_id 7 reports their work indirectly to the head of the company 7 --> 4 --> 2 --> 1.
# The employees with employee\_id 3, 8, and 9 do not report their work to the head of the company directly or indirectly.


import pandas as pd

def find_reporting_people(employees: pd.DataFrame) -> pd.DataFrame:
    f = employees.loc[(employees.manager_id == 1) & (employees.employee_id != 1),['employee_id']]
    s = employees.loc[employees.manager_id.isin(f.employee_id),['employee_id']]
    t = employees.loc[employees.manager_id.isin(s.employee_id),['employee_id']]
    return pd.concat([f,s,t])