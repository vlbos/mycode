# Easy

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Orders`

# +-------------------+------+
# | Column Name       | Type |
# +-------------------+------+
# | order\_id          | int  |
# | item\_count        | int  |
# | order\_occurrences | int  |
# +-------------------+------+
# order\_id is column of unique values for this table.
# This table contains order\_id, item\_count, and order\_occurrences.

# Write a solution to calculate the **average** number of items per order, rounded to `2` **decimal places**.

# Return _the result table_ _in **any** order__._

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Orders table:
# +----------+------------+-------------------+
# | order\_id | item\_count | order\_occurrences | 
# +----------+------------+-------------------+
# | 10       | 1          | 500               | 
# | 11       | 2          | 1000              |     
# | 12       | 3          | 800               |  
# | 13       | 4          | 1000              | 
# +----------+------------+-------------------+
# **Output**
# +-------------------------+
# | average\_items\_per\_order | 
# +-------------------------+
# | 2.70                    |
# +-------------------------+
# **Explanation**
# The calculation is as follows:
#  - Total items: (1 \* 500) + (2 \* 1000) + (3 \* 800) + (4 \* 1000) = 8900 
#  - Total orders: 500 + 1000 + 800 + 1000 = 3300 
#  - Therefore, the average items per order is 8900 / 3300 = 2.70



import pandas as pd

def compressed_mean(orders: pd.DataFrame) -> pd.DataFrame:
   return pd.DataFrame({'average_items_per_order': [round(sum(orders['item_count'] * orders['order_occurrences'])/sum(orders['order_occurrences']),2)]})