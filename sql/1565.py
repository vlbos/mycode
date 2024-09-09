# Easy

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Orders`

# +---------------+---------+
# | Column Name   | Type    |
# +---------------+---------+
# | order\_id      | int     |
# | order\_date    | date    |
# | customer\_id   | int     |
# | invoice       | int     |
# +---------------+---------+
# order\_id is the column with unique values for this table.
# This table contains information about the orders made by customer\_id.

# Write a solution to find the number of **unique orders** and the number of **unique customers** with invoices **\> $20** for each **different month**.

# Return the result table sorted in **any order**.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Orders table:
# +----------+------------+-------------+------------+
# | order\_id | order\_date | customer\_id | invoice    |
# +----------+------------+-------------+------------+
# | 1        | 2020-09-15 | 1           | 30         |
# | 2        | 2020-09-17 | 2           | 90         |
# | 3        | 2020-10-06 | 3           | 20         |
# | 4        | 2020-10-20 | 3           | 21         |
# | 5        | 2020-11-10 | 1           | 10         |
# | 6        | 2020-11-21 | 2           | 15         |
# | 7        | 2020-12-01 | 4           | 55         |
# | 8        | 2020-12-03 | 4           | 77         |
# | 9        | 2021-01-07 | 3           | 31         |
# | 10       | 2021-01-15 | 2           | 20         |
# +----------+------------+-------------+------------+
# **Output:** 
# +---------+-------------+----------------+
# | month   | order\_count | customer\_count |
# +---------+-------------+----------------+
# | 2020-09 | 2           | 2              |
# | 2020-10 | 1           | 1              |
# | 2020-12 | 2           | 1              |
# | 2021-01 | 1           | 1              |
# +---------+-------------+----------------+
# **Explanation:** 
# In September 2020 we have two orders from 2 different customers with invoices > $20.
# In October 2020 we have two orders from 1 customer, and only one of the two orders has invoice > $20.
# In November 2020 we have two orders from 2 different customers but invoices < $20, so we don't include that month.
# In December 2020 we have two orders from 1 customer both with invoices > $20.
# In January 2021 we have two orders from 2 different customers, but only one of them with invoice > $20.



import pandas as pd

def unique_orders_and_customers(orders: pd.DataFrame) -> pd.DataFrame:
    return (
        orders[orders["invoice"] > 20]
        .assign(month=orders["order_date"].dt.strftime("%Y-%m"))
        .groupby("month", as_index=False)
        .agg(
            order_count=("order_id", "nunique"),
            customer_count=("customer_id", "nunique"),
        )
    )