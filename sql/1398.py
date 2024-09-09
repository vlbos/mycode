# Medium

# Topics

# Companies

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Customers`

# +---------------------+---------+
# | Column Name         | Type    |
# +---------------------+---------+
# | customer\_id         | int     |
# | customer\_name       | varchar |
# +---------------------+---------+
# customer\_id is the column with unique values for this table.
# customer\_name is the name of the customer.

# Table: `Orders`

# +---------------+---------+
# | Column Name   | Type    |
# +---------------+---------+
# | order\_id      | int     |
# | customer\_id   | int     |
# | product\_name  | varchar |
# +---------------+---------+
# order\_id is the column with unique values for this table.
# customer\_id is the id of the customer who bought the product "product\_name".

# Write a solution to report the customer\_id and customer\_name of customers who bought products **"A"**, **"B"** but did not buy the product **"C"** since we want to recommend them to purchase this product.

# Return the result table **ordered** by `customer_id`.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Customers table:
# +-------------+---------------+
# | customer\_id | customer\_name |
# +-------------+---------------+
# | 1           | Daniel        |
# | 2           | Diana         |
# | 3           | Elizabeth     |
# | 4           | Jhon          |
# +-------------+---------------+
# Orders table:
# +------------+--------------+---------------+
# | order\_id   | customer\_id  | product\_name  |
# +------------+--------------+---------------+
# | 10         |     1        |     A         |
# | 20         |     1        |     B         |
# | 30         |     1        |     D         |
# | 40         |     1        |     C         |
# | 50         |     2        |     A         |
# | 60         |     3        |     A         |
# | 70         |     3        |     B         |
# | 80         |     3        |     D         |
# | 90         |     4        |     C         |
# +------------+--------------+---------------+
# **Output:** 
# +-------------+---------------+
# | customer\_id | customer\_name |
# +-------------+---------------+
# | 3           | Elizabeth     |
# +-------------+---------------+
# **Explanation:** Only the customer\_id with id 3 bought the product A and B but not the product C.



import pandas as pd

def find_customers(customers: pd.DataFrame, orders: pd.DataFrame) -> pd.DataFrame:
    grouped = (orders
               .merge(right=customers, on="customer_id", how="left")
               .groupby(by=["customer_id", "customer_name"], as_index=False)
               .agg(product_set=("product_name", lambda x: set(x))))
    return (grouped
            .loc[
            ~(grouped["product_set"] >= set("C")) &
            (grouped["product_set"] >= set(["A", "B"])),
            ["customer_id", "customer_name"]
        ].sort_values(by="customer_id", ascending=True)
    )