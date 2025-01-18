# Hard

# Topics

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
# | workload    | int     |
# +-------------+---------+
# employee\_id is the primary key (column with unique values) of this table.
# employee\_id is a foreign key (reference column) to `Employee` table.
# Each row of this table indicates that the employee with employee\_id is working on the project with project\_id and the workload of the project.

# Table: `Employees`

# +------------------+---------+
# | Column Name      | Type    |
# +------------------+---------+
# | employee\_id      | int     |
# | name             | varchar |
# | team             | varchar |
# +------------------+---------+
# employee\_id is the primary key (column with unique values) of this table.
# Each row of this table contains information about one employee.

# Write a solution to find the **employees** who are allocated to projects with a **workload that exceeds the average** workload of all employees for **their respective teams**

# Return t_he result table ordered by_ `employee_id`, `project_id` _in **ascending** order._

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Project table:
# +-------------+-------------+----------+
# | project\_id  | employee\_id | workload |
# +-------------+-------------+----------+
# | 1           | 1           |  45      |
# | 1           | 2           |  90      | 
# | 2           | 3           |  12      |
# | 2           | 4           |  68      |
# +-------------+-------------+----------+
# Employees table:
# +-------------+--------+------+
# | employee\_id | name   | team |
# +-------------+--------+------+
# | 1           | Khaled | A    |
# | 2           | Ali    | B    |
# | 3           | John   | B    |
# | 4           | Doe    | A    |
# +-------------+--------+------+
# **Output:** 
# +-------------+------------+---------------+------------------+
# | employee\_id | project\_id | employee\_name | project\_workload |
# +-------------+------------+---------------+------------------+  
# | 2           | 1          | Ali           | 90               | 
# | 4           | 2          | Doe           | 68               | 
# +-------------+------------+---------------+------------------+
# **Explanation:** 
# - Employee with ID 1 has a project workload of 45 and belongs to Team A, where the average workload is 56.50. Since his project workload does not exceed the team's average workload, he will be excluded.
# - Employee with ID 2 has a project workload of 90 and belongs to Team B, where the average workload is 51.00. Since his project workload does exceed the team's average workload, he will be included.
# - Employee with ID 3 has a project workload of 12 and belongs to Team B, where the average workload is 51.00. Since his project workload does not exceed the team's average workload, he will be excluded.
# - Employee with ID 4 has a project workload of 68 and belongs to Team A, where the average workload is 56.50. Since his project workload does exceed the team's average workload, he will be included.
# Result table orderd by employee\_id, project\_id in ascending order.




import pandas as pd

def employees_with_above_avg_workload(project: pd.DataFrame,
                                      employees: pd.DataFrame) -> pd.DataFrame:
    
    df = project.merge(employees)
    mean = df.groupby('team')['workload'].mean().reset_index()
    df = df.merge(mean, how = 'left', on = 'team')

    return df[df.workload_x > df.workload_y
               ].iloc[:,[1,0,3,2]
               ].rename(columns = {'workload_x':'project_workload',
                                    'name':'employee_name'}
               ).sort_values(['employee_id', 'project_id'])