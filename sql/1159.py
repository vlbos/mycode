# Hard

# Topics

# Companies

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Users`

# +----------------+---------+
# | Column Name    | Type    |
# +----------------+---------+
# | user\_id        | int     |
# | join\_date      | date    |
# | favorite\_brand | varchar |
# +----------------+---------+
# user\_id is the primary key (column with unique values) of this table.
# This table has the info of the users of an online shopping website where users can sell and buy items.

# Table: `Orders`

# +---------------+---------+
# | Column Name   | Type    |
# +---------------+---------+
# | order\_id      | int     |
# | order\_date    | date    |
# | item\_id       | int     |
# | buyer\_id      | int     |
# | seller\_id     | int     |
# +---------------+---------+
# order\_id is the primary key (column with unique values) of this table.
# item\_id is a foreign key (reference column) to the Items table.
# buyer\_id and seller\_id are foreign keys to the Users table.

# Table: `Items`

# +---------------+---------+
# | Column Name   | Type    |
# +---------------+---------+
# | item\_id       | int     |
# | item\_brand    | varchar |
# +---------------+---------+
# item\_id is the primary key (column with unique values) of this table.

# Write a solution to find for each user whether the brand of the second item (by date) they sold is their favorite brand. If a user sold less than two items, report the answer for that user as no. It is guaranteed that no seller sells more than one item in a day.

# Return the result table in **any order**.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Users table:
# +---------+------------+----------------+
# | user\_id | join\_date  | favorite\_brand |
# +---------+------------+----------------+
# | 1       | 2019-01-01 | Lenovo         |
# | 2       | 2019-02-09 | Samsung        |
# | 3       | 2019-01-19 | LG             |
# | 4       | 2019-05-21 | HP             |
# +---------+------------+----------------+
# Orders table:
# +----------+------------+---------+----------+-----------+
# | order\_id | order\_date | item\_id | buyer\_id | seller\_id |
# +----------+------------+---------+----------+-----------+
# | 1        | 2019-08-01 | 4       | 1        | 2         |
# | 2        | 2019-08-02 | 2       | 1        | 3         |
# | 3        | 2019-08-03 | 3       | 2        | 3         |
# | 4        | 2019-08-04 | 1       | 4        | 2         |
# | 5        | 2019-08-04 | 1       | 3        | 4         |
# | 6        | 2019-08-05 | 2       | 2        | 4         |
# +----------+------------+---------+----------+-----------+
# Items table:
# +---------+------------+
# | item\_id | item\_brand |
# +---------+------------+
# | 1       | Samsung    |
# | 2       | Lenovo     |
# | 3       | LG         |
# | 4       | HP         |
# +---------+------------+
# **Output:** 
# +-----------+--------------------+
# | seller\_id | 2nd\_item\_fav\_brand |
# +-----------+--------------------+
# | 1         | no                 |
# | 2         | yes                |
# | 3         | yes                |
# | 4         | no                 |
# +-----------+--------------------+
# **Explanation:** 
# The answer for the user with id 1 is no because they sold nothing.
# The answer for the users with id 2 and 3 is yes because the brands of their second sold items are their favorite brands.
# The answer for the user with id 4 is no because the brand of their second sold item is not their favorite brand.



import pandas as pd

def market_analysis(users: pd.DataFrame, orders: pd.DataFrame, items: pd.DataFrame) -> pd.DataFrame:
  order_item = pd.merge(orders, items, on='item_id', how='inner')
  order_item['rank'] = order_item.groupby('seller_id')['order_date'].rank()
  order_item = order_item[order_item['rank']==2][['seller_id', 'item_brand']]

  df = pd.merge(users, order_item, left_on='user_id', right_on='seller_id', how='left')
  df['2nd_item_fav_brand'] = np.where(df['favorite_brand']==df['item_brand'], 'yes', 'no')
  return df[['user_id', '2nd_item_fav_brand']].rename(columns={'user_id':'seller_id'})