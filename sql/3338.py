# [3338\. Second Highest Salary II 🔒](https://leetcode.com/problems/second-highest-salary-ii)
# ============================================================================================

# [![](https://img.shields.io/badge/Difficulty-Medium-4051B5?style=flat-square)](https://img.shields.io/badge/Difficulty-Medium-4051B5?style=flat-square)

# Description
# -----------

# Table: `employees`

# +------------------+---------+
# | Column Name      | Type    |
# +------------------+---------+
# | emp\_id           | int     |
# | salary           | int     |
# | dept             | varchar |
# +------------------+---------+
# emp\_id is the unique key for this table.
# Each row of this table contains information about an employee including their ID, salary, and department.

# Write a solution to find the employees who earn the **second-highest salary** in each department. If **multiple employees have the second-highest salary**, **include** **all employees** with **that salary**.

# Return _the result table_ _ordered by_ `emp_id` _in_ _**ascending**_ _order_.

# The result format is in the following example.

# **Example:**

# **Input:**

# employees table:

# +--------+--------+-----------+
# | emp\_id | salary | dept      |
# +--------+--------+-----------+
# | 1      | 70000  | Sales     |
# | 2      | 80000  | Sales     |
# | 3      | 80000  | Sales     |
# | 4      | 90000  | Sales     |
# | 5      | 55000  | IT        |
# | 6      | 65000  | IT        |
# | 7      | 65000  | IT        |
# | 8      | 50000  | Marketing |
# | 9      | 55000  | Marketing |
# | 10     | 55000  | HR        |
# +--------+--------+-----------+

# **Output:**

# +--------+-----------+
# | emp\_id | dept      |
# +--------+-----------+
# | 2      | Sales     |
# | 3      | Sales     |
# | 5      | IT        |
# | 8      | Marketing |
# +--------+-----------+

# **Explanation:**

# *   **Sales Department**:
#     *   Highest salary is 90000 (emp\_id: 4)
#     *   Second-highest salary is 80000 (emp\_id: 2, 3)
#     *   Both employees with salary 80000 are included
# *   **IT Department**:
#     *   Highest salary is 65000 (emp\_id: 6, 7)
#     *   Second-highest salary is 55000 (emp\_id: 5)
#     *   Only emp\_id 5 is included as they have the second-highest salary
# *   **Marketing Department**:
#     *   Highest salary is 55000 (emp\_id: 9)
#     *   Second-highest salary is 50000 (emp\_id: 8)
#     *   Employee 8 is included
# *   **HR Department**:
#     *   Only has one employee
#     *   Not included in the result as it has fewer than 2 employees

import pandas as pd


def find_second_highest_salary(employees: pd.DataFrame) -> pd.DataFrame:
    employees["rk"] = employees.groupby("dept")["salary"].rank(
        method="dense", ascending=False
    )
    second_highest = employees[employees["rk"] == 2][["emp_id", "dept"]]
    return second_highest.sort_values(by="emp_id")