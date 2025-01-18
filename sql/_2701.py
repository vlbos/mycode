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
# transaction\_id is the primary key of this table. 
# Each row contains information about transactions that includes unique (customer\_id, transaction\_date) along with the corresponding customer\_id and amount.  

# Write an SQL query to find the customers who have made consecutive transactions with increasing `amount` for at least three consecutive days. Include the `customer_id`, start date of the consecutive transactions period and the end date of the consecutive transactions period. There can be multiple consecutive transactions by a customer.

# Return _the result table ordered by_ `customer_id` _in **ascending** order._

# The query result format is in the following example.

# **Example 1:**

# **Input:** 
# Transactions table:
# +----------------+-------------+------------------+--------+
# | transaction\_id | customer\_id | transaction\_date | amount |
# +----------------+-------------+------------------+--------+
# | 1              | 101         | 2023-05-01       | 100    |
# | 2              | 101         | 2023-05-02       | 150    |
# | 3              | 101         | 2023-05-03       | 200    |
# | 4              | 102         | 2023-05-01       | 50     |
# | 5              | 102         | 2023-05-03       | 100    |
# | 6              | 102         | 2023-05-04       | 200    |
# | 7              | 105         | 2023-05-01       | 100    |
# | 8              | 105         | 2023-05-02       | 150    |
# | 9              | 105         | 2023-05-03       | 200    |
# | 10             | 105         | 2023-05-04       | 300    |
# | 11             | 105         | 2023-05-12       | 250    |
# | 12             | 105         | 2023-05-13       | 260    |
# | 13             | 105         | 2023-05-14       | 270    |
# +----------------+-------------+------------------+--------+
# **Output:** 
# +-------------+-------------------+-----------------+
# | customer\_id | consecutive\_start | consecutive\_end | 
# +-------------+-------------------+-----------------+
# | 101         |  2023-05-01       | 2023-05-03      | 
# | 105         |  2023-05-01       | 2023-05-04      |
# | 105         |  2023-05-12       | 2023-05-14      | 
# +-------------+-------------------+-----------------+
# **Explanation:** 
# - customer\_id 101 has made consecutive transactions with increasing amounts from May 1st, 2023, to May 3rd, 2023
# - customer\_id 102 does not have any consecutive transactions for at least 3 days. 
# - customer\_id 105 has two sets of consecutive transactions: from May 1st, 2023, to May 4th, 2023, and from May 12th, 2023, to May 14th, 2023. 
# customer\_id is sorted in ascending order.


import pandas as pd

def consecutive_increasing_transactions(transactions: pd.DataFrame) -> pd.DataFrame:
    df = transactions.sort_values(["customer_id","transaction_date"]).reset_index(drop=True).reset_index()
    df["group1"] = (df["transaction_date"]-pd.to_datetime("2023-01-01")).dt.days - df.index
    df["group2"] = (df.amount <= df.amount.shift(1)).cumsum().fillna(0)
    return (  df.groupby(["customer_id","group1","group2"])
                .agg(cnt=("index","count"),consecutive_start=("transaction_date","min"),consecutive_end=("transaction_date","max"))
                .reset_index().query("cnt>2").iloc[:,[0,4,5]]  )