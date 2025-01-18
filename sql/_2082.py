# Easy

# Topics

# Companies

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Store`

# +-------------+------+
# | Column Name | Type |
# +-------------+------+
# | bill\_id     | int  |
# | customer\_id | int  |
# | amount      | int  |
# +-------------+------+
# bill\_id is the primary key (column with unique values) for this table.
# Each row contains information about the amount of one bill and the customer associated with it.

# Write a solution to report the number of customers who had **at least one** bill with an amount **strictly greater** than `500`.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Store table:
# +---------+-------------+--------+
# | bill\_id | customer\_id | amount |
# +---------+-------------+--------+
# | 6       | 1           | 549    |
# | 8       | 1           | 834    |
# | 4       | 2           | 394    |
# | 11      | 3           | 657    |
# | 13      | 3           | 257    |
# +---------+-------------+--------+
# **Output:** 
# +------------+
# | rich\_count |
# +------------+
# | 2          |
# +------------+
# **Explanation:** 
# Customer 1 has two bills with amounts strictly greater than 500.
# Customer 2 does not have any bills with an amount strictly greater than 500.
# Customer 3 has one bill with an amount strictly greater than 500.


import pandas as pd

def count_rich_customers(store: pd.DataFrame) -> pd.DataFrame:
     return pd.DataFrame(
        store[store['amount'] > 500][['customer_id']].nunique(),
        columns=['rich_count'],
    )