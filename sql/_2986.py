# Medium

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Transactions`

# +------------------+----------+
# | Column Name      | Type     |
# +------------------+----------+
# | user\_id          | int      |
# | spend            | decimal  |
# | transaction\_date | datetime |
# +------------------+----------+
# (user\_id, transaction\_date) is column of unique values for this table.
# This table contains user\_id, spend, and transaction\_date.

# Write a solution to find the **third transaction** (if they have at least three transactions) of every user, where the **spending** on the preceding **two transactions** is **lower** than the spending on the **third** transaction.

# Return _the result table by_ `user_id` _in **ascending** order__._

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Transactions table:
# +---------+--------+---------------------+
# | user\_id | spend  | transaction\_date    | 
# +---------+--------+---------------------+
# | 1       | 65.56  | 2023-11-18 13:49:42 | 
# | 1       | 96.0   | 2023-11-30 02:47:26 |     
# | 1       | 7.44   | 2023-11-02 12:15:23 | 
# | 1       | 49.78  | 2023-11-12 00:13:46 | 
# | 2       | 40.89  | 2023-11-21 04:39:15 |  
# | 2       | 100.44 | 2023-11-20 07:39:34 | 
# | 3       | 37.33  | 2023-11-03 06:22:02 | 
# | 3       | 13.89  | 2023-11-11 16:00:14 | 
# | 3       | 7.0    | 2023-11-29 22:32:36 | 
# +---------+--------+---------------------+
# **Output**
# +---------+-------------------------+------------------------+
# | user\_id | third\_transaction\_spend | third\_transaction\_date | 
# +---------+-------------------------+------------------------+
# | 1       | 65.56                   | 2023-11-18 13:49:42    |  
# +---------+-------------------------+------------------------+
# **Explanation**
# - For user\_id 1, their third transaction occurred on 2023-11-18 at 13:49:42 with an amount of $65.56, surpassing the expenditures of the previous two transactions which were $7.44 on 2023-11-02 at 12:15:23 and $49.78 on 2023-11-12 at 00:13:46. Thus, this third transaction will be included in the output table.
# - user\_id 2 only has a total of 2 transactions, so there isn't a third transaction to consider.
# - For user\_id 3, the amount of $7.0 for their third transaction is less than that of the preceding two transactions, so it won't be included.
# Output table is ordered by user\_id in ascending order.




import pandas as pd

def find_third_transaction(transactions: pd.DataFrame) -> pd.DataFrame:
    
    df = transactions.sort_values(['user_id', 'transaction_date'])
    df = df.groupby('user_id').head(3)
    df = df[(df['spend'] > df.shift(1)['spend']) & (df['user_id'] == df.shift(1)['user_id']) &
            (df['spend'] > df.shift(2)['spend']) & (df['user_id'] == df.shift(2)['user_id'])]

    return df.rename(columns={'spend': 'third_transaction_spend', 'transaction_date': 'third_transaction_date'})[['user_id', 'third_transaction_spend', 'third_transaction_date']].sort_values('user_id')

