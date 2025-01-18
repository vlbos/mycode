# Hard

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Product`

# +---------------+---------+
# | Column Name   | Type    |
# +---------------+---------+
# | product\_id    | int     |
# | product\_name  | varchar |
# +---------------+---------+
# product\_id is the primary key (column with unique values) for this table.
# product\_name is the name of the product.

# Table: `Sales`

# +---------------------+---------+
# | Column Name         | Type    |
# +---------------------+---------+
# | product\_id          | int     |
# | period\_start        | date    |
# | period\_end          | date    |
# | average\_daily\_sales | int     |
# +---------------------+---------+
# product\_id is the primary key (column with unique values) for this table. 
# period\_start and period\_end indicate the start and end date for the sales period, and both dates are inclusive.
# The average\_daily\_sales column holds the average daily sales amount of the items for the period.
# The dates of the sales years are between 2018 to 2020.

# Write a solution to report the total sales amount of each item for each year, with corresponding `product_name`, `product_id`, `report_year`, and `total_amount`.

# Return the result table **ordered** by `product_id` and `report_year`.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Product table:
# +------------+--------------+
# | product\_id | product\_name |
# +------------+--------------+
# | 1          | LC Phone     |
# | 2          | LC T-Shirt   |
# | 3          | LC Keychain  |
# +------------+--------------+
# Sales table:
# +------------+--------------+-------------+---------------------+
# | product\_id | period\_start | period\_end  | average\_daily\_sales |
# +------------+--------------+-------------+---------------------+
# | 1          | 2019-01-25   | 2019-02-28  | 100                 |
# | 2          | 2018-12-01   | 2020-01-01  | 10                  |
# | 3          | 2019-12-01   | 2020-01-31  | 1                   |
# +------------+--------------+-------------+---------------------+
# **Output:** 
# +------------+--------------+-------------+--------------+
# | product\_id | product\_name | report\_year | total\_amount |
# +------------+--------------+-------------+--------------+
# | 1          | LC Phone     |    2019     | 3500         |
# | 2          | LC T-Shirt   |    2018     | 310          |
# | 2          | LC T-Shirt   |    2019     | 3650         |
# | 2          | LC T-Shirt   |    2020     | 10           |
# | 3          | LC Keychain  |    2019     | 31           |
# | 3          | LC Keychain  |    2020     | 31           |
# +------------+--------------+-------------+--------------+
# **Explanation:** 
# LC Phone was sold for the period of 2019-01-25 to 2019-02-28, and there are 35 days for this period. Total amount 35\*100 = 3500. 
# LC T-shirt was sold for the period of 2018-12-01 to 2020-01-01, and there are 31, 365, 1 days for years 2018, 2019 and 2020 respectively.
# LC Keychain was sold for the period of 2019-12-01 to 2020-01-31, and there are 31, 31 days for years 2019 and 2020 respectively.


import pandas as pd
from datetime import datetime

def total_sales(product: pd.DataFrame, sales: pd.DataFrame) -> pd.DataFrame:

    df = sales.merge(
         pd.DataFrame({'report_year': ['2018', '2019', '2020'],
                       'beg_year':[datetime(2018,1,1),
                                   datetime(2019,1,1),
                                   datetime(2020,1,1)],
                       'end_year':[datetime(2018,12,31),
                                   datetime(2019,12,31),
                                   datetime(2020,12,31)]}),
                      how = 'cross').merge(product)
                      
    df['beg'] = df[["beg_year", "period_start"]].max(axis=1)
    df['end'] = df[["end_year", "period_end"]].min(axis=1)
    df['total_amount'] = ((df.end - df.beg).dt.days + 1)*df.average_daily_sales
               
    return df[df.total_amount > 0].iloc[:,[0,7,4,10]]