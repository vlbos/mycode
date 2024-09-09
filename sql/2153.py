
# Hard

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Buses`

# +--------------+------+
# | Column Name  | Type |
# +--------------+------+
# | bus\_id       | int  |
# | arrival\_time | int  |
# | capacity     | int  |
# +--------------+------+
# bus\_id contains unique values.
# Each row of this table contains information about the arrival time of a bus at the LeetCode station and its capacity (the number of empty seats it has).
# No two buses will arrive at the same time and all bus capacities will be positive integers.

# Table: `Passengers`

# +--------------+------+
# | Column Name  | Type |
# +--------------+------+
# | passenger\_id | int  |
# | arrival\_time | int  |
# +--------------+------+
# passenger\_id contains unique values.
# Each row of this table contains information about the arrival time of a passenger at the LeetCode station.

# Buses and passengers arrive at the LeetCode station. If a bus arrives at the station at a time `tbus` and a passenger arrived at a time `tpassenger` where `tpassenger <= tbus` and the passenger did not catch any bus, the passenger will use that bus. In addition, each bus has a capacity. If at the moment the bus arrives at the station there are more passengers waiting than its capacity `capacity`, only `capacity` passengers will use the bus.

# Write a solution to report the number of users that used each bus.

# Return the result table ordered by `bus_id` in **ascending order**.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Buses table:
# +--------+--------------+----------+
# | bus\_id | arrival\_time | capacity |
# +--------+--------------+----------+
# | 1      | 2            | 1        |
# | 2      | 4            | 10       |
# | 3      | 7            | 2        |
# +--------+--------------+----------+
# Passengers table:
# +--------------+--------------+
# | passenger\_id | arrival\_time |
# +--------------+--------------+
# | 11           | 1            |
# | 12           | 1            |
# | 13           | 5            |
# | 14           | 6            |
# | 15           | 7            |
# +--------------+--------------+
# **Output:** 
# +--------+----------------+
# | bus\_id | passengers\_cnt |
# +--------+----------------+
# | 1      | 1              |
# | 2      | 1              |
# | 3      | 2              |
# +--------+----------------+
# **Explanation:** 
# - Passenger 11 arrives at time 1.
# - Passenger 12 arrives at time 1.
# - Bus 1 arrives at time 2 and collects passenger 11 as it has one empty seat.

# - Bus 2 arrives at time 4 and collects passenger 12 as it has ten empty seats.

# - Passenger 12 arrives at time 5.
# - Passenger 13 arrives at time 6.
# - Passenger 14 arrives at time 7.
# - Bus 3 arrives at time 7 and collects passengers 12 and 13 as it has two empty seats.

import pandas as pd

def number_of_passengers(buses: pd.DataFrame, passengers: pd.DataFrame) -> pd.DataFrame:
    df = pd.merge(passengers, buses, how='cross')
    df = df[df['arrival_time_x'] <= df['arrival_time_y']]
    df = df.groupby(['bus_id', 'capacity', 'arrival_time_y']).agg(total_passenger=('passenger_id', 'nunique')).reset_index().rename(columns={'arrival_time_y':'arrival_time'})
    df = df.sort_values(by='arrival_time', ignore_index=True)
    df['passengers_cnt'] = 0
    df['running_cnt'] = 0

    for i in range(len(df)):
        if i == 0:
            df.loc[0, 'passengers_cnt'] = min(df.loc[0, 'total_passenger'], df.loc[0, 'capacity'])
            df.loc[0, 'running_cnt'] = df.loc[0, 'passengers_cnt']
        else:
            df.loc[i, 'passengers_cnt'] = min(df.loc[i, 'total_passenger'] - df.loc[i-1, 'running_cnt'], df.loc[i, 'capacity'])
            df.loc[i, 'running_cnt'] = df.loc[i-1, 'running_cnt'] + df.loc[i, 'passengers_cnt']

    df = pd.merge(buses, df, on='bus_id', how='left')
    df = df.sort_values(by='bus_id')
    
    return df[['bus_id', 'passengers_cnt']].fillna(0)