# #Medium

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Salesperson`

# +----------------+---------+
# | Column Name    | Type    |
# +----------------+---------+
# | salesperson\_id | int     |
# | name           | varchar |
# +----------------+---------+
# salesperson\_id contains unique values.
# Each row in this table shows the ID of a salesperson.

# Table: `Customer`

# +----------------+------+
# | Column Name    | Type |
# +----------------+------+
# | customer\_id    | int  |
# | salesperson\_id | int  |
# +----------------+------+
# customer\_id contains unique values.
# salesperson\_id is a foreign key (reference column) from the Salesperson table.
# Each row in this table shows the ID of a customer and the ID of the salesperson. 

# Table: `Sales`

# +-------------+------+
# | Column Name | Type |
# +-------------+------+
# | sale\_id     | int  |
# | customer\_id | int  |
# | price       | int  |
# +-------------+------+
# sale\_id contains unique values.
# customer\_id is a foreign key (reference column) from the Customer table.
# Each row in this table shows ID of a customer and the price they paid for the sale with sale\_id.

# Write a solution to report the sum of prices paid by the customers of each salesperson. If a salesperson does not have any customers, the total value should be `0`.

# Return the result table in **any order**.

# The result format is shown in the following example.

# **Example 1:**

# **Input:** 
# Salesperson table:
# +----------------+-------+
# | salesperson\_id | name  |
# +----------------+-------+
# | 1              | Alice |
# | 2              | Bob   |
# | 3              | Jerry |
# +----------------+-------+
# Customer table:
# +-------------+----------------+
# | customer\_id | salesperson\_id |
# +-------------+----------------+
# | 1           | 1              |
# | 2           | 1              |
# | 3           | 2              |
# +-------------+----------------+
# Sales table:
# +---------+-------------+-------+
# | sale\_id | customer\_id | price |
# +---------+-------------+-------+
# | 1       | 2           | 892   |
# | 2       | 1           | 354   |
# | 3       | 3           | 988   |
# | 4       | 3           | 856   |
# +---------+-------------+-------+
# **Output:** 
# +----------------+-------+-------+
# | salesperson\_id | name  | total |
# +----------------+-------+-------+
# | 1              | Alice | 1246  |
# | 2              | Bob   | 1844  |
# | 3              | Jerry | 0     |
# +----------------+-------+-------+
# **Explanation:** 
# Alice is the salesperson for customers 1 and 2.
#   - Customer 1 made one purchase with 354.
#   - Customer 2 made one purchase with 892.
# The total for Alice is 354 + 892 = 1246.

# Bob is the salesperson for customers 3.
#   - Customer 1 made one purchase with 988 and 856.
# The total for Bob is 988 + 856 = 1844.

# Jerry is not the salesperson of any customer.
# The total for Jerry is 0.


import pandas as pd

def calculate_influence(salesperson: pd.DataFrame, customer: pd.DataFrame, sales: pd.DataFrame) -> pd.DataFrame:
    return salesperson.merge(customer, on='salesperson_id', how='left').merge(sales, on='customer_id', how='left').groupby(['salesperson_id','name']).agg(total=('price', 'sum')).reset_index()#.rename(columns={'price':'total'})