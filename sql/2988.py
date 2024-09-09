# Medium

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Employees`

# +-------------+---------+
# | Column Name | Type    |
# +-------------+---------+
# | emp\_id      | int     |
# | emp\_name    | varchar |
# | dep\_id      | int     |
# | position    | varchar |
# +-------------+---------+
# emp\_id is column of unique values for this table.
# This table contains emp\_id, emp\_name, dep\_id, and position.

# Write a solution to find the **name** of the **manager** from the **largest department**. There may be multiple largest departments when the number of employees in those departments is the same.

# Return _the result table sorted by_ `dep_id` _in **ascending** order__._

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Employees table:
# +--------+----------+--------+---------------+
# | emp\_id | emp\_name | dep\_id | position      | 
# +--------+----------+--------+---------------+
# | 156    | Michael  | 107    | Manager       |
# | 112    | Lucas    | 107    | Consultant    |    
# | 8      | Isabella | 101    | Manager       | 
# | 160    | Joseph   | 100    | Manager       | 
# | 80     | Aiden    | 100    | Engineer      | 
# | 190    | Skylar   | 100    | Freelancer    | 
# | 196    | Stella   | 101    | Coordinator   |
# | 167    | Audrey   | 100    | Consultant    |
# | 97     | Nathan   | 101    | Supervisor    |
# | 128    | Ian      | 101    | Administrator |
# | 81     | Ethan    | 107    | Administrator |
# +--------+----------+--------+---------------+
# **Output**
# +--------------+--------+
# | manager\_name | dep\_id | 
# +--------------+--------+
# | Joseph       | 100    | 
# | Isabella     | 101    | 
# +--------------+--------+
# **Explanation**
# - Departments with IDs 100 and 101 each has a total of 4 employees, while department 107 has 3 employees. Since both departments 100 and 101 have an equal number of employees, their respective managers will be included.
# Output table is ordered by dep\_id in ascending order.


import pandas as pd

def find_manager(employees: pd.DataFrame) -> pd.DataFrame:
    employees['cnt'] = employees.groupby('dep_id')['emp_id'].transform('count')
    employees['rnk'] = employees['cnt'].rank(method = 'min', ascending = False)
    return employees[(employees['rnk'] == 1) & (employees['position'] == 'Manager')][['emp_name', 'dep_id']].rename(columns = {'emp_name': 'manager_name'}).sort_values('dep_id', ascending = True)
    