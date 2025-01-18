# Medium

# Topics

# Companies

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Rides`

# +--------------+------+
# | Column Name  | Type |
# +--------------+------+
# | ride\_id      | int  |
# | driver\_id    | int  |
# | passenger\_id | int  |
# +--------------+------+
# ride\_id is the column with unique values for this table.
# Each row of this table contains the ID of the driver and the ID of the passenger that rode in ride\_id.
# Note that driver\_id != passenger\_id.

# Write a solution to report the ID of each driver and the number of times they were a passenger.

# Return the result table in **any order**.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Rides table:
# +---------+-----------+--------------+
# | ride\_id | driver\_id | passenger\_id |
# +---------+-----------+--------------+
# | 1       | 7         | 1            |
# | 2       | 7         | 2            |
# | 3       | 11        | 1            |
# | 4       | 11        | 7            |
# | 5       | 11        | 7            |
# | 6       | 11        | 3            |
# +---------+-----------+--------------+
# **Output:** 
# +-----------+-----+
# | driver\_id | cnt |
# +-----------+-----+
# | 7         | 2   |
# | 11        | 0   |
# +-----------+-----+
# **Explanation:** 
# There are two drivers in all the given rides: 7 and 11.
# The driver with ID = 7 was a passenger two times.
# The driver with ID = 11 was never a passenger.




import pandas as pd

def driver_passenger(rides: pd.DataFrame) -> pd.DataFrame:
  return rides.merge(rides, left_on='driver_id', right_on='passenger_id', how='left').groupby('driver_id_x')[['ride_id_y']].nunique().reset_index().rename(columns={'driver_id_x': 'driver_id', 'ride_id_y': 'cnt'})
    