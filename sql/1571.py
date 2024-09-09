# Easy

# Topics

# Companies

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Warehouse`

# +--------------+---------+
# | Column Name  | Type    |
# +--------------+---------+
# | name         | varchar |
# | product\_id   | int     |
# | units        | int     |
# +--------------+---------+
# (name, product\_id) is the primary key (combination of columns with unique values) for this table.
# Each row of this table contains the information of the products in each warehouse.

# Table: `Products`

# +---------------+---------+
# | Column Name   | Type    |
# +---------------+---------+
# | product\_id    | int     |
# | product\_name  | varchar |
# | Width         | int     |
# | Length        | int     |
# | Height        | int     |
# +---------------+---------+
# product\_id is the primary key (column with unique values) for this table.
# Each row of this table contains information about the product dimensions (Width, Lenght, and Height) in feets of each product.

# Write a solution to report the number of cubic feet of **volume** the inventory occupies in each warehouse.

# Return the result table in **any order**.

# The query result format is in the following example.

# **Example 1:**

# **Input:** 
# Warehouse table:
# +------------+--------------+-------------+
# | name       | product\_id   | units       |
# +------------+--------------+-------------+
# | LCHouse1   | 1            | 1           |
# | LCHouse1   | 2            | 10          |
# | LCHouse1   | 3            | 5           |
# | LCHouse2   | 1            | 2           |
# | LCHouse2   | 2            | 2           |
# | LCHouse3   | 4            | 1           |
# +------------+--------------+-------------+
# Products table:
# +------------+--------------+------------+----------+-----------+
# | product\_id | product\_name | Width      | Length   | Height    |
# +------------+--------------+------------+----------+-----------+
# | 1          | LC-TV        | 5          | 50       | 40        |
# | 2          | LC-KeyChain  | 5          | 5        | 5         |
# | 3          | LC-Phone     | 2          | 10       | 10        |
# | 4          | LC-T-Shirt   | 4          | 10       | 20        |
# +------------+--------------+------------+----------+-----------+
# **Output:** 
# +----------------+------------+
# | warehouse\_name | volume     | 
# +----------------+------------+
# | LCHouse1       | 12250      | 
# | LCHouse2       | 20250      |
# | LCHouse3       | 800        |
# +----------------+------------+
# **Explanation:** 
# Volume of product\_id = 1 (LC-TV), 5x50x40 = 10000
# Volume of product\_id = 2 (LC-KeyChain), 5x5x5 = 125 
# Volume of product\_id = 3 (LC-Phone), 2x10x10 = 200
# Volume of product\_id = 4 (LC-T-Shirt), 4x10x20 = 800
# LCHouse1: 1 unit of LC-TV + 10 units of LC-KeyChain + 5 units of LC-Phone.
#           Total volume: 1\*10000 + 10\*125  + 5\*200 = 12250 cubic feet
# LCHouse2: 2 units of LC-TV + 2 units of LC-KeyChain.
#           Total volume: 2\*10000 + 2\*125 = 20250 cubic feet
# LCHouse3: 1 unit of LC-T-Shirt.
#           Total volume: 1\*800 = 800 cubic feet.




import pandas as pd

def warehouse_manager(warehouse: pd.DataFrame, products: pd.DataFrame) -> pd.DataFrame:
    return (
        warehouse.merge(products)
        .assign(
            volume=lambda df: df["units"] * df["Width"] * df["Length"] * df["Height"]
        )
        .groupby("name", as_index=False)
        .agg({"volume": "sum"})
        .rename(columns={"name": "warehouse_name"})
    )