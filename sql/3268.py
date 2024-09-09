# Hard

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `EmployeeShifts`

# +------------------+----------+
# | Column Name      | Type     |
# +------------------+----------+
# | employee\_id      | int      |
# | start\_time       | datetime |
# | end\_time         | datetime |
# +------------------+----------+
# (employee\_id, start\_time) is the unique key for this table.
# This table contains information about the shifts worked by employees, including the start time, and end time.

# Write a solution to analyze overlapping shifts for each employee. Two shifts are considered overlapping if they occur on the **same date** and one shift's `end_time` is **later than** another shift's `start_time`.

# For **each employee**, calculate the following:

# 1.  The **maximum** number of shifts that **overlap** at any **given time**.
# 2.  The **total duration** of all overlaps in minutes.

# _Return the result table ordered by_ `employee_id` _in **ascending** order_.

# The query result format is in the following example.

# **Example:**

# **Input:**

# `EmployeeShifts` table:

# +-------------+---------------------+---------------------+
# | employee\_id | start\_time          | end\_time            |
# +-------------+---------------------+---------------------+
# | 1           | 2023-10-01 09:00:00 | 2023-10-01 17:00:00 |
# | 1           | 2023-10-01 15:00:00 | 2023-10-01 23:00:00 |
# | 1           | 2023-10-01 16:00:00 | 2023-10-02 00:00:00 |
# | 2           | 2023-10-01 09:00:00 | 2023-10-01 17:00:00 |
# | 2           | 2023-10-01 11:00:00 | 2023-10-01 19:00:00 |
# | 3           | 2023-10-01 09:00:00 | 2023-10-01 17:00:00 |
# +-------------+---------------------+---------------------+

# **Output:**

# +-------------+---------------------------+------------------------+
# | employee\_id | max\_overlapping\_shifts    | total\_overlap\_duration |
# +-------------+---------------------------+------------------------+
# | 1           | 3                         | 600                    |
# | 2           | 2                         | 360                    |
# | 3           | 1                         | 0                      |
# +-------------+---------------------------+------------------------+

# **Explanation:**

# *   Employee 1 has 3 shifts:
#     *   2023-10-01 09:00:00 to 2023-10-01 17:00:00
#     *   2023-10-01 15:00:00 to 2023-10-01 23:00:00
#     *   2023-10-01 16:00:00 to 2023-10-02 00:00:00The maximum number of overlapping shifts is 3 (from 16:00 to 17:00). The total overlap duration is: - 2 hours (15:00-17:00) between 1st and 2nd shifts - 1 hour (16:00-17:00) between 1st and 3rd shifts - 7 hours (16:00-23:00) between 2nd and 3rd shifts Total: 10 hours = 600 minutes
# *   Employee 2 has 2 shifts:
#     *   2023-10-01 09:00:00 to 2023-10-01 17:00:00
#     *   2023-10-01 11:00:00 to 2023-10-01 19:00:00The maximum number of overlapping shifts is 2. The total overlap duration is 6 hours (11:00-17:00) = 360 minutes.
# *   Employee 3 has only 1 shift, so there are no overlaps.

# The output table contains the employee\_id, the maximum number of simultaneous overlaps, and the total overlap duration in minutes for each employee, ordered by employee\_id in ascending order.



import pandas as pd

def calculate_shift_overlaps(employee_shifts: pd.DataFrame) -> pd.DataFrame:

    df = (pd.melt(employee_shifts, id_vars=['employee_id', ],
             value_vars=['start_time', 'end_time'], value_name='time')
             .sort_values(['employee_id', 'time']))


    df['time']  = pd.to_datetime(df.time)                      
    df['incr']  = df.variable.apply(lambda x: 1 if x == 'start_time' else -1)
    df['c_sum'] = df.groupby(['employee_id'])['incr'].cumsum()
    df_max = df.groupby(['employee_id'])['c_sum'].max().reset_index()

    df['mult'] = df.c_sum.apply(lambda x: comb(x, 2))
    df['total_overlap_duration'] = ((((df.time.shift(-1) - df.time) / 
                                                pd.Timedelta(minutes=1)) * df.mult))
    df_tot = (df.groupby(['employee_id'])['total_overlap_duration'].sum().reset_index())

    return (df_max.merge(df_tot, how = 'left')
              .rename(columns = {'c_sum':'max_overlapping_shifts'}))