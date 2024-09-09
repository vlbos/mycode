#  Medium

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `ParkingTransactions`

# +--------------+-----------+
# | Column Name  | Type      |
# +--------------+-----------+
# | lot\_id       | int       |
# | car\_id       | int       |
# | entry\_time   | datetime  |
# | exit\_time    | datetime  |
# | fee\_paid     | decimal   |
# +--------------+-----------+
# (lot\_id, car\_id, entry\_time) is the primary key (combination of columns with unique values) for this table.
# Each row of this table contains the ID of the parking lot, the ID of the car, the entry and exit times, and the fee paid for the parking duration.

# Write a solution to find the **total parking fee** paid by each car **across all parking lots**, and the **average hourly fee** (rounded to `2` decimal places) paid by **each** car. Also, find the **parking lot** where each car spent the **most total time**.

# Return _the result table ordered by_ `car_id` _in **ascending**_ _order._

# **Note:** Test cases are generated in such a way that an individual car cannot be in multiple parking lots at the same time.

# The result format is in the following example.

# **Example:**

# **Input:**

# ParkingTransactions table:

# +--------+--------+---------------------+---------------------+----------+
# | lot\_id | car\_id | entry\_time          | exit\_time           | fee\_paid |
# +--------+--------+---------------------+---------------------+----------+
# | 1      | 1001   | 2023-06-01 08:00:00 | 2023-06-01 10:30:00 | 5.00     |
# | 1      | 1001   | 2023-06-02 11:00:00 | 2023-06-02 12:45:00 | 3.00     |
# | 2      | 1001   | 2023-06-01 10:45:00 | 2023-06-01 12:00:00 | 6.00     |
# | 2      | 1002   | 2023-06-01 09:00:00 | 2023-06-01 11:30:00 | 4.00     |
# | 3      | 1001   | 2023-06-03 07:00:00 | 2023-06-03 09:00:00 | 4.00     |
# | 3      | 1002   | 2023-06-02 12:00:00 | 2023-06-02 14:00:00 | 2.00     |
# +--------+--------+---------------------+---------------------+----------+

# **Output:**

# +--------+----------------+----------------+---------------+
# | car\_id | total\_fee\_paid | avg\_hourly\_fee | most\_time\_lot |
# +--------+----------------+----------------+---------------+
# | 1001   | 18.00          | 2.40           | 1             |
# | 1002   | 6.00           | 1.33           | 2             |
# +--------+----------------+----------------+---------------+

# **Explanation:**

# *   For car ID 1001:
#     *   From 2023-06-01 08:00:00 to 2023-06-01 10:30:00 in lot 1: 2.5 hours, fee 5.00
#     *   From 2023-06-02 11:00:00 to 2023-06-02 12:45:00 in lot 1: 1.75 hours, fee 3.00
#     *   From 2023-06-01 10:45:00 to 2023-06-01 12:00:00 in lot 2: 1.25 hours, fee 6.00
#     *   From 2023-06-03 07:00:00 to 2023-06-03 09:00:00 in lot 3: 2 hours, fee 4.00Total fee paid: 18.00, total hours: 7.5, average hourly fee: 2.40, most time spent in lot 1: 4.25 hours.
# *   For car ID 1002:
#     *   From 2023-06-01 09:00:00 to 2023-06-01 11:30:00 in lot 2: 2.5 hours, fee 4.00
#     *   From 2023-06-02 12:00:00 to 2023-06-02 14:00:00 in lot 3: 2 hours, fee 2.00Total fee paid: 6.00, total hours: 4.5, average hourly fee: 1.33, most time spent in lot 2: 2.5 hours.

# **Note:** Output table is ordered by car\_id in ascending order.




import pandas as pd

def calculate_fees_and_duration(df: pd.DataFrame) -> pd.DataFrame:
    df['hours'] = (df['exit_time'] - df['entry_time']).dt.seconds / 3600
    def fn(group):
        return pd.Series({
            'total_fee_paid': group.fee_paid.sum(),
            'avg_hourly_fee': round(group.fee_paid.sum() / group.hours.sum(), 2),
            'most_time_lot': group.groupby('lot_id')['hours'].sum().idxmax()
        })
    return df.groupby('car_id', as_index=False).apply(fn)