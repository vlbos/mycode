# Hard

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Products`

# +-------------+------+
# | Column Name | Type |
# +-------------+------+
# | product\_id  | int  |
# | price       | int  |
# +-------------+------+
# product\_id contains unique values.
# Each row in this table shows the ID of a product and the price of one unit.

# Table: `Purchases`

# +-------------+------+
# | Column Name | Type |
# +-------------+------+
# | invoice\_id  | int  |
# | product\_id  | int  |
# | quantity    | int  |
# +-------------+------+
# (invoice\_id, product\_id) is the primary key (combination of columns with unique values) for this table.
# Each row in this table shows the quantity ordered from one product in an invoice. 

# Write a solution to show the details of the invoice with the highest price. If two or more invoices have the same price, return the details of the one with the smallest `invoice_id`.

# Return the result table in **any order**.

# The result format is shown in the following example.

# **Example 1:**

# **Input:** 
# Products table:
# +------------+-------+
# | product\_id | price |
# +------------+-------+
# | 1          | 100   |
# | 2          | 200   |
# +------------+-------+
# Purchases table:
# +------------+------------+----------+
# | invoice\_id | product\_id | quantity |
# +------------+------------+----------+
# | 1          | 1          | 2        |
# | 3          | 2          | 1        |
# | 2          | 2          | 3        |
# | 2          | 1          | 4        |
# | 4          | 1          | 10       |
# +------------+------------+----------+
# **Output:** 
# +------------+----------+-------+
# | product\_id | quantity | price |
# +------------+----------+-------+
# | 2          | 3        | 600   |
# | 1          | 4        | 400   |
# +------------+----------+-------+
# **Explanation:** 
# Invoice 1: price = (2 \* 100) = $200
# Invoice 2: price = (4 \* 100) + (3 \* 200) = $1000
# Invoice 3: price = (1 \* 200) = $200
# Invoice 4: price = (10 \* 100) = $1000

# The highest price is $1000, and the invoices with the highest prices are 2 and 4. We return the details of the one with the smallest ID, which is invoice 2.



import pandas as pd

def generate_the_invoice(products: pd.DataFrame, purchases: pd.DataFrame) -> pd.DataFrame:
    purchases = purchases.merge(products)
    purchases['price'] *= purchases['quantity']
    prices = purchases.groupby('invoice_id')['price'].sum().reset_index(name='total')
    prices.sort_values(['total', 'invoice_id'], ascending=[False, True], inplace=True)
    return purchases[purchases.invoice_id == prices.iloc[0].invoice_id][['product_id', 'quantity', 'price']]