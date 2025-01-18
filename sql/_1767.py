# Hard

# Topics

# Companies

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Tasks`

# +----------------+---------+
# | Column Name    | Type    |
# +----------------+---------+
# | task\_id        | int     |
# | subtasks\_count | int     |
# +----------------+---------+
# task\_id is the column with unique values for this table.
# Each row in this table indicates that task\_id was divided into subtasks\_count subtasks labeled from 1 to subtasks\_count.
# It is guaranteed that 2 <= subtasks\_count <= 20.

# Table: `Executed`

# +---------------+---------+
# | Column Name   | Type    |
# +---------------+---------+
# | task\_id       | int     |
# | subtask\_id    | int     |
# +---------------+---------+
# (task\_id, subtask\_id) is the combination of columns with unique values for this table.
# Each row in this table indicates that for the task task\_id, the subtask with ID subtask\_id was executed successfully.
# It is **guaranteed** that subtask\_id <= subtasks\_count for each task\_id.

# Write a solution to report the IDs of the missing subtasks for each `task_id`.

# Return the result table in **any order**.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Tasks table:
# +---------+----------------+
# | task\_id | subtasks\_count |
# +---------+----------------+
# | 1       | 3              |
# | 2       | 2              |
# | 3       | 4              |
# +---------+----------------+
# Executed table:
# +---------+------------+
# | task\_id | subtask\_id |
# +---------+------------+
# | 1       | 2          |
# | 3       | 1          |
# | 3       | 2          |
# | 3       | 3          |
# | 3       | 4          |
# +---------+------------+
# **Output:** 
# +---------+------------+
# | task\_id | subtask\_id |
# +---------+------------+
# | 1       | 1          |
# | 1       | 3          |
# | 2       | 1          |
# | 2       | 2          |
# +---------+------------+
# **Explanation:** 
# Task 1 was divided into 3 subtasks (1, 2, 3). Only subtask 2 was executed successfully, so we include (1, 1) and (1, 3) in the answer.
# Task 2 was divided into 2 subtasks (1, 2). No subtask was executed successfully, so we include (2, 1) and (2, 2) in the answer.
# Task 3 was divided into 4 subtasks (1, 2, 3, 4). All of the subtasks were executed successfully.


import pandas as pd

def find_subtasks(tasks: pd.DataFrame, executed: pd.DataFrame) -> pd.DataFrame:
    return (
        tasks.assign(subtask_id=tasks.apply(
            lambda row: [i for i in range(1, row["subtasks_count"]+1)],
            axis=1,
        ))
        .explode("subtask_id")
        .merge(executed, indicator=True, how="outer")
        .query("_merge=='left_only'")
    )[["task_id", "subtask_id"]]