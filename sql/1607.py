# Easy

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Customer`

# +---------------+---------+
# | Column Name   | Type    |
# +---------------+---------+
# | customer\_id   | int     |
# | customer\_name | varchar |
# +---------------+---------+
# customer\_id is the column with unique values for this table.
# Each row of this table contains the information of each customer in the WebStore.

# Table: `Orders`

# +---------------+---------+
# | Column Name   | Type    |
# +---------------+---------+
# | order\_id      | int     |
# | sale\_date     | date    |
# | order\_cost    | int     |
# | customer\_id   | int     |
# | seller\_id     | int     |
# +---------------+---------+
# order\_id is the column with unique values for this table.
# Each row of this table contains all orders made in the webstore.
# sale\_date is the date when the transaction was made between the customer (customer\_id) and the seller (seller\_id).

# Table: `Seller`

# +---------------+---------+
# | Column Name   | Type    |
# +---------------+---------+
# | seller\_id     | int     |
# | seller\_name   | varchar |
# +---------------+---------+
# seller\_id is the column with unique values for this table.
# Each row of this table contains the information of each seller.

# Write a solution to report the names of all sellers who did not make any sales in `2020`.

# Return the result table ordered by `seller_name` in **ascending order**.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Customer table:
# +--------------+---------------+
# | customer\_id  | customer\_name |
# +--------------+---------------+
# | 101          | Alice         |
# | 102          | Bob           |
# | 103          | Charlie       |
# +--------------+---------------+
# Orders table:
# +-------------+------------+--------------+-------------+-------------+
# | order\_id    | sale\_date  | order\_cost   | customer\_id | seller\_id   |
# +-------------+------------+--------------+-------------+-------------+
# | 1           | 2020-03-01 | 1500         | 101         | 1           |
# | 2           | 2020-05-25 | 2400         | 102         | 2           |
# | 3           | 2019-05-25 | 800          | 101         | 3           |
# | 4           | 2020-09-13 | 1000         | 103         | 2           |
# | 5           | 2019-02-11 | 700          | 101         | 2           |
# +-------------+------------+--------------+-------------+-------------+
# Seller table:
# +-------------+-------------+
# | seller\_id   | seller\_name |
# +-------------+-------------+
# | 1           | Daniel      |
# | 2           | Elizabeth   |
# | 3           | Frank       |
# +-------------+-------------+
# **Output:** 
# +-------------+
# | seller\_name |
# +-------------+
# | Frank       |
# +-------------+
# **Explanation:** 
# Daniel made 1 sale in March 2020.
# Elizabeth made 2 sales in 2020 and 1 sale in 2019.
# Frank made 1 sale in 2019 but no sales in 2020.



import pandas as pd

def sellers_with_no_sales(customer: pd.DataFrame, orders: pd.DataFrame, seller: pd.DataFrame) -> pd.DataFrame:
    return seller[
        ~seller["seller_id"].isin(
            orders[orders["sale_date"].dt.year == 2020]["seller_id"]
        )
    ][["seller_name"]].sort_values("seller_name")