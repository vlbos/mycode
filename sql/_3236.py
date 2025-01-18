# Hard

# Topics

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
# | salary        | int     |
# +---------------+---------+
# employee\_id is the unique identifier for this table.
# manager\_id is the employee\_id of the employee's manager. The CEO has a NULL manager\_id.

# Write a solution to find subordinates of the CEO (both **direct** and **indirect**), along with their **level in the hierarchy** and their **salary difference** from the CEO.

# The result should have the following columns:

# The query result format is in the following example.

# *   `subordinate_id`: The employee\_id of the subordinate
# *   `subordinate_name`: The name of the subordinate
# *   `hierarchy_level`: The level of the subordinate in the hierarchy (`1` for **direct** reports, `2` for **their direct** reports, and **so on**)
# *   `salary_difference`: The difference between the subordinate's salary and the CEO's salary

# Return _the result table ordered by_ `hierarchy_level` _**ascending**_, _and then by_ `subordinate_id` _**ascending**_.

# The query result format is in the following example.

# **Example:**

# **Input:**

# `Employees` table:

# +-------------+----------------+------------+---------+
# | employee\_id | employee\_name  | manager\_id | salary  |
# +-------------+----------------+------------+---------+
# | 1           | Alice          | NULL       | 150000  |
# | 2           | Bob            | 1          | 120000  |
# | 3           | Charlie        | 1          | 110000  |
# | 4           | David          | 2          | 105000  |
# | 5           | Eve            | 2          | 100000  |
# | 6           | Frank          | 3          | 95000   |
# | 7           | Grace          | 3          | 98000   |
# | 8           | Helen          | 5          | 90000   |
# +-------------+----------------+------------+---------+

# **Output:**

# +----------------+------------------+------------------+-------------------+
# | subordinate\_id | subordinate\_name | hierarchy\_level  | salary\_difference |
# +----------------+------------------+------------------+-------------------+
# | 2              | Bob              | 1                | -30000            |
# | 3              | Charlie          | 1                | -40000            |
# | 4              | David            | 2                | -45000            |
# | 5              | Eve              | 2                | -50000            |
# | 6              | Frank            | 2                | -55000            |
# | 7              | Grace            | 2                | -52000            |
# | 8              | Helen            | 3                | -60000            |
# +----------------+------------------+------------------+-------------------+

# **Explanation:**

# *   Bob and Charlie are direct subordinates of Alice (CEO) and thus have a hierarchy\_level of 1.
# *   David and Eve report to Bob, while Frank and Grace report to Charlie, making them second-level subordinates (hierarchy\_level 2).
# *   Helen reports to Eve, making Helen a third-level subordinate (hierarchy\_level 3).
# *   Salary differences are calculated relative to Alice's salary of 150000.
# *   The result is ordered by hierarchy\_level ascending, and then by subordinate\_id ascending.

# **Note:** The output is ordered first by hierarchy\_level in ascending order, then by subordinate\_id in ascending order.

import pandas as pd

def find_subordinates(employees: pd.DataFrame) -> pd.DataFrame:

    ceo = employees[employees['manager_id'].isna()].iloc[0]
    ceo_id = ceo['employee_id']
    ceo_salary = ceo['salary']
    

    result = []
    

    def dfs(manager_id, level):
        subordinates = employees[employees['manager_id'] == manager_id]
        for _, row in subordinates.iterrows():
            salary_difference = row['salary'] - ceo_salary
            result.append({
                'subordinate_id': row['employee_id'],
                'subordinate_name': row['employee_name'],
                'hierarchy_level': level,
                'salary_difference': salary_difference
            })
            dfs(row['employee_id'], level + 1)
    

    dfs(ceo_id, 1)
    
    if result: 
        result_df = pd.DataFrame(result)
        result_df = result_df.sort_values(by=['hierarchy_level', 'subordinate_id']).reset_index(drop=True)
        return result_df
    else:
        return pd.DataFrame(columns=['subordinate_id', 'subordinate_name', 'hierarchy_level', 'salary_difference'])    