# Medium

# Topics

# Companies

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Data`

# +-------------+------+
# | Column Name | Type |
# +-------------+------+
# | first\_col   | int  |
# | second\_col  | int  |
# +-------------+------+
# This table may contain duplicate rows.

# Write a solution to independently:

# *   order `first_col` in **ascending order**.
# *   order `second_col` in **descending order**.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Data table:
# +-----------+------------+
# | first\_col | second\_col |
# +-----------+------------+
# | 4         | 2          |
# | 2         | 3          |
# | 3         | 1          |
# | 1         | 4          |
# +-----------+------------+
# **Output:** 
# +-----------+------------+
# | first\_col | second\_col |
# +-----------+------------+
# | 1         | 4          |
# | 2         | 3          |
# | 3         | 2          |
# | 4         | 1          |
# +-----------+------------+



import pandas as pd

def order_two_columns(data: pd.DataFrame) -> pd.DataFrame:
    return pd.DataFrame({
        'first_col': data.first_col.sort_values().reset_index(drop=True),
        'second_col': data.second_col.sort_values(ascending=False).reset_index(drop=True),
    })