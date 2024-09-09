# Easy

# Topics

# Companies

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Product`

# +--------------+---------+
# | Column Name  | Type    |
# +--------------+---------+
# | product\_id   | int     |
# | product\_name | varchar |
# | unit\_price   | int     |
# +--------------+---------+
# product\_id is the primary key (column with unique values) of this table.
# Each row of this table indicates the name and the price of each product.

# Table: `Sales`

# +-------------+---------+
# | Column Name | Type    |
# +-------------+---------+
# | seller\_id   | int     |
# | product\_id  | int     |
# | buyer\_id    | int     |
# | sale\_date   | date    |
# | quantity    | int     |
# | price       | int     |
# +-------------+---------+
# This table can have repeated rows.
# product\_id is a foreign key (reference column) to the Product table.
# Each row of this table contains some information about one sale.

# Write a solution that reports the best **seller** by total sales price, If there is a tie, report them all.

# Return the result table in **any order**.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Product table:
# +------------+--------------+------------+
# | product\_id | product\_name | unit\_price |
# +------------+--------------+------------+
# | 1          | S8           | 1000       |
# | 2          | G4           | 800        |
# | 3          | iPhone       | 1400       |
# +------------+--------------+------------+
# Sales table:
# +-----------+------------+----------+------------+----------+-------+
# | seller\_id | product\_id | buyer\_id | sale\_date  | quantity | price |
# +-----------+------------+----------+------------+----------+-------+
# | 1         | 1          | 1        | 2019-01-21 | 2        | 2000  |
# | 1         | 2          | 2        | 2019-02-17 | 1        | 800   |
# | 2         | 2          | 3        | 2019-06-02 | 1        | 800   |
# | 3         | 3          | 4        | 2019-05-13 | 2        | 2800  |
# +-----------+------------+----------+------------+----------+-------+
# **Output:** 
# +-------------+
# | seller\_id   |
# +-------------+
# | 1           |
# | 3           |
# +-------------+
# **Explanation:** Both sellers with id 1 and 3 sold products with the most total price of 2800.

import pandas as pd

def sales_analysis(product: pd.DataFrame, sales: pd.DataFrame) -> pd.DataFrame:
    sales = sales.groupby(["seller_id"])["price"].sum().reset_index()
    sales = sales[["seller_id","price"]]
    sales["ranking"]= sales["price"].rank(method = "dense",ascending = False)
    return sales[sales["ranking"] == 1][["seller_id"]]