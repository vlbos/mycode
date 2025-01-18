# Medium

# Topics

# Companies

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Transactions`

# +----------------+---------+
# | Column Name    | Type    |
# +----------------+---------+
# | id             | int     |
# | country        | varchar |
# | state          | enum    |
# | amount         | int     |
# | trans\_date     | date    |
# +----------------+---------+
# id is the column of unique values of this table.
# The table has information about incoming transactions.
# The state column is an ENUM (category) of type \["approved", "declined"\].

# Table: `Chargebacks`

# +----------------+---------+
# | Column Name    | Type    |
# +----------------+---------+
# | trans\_id       | int     |
# | trans\_date     | date    |
# +----------------+---------+
# Chargebacks contains basic information regarding incoming chargebacks from some transactions placed in Transactions table.
# trans\_id is a foreign key (reference column) to the id column of Transactions table.
# Each chargeback corresponds to a transaction made previously even if they were not approved.

# Write a solution to find for each month and country: the number of approved transactions and their total amount, the number of chargebacks, and their total amount.

# **Note**: In your solution, given the month and country, ignore rows with all zeros.

# Return the result table in **any order**.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Transactions table:
# +-----+---------+----------+--------+------------+
# | id  | country | state    | amount | trans\_date |
# +-----+---------+----------+--------+------------+
# | 101 | US      | approved | 1000   | 2019-05-18 |
# | 102 | US      | declined | 2000   | 2019-05-19 |
# | 103 | US      | approved | 3000   | 2019-06-10 |
# | 104 | US      | declined | 4000   | 2019-06-13 |
# | 105 | US      | approved | 5000   | 2019-06-15 |
# +-----+---------+----------+--------+------------+
# Chargebacks table:
# +----------+------------+
# | trans\_id | trans\_date |
# +----------+------------+
# | 102      | 2019-05-29 |
# | 101      | 2019-06-30 |
# | 105      | 2019-09-18 |
# +----------+------------+
# **Output:** 
# +---------+---------+----------------+-----------------+------------------+-------------------+
# | month   | country | approved\_count | approved\_amount | chargeback\_count | chargeback\_amount |
# +---------+---------+----------------+-----------------+------------------+-------------------+
# | 2019-05 | US      | 1              | 1000            | 1                | 2000              |
# | 2019-06 | US      | 2              | 8000            | 1                | 1000              |
# | 2019-09 | US      | 0              | 0               | 1                | 5000              |
# +---------+---------+----------------+-----------------+------------------+-------------------+


import pandas as pd

def monthly_transactions(transactions: pd.DataFrame, chargebacks: pd.DataFrame) -> pd.DataFrame:
    result = transactions.merge(chargebacks, how = 'left', left_on = 'id', right_on = 'trans_id')
    result['trans_month'] = result['trans_date_x'].dt.strftime('%Y-%m')
    result['charge_month'] = result['trans_date_y'].dt.strftime('%Y-%m')

    result1 = result[result['state']=='approved'].groupby(['trans_month','country'], as_index = False).agg(
        approved_count = ('id', 'count'),
        approved_amount = ('amount', 'sum')
    ).rename(columns = {'trans_month':'month'})
    
    result2 = result.groupby(['charge_month','country'], as_index = False).agg(
        chargeback_count = ('id', 'count'),
        chargeback_amount = ('amount', 'sum')
    ).rename(columns = {'charge_month':'month'})

    result3 = result1.merge(result2, how = 'outer', on =['month','country']).fillna(0)
 
    return (result3)