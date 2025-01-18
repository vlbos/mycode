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
# This table might have repeated rows.
# product\_id is a foreign key (reference column) to the Product table.
# buyer\_id is never NULL. 
# sale\_date is never NULL. 
# Each row of this table contains some information about one sale.

# Write a solution to report the **buyers** who have bought _S8_ but not _iPhone_. Note that _S8_ and _iPhone_ are products presented in the `Product` table.

# Return the result table in **any order**.

# The result format is in the following example.

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
# | 2         | 1          | 3        | 2019-06-02 | 1        | 800   |
# | 3         | 3          | 3        | 2019-05-13 | 2        | 2800  |
# +-----------+------------+----------+------------+----------+-------+
# **Output:** 
# +-------------+
# | buyer\_id    |
# +-------------+
# | 1           |
# +-------------+
# **Explanation:** The buyer with id 1 bought an S8 but did not buy an iPhone. The buyer with id 3 bought both.



import pandas as pd

def sales_analysis(product: pd.DataFrame, sales: pd.DataFrame) -> pd.DataFrame:
  df = pd.merge(product, sales, on='product_id', how='inner')
  iphone_buyer = df[df['product_name']=='iPhone']['buyer_id'].drop_duplicates()
  s8_buyer = df[df['product_name']=='S8'][['buyer_id']].drop_duplicates()
  return s8_buyer[~s8_buyer['buyer_id'].isin(iphone_buyer)]