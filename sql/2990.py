# Easy

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Loans`

# +-------------+---------+
# | Column Name | Type    |
# +-------------+---------+
# | loan\_id     | int     |
# | user\_id     | int     |
# | loan\_type   | varchar |
# +-------------+---------+
# loan\_id is column of unique values for this table.
# This table contains loan\_id, user\_id, and loan\_type.

# Write a solution to find all **distinct** `user_id`'s that have **at least one** **Refinance** loan type and at least one **Mortgage** loan type.

# Return _the result table ordered by_ `user_id` _in **ascending** order__._

# The result format is in the following example.

# **Example 1:**

# **Input:**
# Loans table:
# +---------+---------+-----------+
# | loan\_id | user\_id | loan\_type |
# +---------+---------+-----------+
# | 683     | 101     | Mortgage  |
# | 218     | 101     | AutoLoan  |
# | 802     | 101     | Inschool  |
# | 593     | 102     | Mortgage  |
# | 138     | 102     | Refinance |
# | 294     | 102     | Inschool  |
# | 308     | 103     | Refinance |
# | 389     | 104     | Mortgage  |
# +---------+---------+-----------+
# **Output**
# +---------+
# | user\_id | 
# +---------+
# | 102     | 
# +---------+
# **Explanation**
# - User\_id 101 has three loan types, one of which is a Mortgage. However, this user does not have any loan type categorized as Refinance, so user\_id 101 won't be considered.
# - User\_id 102 possesses three loan types: one for Mortgage and one for Refinance. Hence, user\_id 102 will be included in the result.
# - User\_id 103 has a loan type of Refinance but lacks a Mortgage loan type, so user\_id 103 won't be considered.
# - User\_id 104 has a Mortgage loan type but doesn't have a Refinance loan type, thus, user\_id 104 won't be considered.
# Output table is ordered by user\_id in ascending order.


import pandas as pd

def loan_types(loans: pd.DataFrame) -> pd.DataFrame:
    return loans.query("loan_type=='Refinance'")[['user_id']].merge(loans.query("loan_type=='Mortgage'")[['user_id']]).drop_duplicates().sort_values(by='user_id')