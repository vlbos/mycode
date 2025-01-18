# Hard

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Transactions`

# +------------------+------+
# | Column Name      | Type |
# +------------------+------+
# | transaction\_id   | int  |
# | customer\_id      | int  |
# | transaction\_date | date |
# | amount           | int  |
# +------------------+------+
# transaction\_id is the column with unique values of this table.
# Each row contains information about transactions that includes unique (customer\_id, transaction\_date) along with the corresponding customer\_id and amount.   

# Write a solution to find all `customer_id` who made the maximum number of transactions on consecutive days.

# Return all `customer_id` with the maximum number of consecutive transactions. Order the result table by `customer_id` in **ascending** order.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Transactions table:
# +----------------+-------------+------------------+--------+
# | transaction\_id | customer\_id | transaction\_date | amount |
# +----------------+-------------+------------------+--------+
# | 1              | 101         | 2023-05-01       | 100    |
# | 2              | 101         | 2023-05-02       | 150    |
# | 3              | 101         | 2023-05-03       | 200    |
# | 4              | 102         | 2023-05-01       | 50     |
# | 5              | 102         | 2023-05-03       | 100    |
# | 6              | 102         | 2023-05-04       | 200    |
# | 7              | 105         | 2023-05-01       | 100    |
# | 8              | 105         | 2023-05-02       | 150    |
# | 9              | 105         | 2023-05-03       | 200    |
# +----------------+-------------+------------------+--------+
# **Output:** 
# +-------------+
# | customer\_id | 
# +-------------+
# | 101         | 
# | 105         | 
# +-------------+
# **Explanation:** 
# - customer\_id 101 has a total of 3 transactions, and all of them are consecutive.
# - customer\_id 102 has a total of 3 transactions, but only 2 of them are consecutive. 
# - customer\_id 105 has a total of 3 transactions, and all of them are consecutive.
# In total, the highest number of consecutive transactions is 3, achieved by customer\_id 101 and 105. The customer\_id are sorted in ascending order.




import pandas as pd

def find_customers(transactions: pd.DataFrame) -> pd.DataFrame:
    df = transactions.sort_values(by=['customer_id', 'transaction_date'])
    df['row_num'] = df.groupby('customer_id')['transaction_date'].rank()
    df['group_id'] = df['transaction_date'] - pd.to_timedelta(df['row_num'], unit='d')
    df = df.groupby(['group_id', 'customer_id'])['transaction_id'].nunique().reset_index().rename(columns={'transaction_id':'transaction_cnt'})
    max_trans_cnt = df['transaction_cnt'].max()
    df = df[df['transaction_cnt'] == max_trans_cnt][['customer_id']].sort_values(by='customer_id')

    return df