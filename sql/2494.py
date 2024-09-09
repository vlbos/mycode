# Hard

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `HallEvents`

# +-------------+------+
# | Column Name | Type |
# +-------------+------+
# | hall\_id     | int  |
# | start\_day   | date |
# | end\_day     | date |
# +-------------+------+
# This table may contain duplicates rows.
# Each row of this table indicates the start day and end day of an event and the hall in which the event is held.

# Write a solution to merge all the overlapping events that are held **in the same hall**. Two events overlap if they have **at least one day** in common.

# Return the result table **in any order**.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# HallEvents table:
# +---------+------------+------------+
# | hall\_id | start\_day  | end\_day    |
# +---------+------------+------------+
# | 1       | 2023-01-13 | 2023-01-14 |
# | 1       | 2023-01-14 | 2023-01-17 |
# | 1       | 2023-01-18 | 2023-01-25 |
# | 2       | 2022-12-09 | 2022-12-23 |
# | 2       | 2022-12-13 | 2022-12-17 |
# | 3       | 2022-12-01 | 2023-01-30 |
# +---------+------------+------------+
# **Output:** 
# +---------+------------+------------+
# | hall\_id | start\_day  | end\_day    |
# +---------+------------+------------+
# | 1       | 2023-01-13 | 2023-01-17 |
# | 1       | 2023-01-18 | 2023-01-25 |
# | 2       | 2022-12-09 | 2022-12-23 |
# | 3       | 2022-12-01 | 2023-01-30 |
# +---------+------------+------------+
# **Explanation:** There are three halls.
# Hall 1:
# - The two events \["2023-01-13", "2023-01-14"\] and \["2023-01-14", "2023-01-17"\] overlap. We merge them in one event \["2023-01-13", "2023-01-17"\].
# - The event \["2023-01-18", "2023-01-25"\] does not overlap with any other event, so we leave it as it is.
# Hall 2:
# - The two events \["2022-12-09", "2022-12-23"\] and \["2022-12-13", "2022-12-17"\] overlap. We merge them in one event \["2022-12-09", "2022-12-23"\].
# Hall 3:
# - The hall has only one event, so we return it. Note that we only consider the events of each hall separately.


import pandas as pd

def merge_events(hall_events: pd.DataFrame) -> pd.DataFrame:
    df = hall_events.sort_values(by=['hall_id', 'start_day', 'end_day'])
    df['prev_end_day'] = df.groupby('hall_id')['end_day'].shift(1)
    df['prev_max_end_day'] = df.groupby('hall_id')['prev_end_day'].cummax()
    df['is_new_event'] = (df['start_day'] > df['prev_max_end_day']) | (df['prev_max_end_day'].isna())
    df['group_id'] = df.groupby('hall_id')['is_new_event'].cumsum()

    df = df.groupby(['hall_id', 'group_id']).agg(start_day=('start_day', 'min'), end_day=('end_day', 'max')).reset_index()

    return df[['hall_id', 'start_day', 'end_day']]