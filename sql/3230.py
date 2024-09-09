# Medium

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Transactions`

# +------------------+---------+
# | Column Name      | Type    |
# +------------------+---------+
# | transaction\_id   | int     |
# | customer\_id      | int     |
# | product\_id       | int     |
# | transaction\_date | date    |
# | amount           | decimal |
# +------------------+---------+
# transaction\_id is the unique identifier for this table.
# Each row of this table contains information about a transaction, including the customer ID, product ID, date, and amount spent.

# Table: `Products`

# +-------------+---------+
# | Column Name | Type    |
# +-------------+---------+
# | product\_id  | int     |
# | category    | varchar |
# | price       | decimal |
# +-------------+---------+
# product\_id is the unique identifier for this table.
# Each row of this table contains information about a product, including its category and price.

# Write a solution to analyze customer purchasing behavior. For **each customer**, calculate:

# *   The total amount spent.
# *   The number of transactions.
# *   The number of **unique** product categories purchased.
# *   The average amount spent. 
# *   The **most frequently** purchased product category (if there is a tie, choose the one with the most recent transaction).
# *   A **loyalty score** defined as: (Number of transactions \* 10) + (Total amount spent / 100).

# Round `total_amount`, `avg_transaction_amount`, and `loyalty_score` to `2` decimal places.

# Return _the result table ordered by_ `loyalty_score` _in **descending** order_, _then by_ `customer_id` _in **ascending** order_.

# The query result format is in the following example.

# **Example:**

# **Input:**

# `Transactions` table:

# +----------------+-------------+------------+------------------+--------+
# | transaction\_id | customer\_id | product\_id | transaction\_date | amount |
# +----------------+-------------+------------+------------------+--------+
# | 1              | 101         | 1          | 2023-01-01       | 100.00 |
# | 2              | 101         | 2          | 2023-01-15       | 150.00 |
# | 3              | 102         | 1          | 2023-01-01       | 100.00 |
# | 4              | 102         | 3          | 2023-01-22       | 200.00 |
# | 5              | 101         | 3          | 2023-02-10       | 200.00 |
# +----------------+-------------+------------+------------------+--------+

# `Products` table:

# +------------+----------+--------+
# | product\_id | category | price  |
# +------------+----------+--------+
# | 1          | A        | 100.00 |
# | 2          | B        | 150.00 |
# | 3          | C        | 200.00 |
# +------------+----------+--------+

# **Output:**

# +-------------+--------------+-------------------+-------------------+------------------------+--------------+---------------+
# | customer\_id | total\_amount | transaction\_count | unique\_categories | avg\_transaction\_amount | top\_category | loyalty\_score |
# +-------------+--------------+-------------------+-------------------+------------------------+--------------+---------------+
# | 101         | 450.00       | 3                 | 3                 | 150.00                 | C            | 34.50         |
# | 102         | 300.00       | 2                 | 2                 | 150.00                 | C            | 23.00         |
# +-------------+--------------+-------------------+-------------------+------------------------+--------------+---------------+

# **Explanation:**

# *   For customer 101:
#     *   Total amount spent: 100.00 + 150.00 + 200.00 = 450.00
#     *   Number of transactions: 3
#     *   Unique categories: A, B, C (3 categories)
#     *   Average transaction amount: 450.00 / 3 = 150.00
#     *   Top category: C (Customer 101 made 1 purchase each in categories A, B, and C. Since the count is the same for all categories, we choose the most recent transaction, which is category C on 2023-02-10)
#     *   Loyalty score: (3 \* 10) + (450.00 / 100) = 34.50
# *   For customer 102:
#     *   Total amount spent: 100.00 + 200.00 = 300.00
#     *   Number of transactions: 2
#     *   Unique categories: A, C (2 categories)
#     *   Average transaction amount: 300.00 / 2 = 150.00
#     *   Top category: C (Customer 102 made 1 purchase each in categories A and C. Since the count is the same for both categories, we choose the most recent transaction, which is category C on 2023-01-22)
#     *   Loyalty score: (2 \* 10) + (300.00 / 100) = 23.00

# **Note:** The output is ordered by loyalty\_score in descending order, then by customer\_id in ascending order.




import pandas as pd




def analyze_customer_behavior(transactions: pd.DataFrame, products: pd.DataFrame) -> pd.DataFrame:
    my_rnd = lambda x: round(x+.00001, 2)
    transactions = transactions.merge(products, on = 'product_id')

    agg_df = transactions.groupby('customer_id').agg(
        total_amount =           ('amount', 'sum'),
        transaction_count =      ('transaction_id', 'count'),
        avg_transaction_amount = ('amount', 'mean') ).reset_index()

    uni_df = transactions.groupby('customer_id')['category'].nunique().reset_index()

    top_df = transactions.groupby(['customer_id', 'category']).agg(
        count =                  ('transaction_id', 'count'),
        last_transaction_date =  ('transaction_date', 'max') ).reset_index()

    top_df['rank'] = top_df.groupby('customer_id')['count'
                          ].rank(method = 'dense', ascending = 0)

    top_df = top_df[top_df['rank'] == 1
                  ].sort_values(by = ['customer_id', 'rank', 'last_transaction_date'], ascending = [1,1,0]
                  ).drop_duplicates(subset = ['customer_id'], keep = 'first')

    df = agg_df.merge(uni_df, on = 'customer_id').merge(top_df, on = 'customer_id')

    df['loyalty_score'] = (df.transaction_count * 10) + (df.total_amount / 100).apply(my_rnd)
    df['total_amount'] = df.total_amount.apply(my_rnd)
    df['avg_transaction_amount'] = df.avg_transaction_amount.apply(my_rnd)

    return df.sort_values(by = ['loyalty_score', 'customer_id'], ascending = [0,1]
            ).rename(columns = {'category_x': 'unique_categories', 'category_y': 'top_category'}
            ).iloc[:,[0,1,2,4,3,5,9]]