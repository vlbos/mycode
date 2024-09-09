# Medium

# Topics

# Companies

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Sales`

# +-------------+-------+
# | Column Name | Type  |
# +-------------+-------+
# | sale\_id     | int   |
# | product\_id  | int   |
# | user\_id     | int   |
# | quantity    | int   |
# +-------------+-------+
# sale\_id contains unique values.
# product\_id is a foreign key (reference column) to `Product` table.
# Each row of this table shows the ID of the product and the quantity purchased by a user.

# Table: `Product`

# +-------------+------+
# | Column Name | Type |
# +-------------+------+
# | product\_id  | int  |
# | price       | int  |
# +-------------+------+
# product\_id contains unique values.
# Each row of this table indicates the price of each product.

# Write a solution that reports for each user the product id on which the user spent the most money. In case the same user spent the most money on two or more products, report all of them.

# Return the resulting table in **any order**.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Sales table:
# +---------+------------+---------+----------+
# | sale\_id | product\_id | user\_id | quantity |
# +---------+------------+---------+----------+
# | 1       | 1          | 101     | 10       |
# | 2       | 3          | 101     | 7        |
# | 3       | 1          | 102     | 9        |
# | 4       | 2          | 102     | 6        |
# | 5       | 3          | 102     | 10       |
# | 6       | 1          | 102     | 6        |
# +---------+------------+---------+----------+
# Product table:
# +------------+-------+
# | product\_id | price |
# +------------+-------+
# | 1          | 10    |
# | 2          | 25    |
# | 3          | 15    |
# +------------+-------+
# **Output:** 
# +---------+------------+
# | user\_id | product\_id |
# +---------+------------+
# | 101     | 3          |
# | 102     | 1          |
# | 102     | 2          |
# | 102     | 3          |
# +---------+------------+ 
# **Explanation:** 
# User 101:
#     - Spent 10 \* 10 = 100 on product 1.
#     - Spent 7 \* 15 = 105 on product 3.
# User 101 spent the most money on product 3.
# User 102:
#     - Spent (9 + 6) \* 10 = 150 on product 1.
#     - Spent 6 \* 25 = 150 on product 2.
#     - Spent 10 \* 15 = 150 on product 3.
# User 102 spent the most money on products 1, 2, and 3.


import pandas as pd

def product_sales_analysis(sales: pd.DataFrame, product: pd.DataFrame) -> pd.DataFrame:
    df = pd.merge(sales,product,how ="inner",on="product_id")
    df["total"] = df["quantity"]*df["price"]
    df = df[["product_id","user_id","total"]]
    df2 = df.groupby(["user_id","product_id"])["total"].sum().reset_index()
    df2["ranking"] = df2.groupby(["user_id"])["total"].rank(method="dense",ascending = False)
    df3 = df2[df2["ranking"] == 1].reset_index()[["user_id","product_id"]]
    return df3 