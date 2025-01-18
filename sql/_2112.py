# Medium

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Flights`

# +-------------------+------+
# | Column Name       | Type |
# +-------------------+------+
# | departure\_airport | int  |
# | arrival\_airport   | int  |
# | flights\_count     | int  |
# +-------------------+------+
# (departure\_airport, arrival\_airport) is the primary key column (combination of columns with unique values) for this table.
# Each row of this table indicates that there were flights\_count flights that departed from departure\_airport and arrived at arrival\_airport.

# Write a solution to report the ID of the airport with the **most traffic**. The airport with the most traffic is the airport that has the largest total number of flights that either departed from or arrived at the airport. If there is more than one airport with the most traffic, report them all.

# Return the result table in **any order**.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Flights table:
# +-------------------+-----------------+---------------+
# | departure\_airport | arrival\_airport | flights\_count |
# +-------------------+-----------------+---------------+
# | 1                 | 2               | 4             |
# | 2                 | 1               | 5             |
# | 2                 | 4               | 5             |
# +-------------------+-----------------+---------------+
# **Output:** 
# +------------+
# | airport\_id |
# +------------+
# | 2          |
# +------------+
# **Explanation:** 
# Airport 1 was engaged with 9 flights (4 departures, 5 arrivals).
# Airport 2 was engaged with 14 flights (10 departures, 4 arrivals).
# Airport 4 was engaged with 5 flights (5 arrivals).
# The airport with the most traffic is airport 2.

# **Example 2:**

# **Input:** 
# Flights table:
# +-------------------+-----------------+---------------+
# | departure\_airport | arrival\_airport | flights\_count |
# +-------------------+-----------------+---------------+
# | 1                 | 2               | 4             |
# | 2                 | 1               | 5             |
# | 3                 | 4               | 5             |
# | 4                 | 3               | 4             |
# | 5                 | 6               | 7             |
# +-------------------+-----------------+---------------+
# **Output:** 
# +------------+
# | airport\_id |
# +------------+
# | 1          |
# | 2          |
# | 3          |
# | 4          |
# +------------+
# **Explanation:** 
# Airport 1 was engaged with 9 flights (4 departures, 5 arrivals).
# Airport 2 was engaged with 9 flights (5 departures, 4 arrivals).
# Airport 3 was engaged with 9 flights (5 departures, 4 arrivals).
# Airport 4 was engaged with 9 flights (4 departures, 5 arrivals).
# Airport 5 was engaged with 7 flights (7 departures).
# Airport 6 was engaged with 7 flights (7 arrivals).
# The airports with the most traffic are airports 1, 2, 3, and 4.



import pandas as pd

def airport_with_most_traffic(flights: pd.DataFrame) -> pd.DataFrame:

    df = pd.concat([flights.rename(columns = {
                                'departure_airport':'airport_id'}),
                    flights.rename(columns = {
                                'arrival_airport': 'airport_id'})]

          ).groupby('airport_id')['flights_count'].sum().reset_index()

    mx = df.flights_count.max()
    return df[df.flights_count == mx][['airport_id']]