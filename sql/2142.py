# Medium

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
# +--------------+------+
# bus\_id is the column with unique values for this table.
# Each row of this table contains information about the arrival time of a bus at the LeetCode station.
# No two buses will arrive at the same time.

# Table: `Passengers`

# +--------------+------+
# | Column Name  | Type |
# +--------------+------+
# | passenger\_id | int  |
# | arrival\_time | int  |
# +--------------+------+
# passenger\_id is the column with unique values for this table.
# Each row of this table contains information about the arrival time of a passenger at the LeetCode station.

# Buses and passengers arrive at the LeetCode station. If a bus arrives at the station at time `tbus` and a passenger arrived at time `tpassenger` where `tpassenger <= tbus` and the passenger did not catch any bus, the passenger will use that bus.

# Write a solution to report the number of users that used each bus.

# Return the result table ordered by `bus_id` in **ascending order**.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Buses table:
# +--------+--------------+
# | bus\_id | arrival\_time |
# +--------+--------------+
# | 1      | 2            |
# | 2      | 4            |
# | 3      | 7            |
# +--------+--------------+
# Passengers table:
# +--------------+--------------+
# | passenger\_id | arrival\_time |
# +--------------+--------------+
# | 11           | 1            |
# | 12           | 5            |
# | 13           | 6            |
# | 14           | 7            |
# +--------------+--------------+
# **Output:** 
# +--------+----------------+
# | bus\_id | passengers\_cnt |
# +--------+----------------+
# | 1      | 1              |
# | 2      | 0              |
# | 3      | 3              |
# +--------+----------------+
# **Explanation:** 
# - Passenger 11 arrives at time 1.
# - Bus 1 arrives at time 2 and collects passenger 11.

# - Bus 2 arrives at time 4 and does not collect any passengers.

# - Passenger 12 arrives at time 5.
# - Passenger 13 arrives at time 6.
# - Passenger 14 arrives at time 7.
# - Bus 3 arrives at time 7 and collects passengers 12, 13, and 14.



import pandas as pd

def count_passengers_in_bus(buses: pd.DataFrame, passengers: pd.DataFrame) -> pd.DataFrame:
    
    buses.sort_values('arrival_time', inplace = True)
    arr = sorted(passengers.arrival_time)

    lst = [bisect_right(arr, b) for b in buses.arrival_time]
    buses['passengers_cnt'] = [y - x for x,y in pairwise([0]+lst)]
    
    return buses.iloc[:,[0,2]].sort_values('bus_id')