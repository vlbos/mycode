# Easy

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `cities`

# +-------------+---------+
# | Column Name | Type    | 
# +-------------+---------+
# | state       | varchar |
# | city        | varchar |
# +-------------+---------+
# (state, city) is the primary key (combination of columns with unique values) for this table.
# Each row of this table contains the state name and the city name within that state.

# Write a solution to find **all the cities in each state** and combine them into a **single comma-separated** string.

# Return _the result table ordered by_ `state` _and_ `city` _in **ascending** order_.

# The result format is in the following example.

# **Example:**

# **Input:**

# cities table:

# +-------------+---------------+
# | state       | city          |
# +-------------+---------------+
# | California  | Los Angeles   |
# | California  | San Francisco |
# | California  | San Diego     |
# | Texas       | Houston       |
# | Texas       | Austin        |
# | Texas       | Dallas        |
# | New York    | New York City |
# | New York    | Buffalo       |
# | New York    | Rochester     |
# +-------------+---------------+

# **Output:**

# +-------------+---------------------------------------+
# | state       | cities                                |
# +-------------+---------------------------------------+
# | California  | Los Angeles, San Diego, San Francisco |
# | New York    | Buffalo, New York City, Rochester     |
# | Texas       | Austin, Dallas, Houston               |
# +-------------+---------------------------------------+

# **Explanation:**

# *   **California:** All cities ("Los Angeles", "San Diego", "San Francisco") are listed in a comma-separated string.
# *   **New York:** All cities ("Buffalo", "New York City", "Rochester") are listed in a comma-separated string.
# *   **Texas:** All cities ("Austin", "Dallas", "Houston") are listed in a comma-separated string.

# **Note:** The output table is ordered by the state name in ascending order.


import pandas as pd

def find_cities(cities: pd.DataFrame) -> pd.DataFrame:
    return (
        cities.sort_values(["state", "city"])
        .groupby("state", as_index=False)
        .agg(cities=("city", ", ".join))
    )