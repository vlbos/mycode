# Easy

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Sales`

# +--------------+---------+
# | Column Name  | Type    |
# +--------------+---------+
# | sale\_id      | int     |
# | product\_name | varchar |
# | sale\_date    | date    |
# +--------------+---------+
# sale\_id is the column with unique values for this table.
# Each row of this table contains the product name and the date it was sold.

# Since table Sales was filled manually in the year `2000`, `product_name` may contain leading and/or trailing white spaces, also they are case-insensitive.

# Write a solution to report

# *   `product_name` in lowercase without leading or trailing white spaces.
# *   `sale_date` in the format `('YYYY-MM')`.
# *   `total` the number of times the product was sold in this month.

# Return the result table ordered by `product_name` in **ascending order**. In case of a tie, order it by `sale_date` in **ascending order**.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Sales table:
# +---------+--------------+------------+
# | sale\_id | product\_name | sale\_date  |
# +---------+--------------+------------+
# | 1       | LCPHONE      | 2000-01-16 |
# | 2       | LCPhone      | 2000-01-17 |
# | 3       | LcPhOnE      | 2000-02-18 |
# | 4       | LCKeyCHAiN   | 2000-02-19 |
# | 5       | LCKeyChain   | 2000-02-28 |
# | 6       | Matryoshka   | 2000-03-31 |
# +---------+--------------+------------+
# **Output:** 
# +--------------+-----------+-------+
# | product\_name | sale\_date | total |
# +--------------+-----------+-------+
# | lckeychain   | 2000-02   | 2     |
# | lcphone      | 2000-01   | 2     |
# | lcphone      | 2000-02   | 1     |
# | matryoshka   | 2000-03   | 1     |
# +--------------+-----------+-------+
# **Explanation:** 
# In January, 2 LcPhones were sold. Please note that the product names are not case sensitive and may contain spaces.
# In February, 2 LCKeychains and 1 LCPhone were sold.
# In March, one matryoshka was sold.



import pandas as pd

def fix_name_format(sales: pd.DataFrame) -> pd.DataFrame:
    return (
        sales.assign(
            product_name=sales["product_name"].str.strip().str.lower(),
            sale_date=sales["sale_date"].dt.strftime('%Y-%m'),
        )
        .groupby(["product_name", "sale_date"], as_index=False)
        .agg(total=("sale_id", "count"))
        .sort_values(["product_name", "sale_date"])
    )