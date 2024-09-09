# Medium

# Topics

# Companies

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Users`

# +--------------+---------+
# | Column Name  | Type    |
# +--------------+---------+
# | user\_id      | int     |
# | user\_name    | varchar |
# | credit       | int     |
# +--------------+---------+
# user\_id is the primary key (column with unique values) for this table.
# Each row of this table contains the current credit information for each user.

# Table: `Transactions`

# +---------------+---------+
# | Column Name   | Type    |
# +---------------+---------+
# | trans\_id      | int     |
# | paid\_by       | int     |
# | paid\_to       | int     |
# | amount        | int     |
# | transacted\_on | date    |
# +---------------+---------+
# trans\_id is the primary key (column with unique values) for this table.
# Each row of this table contains information about the transaction in the bank.
# User with id (paid\_by) transfer money to user with id (paid\_to).

# Leetcode Bank (LCB) helps its coders in making virtual payments. Our bank records all transactions in the table _Transaction_, we want to find out the current balance of all users and check whether they have breached their credit limit (If their current credit is less than `0`).

# Write a solution to report.

# *   `user_id`,
# *   `user_name`,
# *   `credit`, current balance after performing transactions, and
# *   `credit_limit_breached`, check credit\_limit (`"Yes"` or `"No"`)

# Return the result table in **any** order.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Users table:
# +------------+--------------+-------------+
# | user\_id    | user\_name    | credit      |
# +------------+--------------+-------------+
# | 1          | Moustafa     | 100         |
# | 2          | Jonathan     | 200         |
# | 3          | Winston      | 10000       |
# | 4          | Luis         | 800         | 
# +------------+--------------+-------------+
# Transactions table:
# +------------+------------+------------+----------+---------------+
# | trans\_id   | paid\_by    | paid\_to    | amount   | transacted\_on |
# +------------+------------+------------+----------+---------------+
# | 1          | 1          | 3          | 400      | 2020-08-01    |
# | 2          | 3          | 2          | 500      | 2020-08-02    |
# | 3          | 2          | 1          | 200      | 2020-08-03    |
# +------------+------------+------------+----------+---------------+
# **Output:** 
# +------------+------------+------------+-----------------------+
# | user\_id    | user\_name  | credit     | credit\_limit\_breached |
# +------------+------------+------------+-----------------------+
# | 1          | Moustafa   | -100       | Yes                   | 
# | 2          | Jonathan   | 500        | No                    |
# | 3          | Winston    | 9900       | No                    |
# | 4          | Luis       | 800        | No                    |
# +------------+------------+------------+-----------------------+
# **Explanation:** 
# Moustafa paid $400 on "2020-08-01" and received $200 on "2020-08-03", credit (100 -400 +200) = -$100
# Jonathan received $500 on "2020-08-02" and paid $200 on "2020-08-08", credit (200 +500 -200) = $500
# Winston received $400 on "2020-08-01" and paid $500 on "2020-08-03", credit (10000 +400 -500) = $9990
# Luis did not received any transfer, credit = $800



import pandas as pd

def bank_account_summary(users: pd.DataFrame, transactions: pd.DataFrame) -> pd.DataFrame:
    loss = transactions.groupby('paid_by')['amount'].sum().reset_index(name='loss')
    gain = transactions.groupby('paid_to')['amount'].sum().reset_index(name='gain')
    users = users.merge(loss, left_on='user_id', how='left', right_on='paid_by').merge(gain, left_on='user_id', how='left', right_on='paid_to').fillna(0)
    users['credit'] = users['credit'] - users['loss'] + users['gain']
    users['credit_limit_breached'] = np.where(users.credit >= 0, 'No', 'Yes')
    return users[['user_id', 'user_name', 'credit', 'credit_limit_breached']]