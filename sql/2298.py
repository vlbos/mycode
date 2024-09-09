# Medium

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Tasks`

# +-------------+------+
# | Column Name | Type |
# +-------------+------+
# | task\_id     | int  |
# | assignee\_id | int  |
# | submit\_date | date |
# +-------------+------+
# task\_id is the primary key (column with unique values) for this table.
# Each row in this table contains the ID of a task, the id of the assignee, and the submission date.

# Write a solution to report:

# *   the number of tasks that were submitted during the weekend (Saturday, Sunday) as `weekend_cnt`, and
# *   the number of tasks that were submitted during the working days as `working_cnt`.

# Return the result table in **any order**.

# The result format is shown in the following example.

# **Example 1:**

# **Input:** 
# Tasks table:
# +---------+-------------+-------------+
# | task\_id | assignee\_id | submit\_date |
# +---------+-------------+-------------+
# | 1       | 1           | 2022-06-13  |
# | 2       | 6           | 2022-06-14  |
# | 3       | 6           | 2022-06-15  |
# | 4       | 3           | 2022-06-18  |
# | 5       | 5           | 2022-06-19  |
# | 6       | 7           | 2022-06-19  |
# +---------+-------------+-------------+
# **Output:** 
# +-------------+-------------+
# | weekend\_cnt | working\_cnt |
# +-------------+-------------+
# | 3           | 3           |
# +-------------+-------------+
# **Explanation:** 
# Task 1 was submitted on Monday.
# Task 2 was submitted on Tuesday.
# Task 3 was submitted on Wednesday.
# Task 4 was submitted on Saturday.
# Task 5 was submitted on Sunday.
# Task 6 was submitted on Sunday.
# 3 tasks were submitted during the weekend.
# 3 tasks were submitted during the working days.



import pandas as pd

def count_tasks(tasks: pd.DataFrame) -> pd.DataFrame:
    tasks['status'] = np.where(tasks['submit_date'].dt.weekday>=5,'weekend','working')
    return pd.DataFrame({'weekend_cnt':[tasks[tasks['status'] == 'weekend'].shape[0]],'working_cnt':[tasks[tasks['status'] == 'working'].shape[0]]})
    