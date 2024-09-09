# Easy

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Product`

# +-------------+---------+
# | Column Name | Type    |
# +-------------+---------+
# | product\_id  | int     |
# | name        | varchar |
# +-------------+---------+
# product\_id is the column with unique values for this table.
# This table contains the ID and the name of the product. The name consists of only lowercase English letters. No two products have the same name.

# Table: `Invoice`

# +-------------+------+
# | Column Name | Type |
# +-------------+------+
# | invoice\_id  | int  |
# | product\_id  | int  |
# | rest        | int  |
# | paid        | int  |
# | canceled    | int  |
# | refunded    | int  |
# +-------------+------+
# invoice\_id is the column with unique values for this table and the id of this invoice.
# product\_id is the id of the product for this invoice.
# rest is the amount left to pay for this invoice.
# paid is the amount paid for this invoice.
# canceled is the amount canceled for this invoice.
# refunded is the amount refunded for this invoice.

# Write a solution that will, for all products, return each product name with the total amount due, paid, canceled, and refunded across all invoices.

# Return the result table ordered by `product_name`.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Product table:
# +------------+-------+
# | product\_id | name  |
# +------------+-------+
# | 0          | ham   |
# | 1          | bacon |
# +------------+-------+
# Invoice table:
# +------------+------------+------+------+----------+----------+
# | invoice\_id | product\_id | rest | paid | canceled | refunded |
# +------------+------------+------+------+----------+----------+
# | 23         | 0          | 2    | 0    | 5        | 0        |
# | 12         | 0          | 0    | 4    | 0        | 3        |
# | 1          | 1          | 1    | 1    | 0        | 1        |
# | 2          | 1          | 1    | 0    | 1        | 1        |
# | 3          | 1          | 0    | 1    | 1        | 1        |
# | 4          | 1          | 1    | 1    | 1        | 0        |
# +------------+------------+------+------+----------+----------+
# **Output:** 
# +-------+------+------+----------+----------+
# | name  | rest | paid | canceled | refunded |
# +-------+------+------+----------+----------+
# | bacon | 3    | 3    | 3        | 3        |
# | ham   | 2    | 4    | 5        | 3        |
# +-------+------+------+----------+----------+
# **Explanation:** 
# - The amount of money left to pay for bacon is 1 + 1 + 0 + 1 = 3
# - The amount of money paid for bacon is 1 + 0 + 1 + 1 = 3
# - The amount of money canceled for bacon is 0 + 1 + 1 + 1 = 3
# - The amount of money refunded for bacon is 1 + 1 + 1 + 0 = 3
# - The amount of money left to pay for ham is 2 + 0 = 2
# - The amount of money paid for ham is 0 + 4 = 4
# - The amount of money canceled for ham is 5 + 0 = 5
# - The amount of money refunded for ham is 0 + 3 = 3


import pandas as pd

def analyze_products(product: pd.DataFrame, invoice: pd.DataFrame) -> pd.DataFrame:
    return (
        product.merge(invoice, how="left")
        .groupby("name", as_index=False)
        .sum()
        .drop(columns=["product_id", "invoice_id"])
        .sort_values("name")
    )