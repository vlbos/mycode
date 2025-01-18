# Medium

# Topics

# Companies

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Orders`

# +---------------+------+
# | Column Name   | Type |
# +---------------+------+
# | order\_id      | int  |
# | product\_id    | int  |
# | quantity      | int  |
# | purchase\_date | date |
# +---------------+------+
# order\_id contains unique values.
# Each row in this table contains the ID of an order, the id of the product purchased, the quantity, and the purchase date.

# Write a solution to report the IDs of all the products that were ordered three or more times in two consecutive years.

# Return the result table in **any order**.

# The result format is shown in the following example.

# **Example 1:**

# **Input:** 
# Orders table:
# +----------+------------+----------+---------------+
# | order\_id | product\_id | quantity | purchase\_date |
# +----------+------------+----------+---------------+
# | 1        | 1          | 7        | 2020-03-16    |
# | 2        | 1          | 4        | 2020-12-02    |
# | 3        | 1          | 7        | 2020-05-10    |
# | 4        | 1          | 6        | 2021-12-23    |
# | 5        | 1          | 5        | 2021-05-21    |
# | 6        | 1          | 6        | 2021-10-11    |
# | 7        | 2          | 6        | 2022-10-11    |
# +----------+------------+----------+---------------+
# **Output:** 
# +------------+
# | product\_id |
# +------------+
# | 1          |
# +------------+
# **Explanation:** 
# Product 1 was ordered in 2020 three times and in 2021 three times. Since it was ordered three times in two consecutive years, we include it in the answer.
# Product 2 was ordered one time in 2022. We do not include it in the answer.



import pandas as pd

def find_valid_products(orders: pd.DataFrame) -> pd.DataFrame:
    df = orders.assign(year = orders["purchase_date"].dt.year)
    df = df.groupby(["product_id","year"],as_index=0).agg(cnt=("order_id","count"))
    df = df.query("cnt >= 3 & cnt.shift() >= 3 & product_id == product_id.shift() & year-1 == year.shift()")
    return df[["product_id"]].drop_duplicates()