# Medium

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Purchases`

# +---------------+------+
# | Column Name   | Type |
# +---------------+------+
# | user\_id       | int  |
# | purchase\_date | date |
# | amount\_spend  | int  |
# +---------------+------+
# (user\_id, purchase\_date, amount\_spend) is the primary key (combination of columns with unique values) for this table.
# purchase\_date will range from November 1, 2023, to November 30, 2023, inclusive of both dates.
# Each row contains user\_id, purchase\_date, and amount\_spend.

# Table: `Users`

# +-------------+------+
# | Column Name | Type |
# +-------------+------+
# | user\_id     | int  |
# | membership  | enum |
# +-------------+------+
# user\_id is the primary key for this table.
# membership is an ENUM (category) type of ('Standard', 'Premium', 'VIP').
# Each row of this table indicates the user\_id, membership type.

# Write a solution to calculate the **total spending** by `Premium` and `VIP` members on **each Friday of every week** in November 2023.  If there are **no purchases** on a **particular Friday** by `Premium` or `VIP` members, it should be considered as `0`.

# Return _the result table_ _ordered by week of the month,  and_ `membership` _in **ascending** order_.

# The result format is in the following example.

# **Example:**

# **Input:**

# Purchases table:

# +---------+---------------+--------------+
# | user\_id | purchase\_date | amount\_spend |
# +---------+---------------+--------------+
# | 11      | 2023-11-03    | 1126         |
# | 15      | 2023-11-10    | 7473         |
# | 17      | 2023-11-17    | 2414         |
# | 12      | 2023-11-24    | 9692         |
# | 8       | 2023-11-24    | 5117         |
# | 1       | 2023-11-24    | 5241         |
# | 10      | 2023-11-22    | 8266         |
# | 13      | 2023-11-21    | 12000        |
# +---------+---------------+--------------+

# Users table:

# +---------+------------+
# | user\_id | membership |
# +---------+------------+
# | 11      | Premium    |
# | 15      | VIP        |
# | 17      | Standard   |
# | 12      | VIP        |
# | 8       | Premium    |
# | 1       | VIP        |
# | 10      | Standard   |
# | 13      | Premium    |
# +---------+------------+

# **Output:**

# +---------------+-------------+--------------+
# | week\_of\_month | membership  | total\_amount |
# +---------------+-------------+--------------+
# | 1             | Premium     | 1126         |
# | 1             | VIP         | 0            |
# | 2             | Premium     | 0            |
# | 2             | VIP         | 7473         |
# | 3             | Premium     | 0            |
# | 3             | VIP         | 0            |
# | 4             | Premium     | 5117         |
# | 4             | VIP         | 14933        |
# +---------------+-------------+--------------+
        

# **Explanation:**

# *   During the first week of November 2023, a transaction occurred on Friday, 2023-11-03, by a Premium member amounting to $1,126. No transactions were made by VIP members on this day, resulting in a value of 0.
# *   For the second week of November 2023, there was a transaction on Friday, 2023-11-10, and it was made by a VIP member, amounting to $7,473. Since there were no purchases by Premium members that Friday, the output shows 0 for Premium members.
# *   Similarly, during the third week of November 2023, no transactions by Premium or VIP members occurred on Friday, 2023-11-17, which shows 0 for both categories in this week.
# *   In the fourth week of November 2023, transactions occurred on Friday, 2023-11-24, involving one Premium member purchase of $5,117 and VIP member purchases totaling $14,933 ($9,692 from one and $5,241 from another).

# **Note:** The output table is ordered by week\_of\_month and membership in ascending order.


import pandas as pd

def friday_purchases(purchases: pd.DataFrame, users: pd.DataFrame) -> pd.DataFrame:

    df = users.merge(purchases, on  = "user_id", how = 'left')
    df['week_of_month'] = df.purchase_date.dt.day.apply(lambda x: x//7+1)

    df =    (
        df[df.purchase_date.dt.weekday == 4][df.membership.isin(['Premium','VIP'])]
          .rename(columns = {'amount_spend':'total_amount'})
          .groupby(['week_of_month','membership'])['total_amount'].sum().reset_index()
            )

    return  (
        pd.DataFrame({'membership':['Premium','VIP']*4, 'week_of_month':[1,1,2,2,3,3,4,4]})
          .merge(df, on = ['week_of_month', 'membership'], how = 'left')
          .fillna(0).iloc[:,[1,0,2]]
            )  