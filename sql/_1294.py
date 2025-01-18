# Easy

# Topics

# Companies

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Countries`

# +---------------+---------+
# | Column Name   | Type    |
# +---------------+---------+
# | country\_id    | int     |
# | country\_name  | varchar |
# +---------------+---------+
# country\_id is the primary key (column with unique values) for this table.
# Each row of this table contains the ID and the name of one country.

# Table: `Weather`

# +---------------+------+
# | Column Name   | Type |
# +---------------+------+
# | country\_id    | int  |
# | weather\_state | int  |
# | day           | date |
# +---------------+------+
# (country\_id, day) is the primary key (combination of columns with unique values) for this table.
# Each row of this table indicates the weather state in a country for one day.

# Write a solution to find the type of weather in each country for **November 2019**.

# The type of weather is:

# *   **Cold** if the average `weather_state` is less than or equal `15`,
# *   **Hot** if the average `weather_state` is greater than or equal to `25`, and
# *   **Warm** otherwise.

# Return the result table in **any order**.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Countries table:
# +------------+--------------+
# | country\_id | country\_name |
# +------------+--------------+
# | 2          | USA          |
# | 3          | Australia    |
# | 7          | Peru         |
# | 5          | China        |
# | 8          | Morocco      |
# | 9          | Spain        |
# +------------+--------------+
# Weather table:
# +------------+---------------+------------+
# | country\_id | weather\_state | day        |
# +------------+---------------+------------+
# | 2          | 15            | 2019-11-01 |
# | 2          | 12            | 2019-10-28 |
# | 2          | 12            | 2019-10-27 |
# | 3          | -2            | 2019-11-10 |
# | 3          | 0             | 2019-11-11 |
# | 3          | 3             | 2019-11-12 |
# | 5          | 16            | 2019-11-07 |
# | 5          | 18            | 2019-11-09 |
# | 5          | 21            | 2019-11-23 |
# | 7          | 25            | 2019-11-28 |
# | 7          | 22            | 2019-12-01 |
# | 7          | 20            | 2019-12-02 |
# | 8          | 25            | 2019-11-05 |
# | 8          | 27            | 2019-11-15 |
# | 8          | 31            | 2019-11-25 |
# | 9          | 7             | 2019-10-23 |
# | 9          | 3             | 2019-12-23 |
# +------------+---------------+------------+
# **Output:** 
# +--------------+--------------+
# | country\_name | weather\_type |
# +--------------+--------------+
# | USA          | Cold         |
# | Australia    | Cold         |
# | Peru         | Hot          |
# | Morocco      | Hot          |
# | China        | Warm         |
# +--------------+--------------+
# **Explanation:** 
# Average weather\_state in USA in November is (15) / 1 = 15 so weather type is Cold.
# Average weather\_state in Austraila in November is (-2 + 0 + 3) / 3 = 0.333 so weather type is Cold.
# Average weather\_state in Peru in November is (25) / 1 = 25 so the weather type is Hot.
# Average weather\_state in China in November is (16 + 18 + 21) / 3 = 18.333 so weather type is Warm.
# Average weather\_state in Morocco in November is (25 + 27 + 31) / 3 = 27.667 so weather type is Hot.
# We know nothing about the average weather\_state in Spain in November so we do not include it in the result table.




import pandas as pd

def weather_type(countries: pd.DataFrame, weather: pd.DataFrame) -> pd.DataFrame:
    df = weather[(weather['day'].dt.year == 2019) & (weather['day'].dt.month == 11)].groupby('country_id')['weather_state'].mean().reset_index()
    df['weather_type'] = np.select(
                                    [df['weather_state'].to_numpy() <= 15, 
                                     df['weather_state'].to_numpy() >= 25]
                                  , ['Cold', 
                                     'Hot']
                                  , default = 'Warm')

    return countries.merge(df, on = 'country_id', how = 'right').reset_index()[['country_name', 'weather_type']]