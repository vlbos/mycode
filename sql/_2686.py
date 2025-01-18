# Medium

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Delivery`

# +-----------------------------+---------+
# | Column Name                 | Type    |
# +-----------------------------+---------+
# | delivery\_id                 | int     |
# | customer\_id                 | int     |
# | order\_date                  | date    |
# | customer\_pref\_delivery\_date | date    |
# +-----------------------------+---------+
# delivery\_id is the column with unique values of this table.
# Each row contains information about food delivery to a customer that makes an order at some date and specifies a preferred delivery date (on the order date or after it).

# If the customer's preferred delivery date is the same as the order date, then the order is called **immediate,** otherwise, it is **scheduled**.

# Write a solution to find the percentage of immediate orders on each unique `order_date`, **rounded to 2 decimal places**. 

# Return _the result table ordered by_ `order_date` _in **ascending** order._

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Delivery table:
# +-------------+-------------+------------+-----------------------------+
# | delivery\_id | customer\_id | order\_date | customer\_pref\_delivery\_date |
# +-------------+-------------+------------+-----------------------------+
# | 1           | 1           | 2019-08-01 | 2019-08-02                  |
# | 2           | 2           | 2019-08-01 | 2019-08-01                  |
# | 3           | 1           | 2019-08-01 | 2019-08-01                  |
# | 4           | 3           | 2019-08-02 | 2019-08-13                  |
# | 5           | 3           | 2019-08-02 | 2019-08-02                  |
# | 6           | 2           | 2019-08-02 | 2019-08-02                  |
# | 7           | 4           | 2019-08-03 | 2019-08-03                  |
# | 8           | 1           | 2019-08-03 | 2019-08-03                  |
# | 9           | 5           | 2019-08-04 | 2019-08-08                  |
# | 10          | 2           | 2019-08-04 | 2019-08-18                  |
# +-------------+-------------+------------+-----------------------------+
# **Output:** 
# +------------+----------------------+
# | order\_date | immediate\_percentage |
# +------------+----------------------+
# | 2019-08-01 | 66.67                |
# | 2019-08-02 | 66.67                |
# | 2019-08-03 | 100.00               |
# | 2019-08-04 | 0.00                 |
# +------------+----------------------+
# **Explanation:** 
# - On 2019-08-01 there were three orders, out of those, two were immediate and one was scheduled. So, immediate percentage for that date was 66.67.
# - On 2019-08-02 there were three orders, out of those, two were immediate and one was scheduled. So, immediate percentage for that date was 66.67.
# - On 2019-08-03 there were two orders, both were immediate. So, the immediate percentage for that date was 100.00.
# - On 2019-08-04 there were two orders, both were scheduled. So, the immediate percentage for that date was 0.00.
# order\_date is sorted in ascending order.




import pandas as pd

def immediate_delivery(delivery: pd.DataFrame) -> pd.DataFrame:
    delivery['immediate_ind'] = np.where(delivery['order_date']==delivery['customer_pref_delivery_date'], 1, 0)
    return delivery.groupby(['order_date'])['immediate_ind'].agg(lambda x: round(sum(x)/len(x),4)*100).reset_index(name='immediate_percentage').sort_values(by='order_date')