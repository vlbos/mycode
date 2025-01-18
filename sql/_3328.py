# [3328\. Find Cities in Each State II 🔒](https://leetcode.com/problems/find-cities-in-each-state-ii)
# ====================================================================================================

# [![](https://img.shields.io/badge/Difficulty-Medium-4051B5?style=flat-square)](https://img.shields.io/badge/Difficulty-Medium-4051B5?style=flat-square)

# Description
# -----------

# Table: `cities`

# +-------------+---------+
# | Column Name | Type    | 
# +-------------+---------+
# | state       | varchar |
# | city        | varchar |
# +-------------+---------+
# (state, city) is the combination of columns with unique values for this table.
# Each row of this table contains the state name and the city name within that state.

# Write a solution to find **all the cities** in **each state** and analyze them based on the following requirements:

# *   Combine all cities into a **comma-separated** string for each state.
# *   Only include states that have **at least** `3` cities.
# *   Only include states where **at least one city** starts with the **same letter as the state name**.

# Return _the result table ordered by_ _the count of matching-letter cities in **descending** order_ _and then by state name in **ascending** order_.

# The result format is in the following example.

# **Example:**

# **Input:**

# cities table:

# +--------------+---------------+
# | state        | city          |
# +--------------+---------------+
# | New York     | New York City |
# | New York     | Newark        |
# | New York     | Buffalo       |
# | New York     | Rochester     |
# | California   | San Francisco |
# | California   | Sacramento    |
# | California   | San Diego     |
# | California   | Los Angeles   |
# | Texas        | Tyler         |
# | Texas        | Temple        |
# | Texas        | Taylor        |
# | Texas        | Dallas        |
# | Pennsylvania | Philadelphia  |
# | Pennsylvania | Pittsburgh    |
# | Pennsylvania | Pottstown     |
# +--------------+---------------+

# **Output:**

# +-------------+-------------------------------------------+-----------------------+
# | state       | cities                                    | matching\_letter\_count |
# +-------------+-------------------------------------------+-----------------------+
# | Pennsylvania| Philadelphia, Pittsburgh, Pottstown       | 3                     |
# | Texas       | Dallas, Taylor, Temple, Tyler             | 3                     |
# | New York    | Buffalo, Newark, New York City, Rochester | 2                     |
# +-------------+-------------------------------------------+-----------------------+

# **Explanation:**

# *   **Pennsylvania**:
#     *   Has 3 cities (meets minimum requirement)
#     *   All 3 cities start with 'P' (same as state)
#     *   matching\_letter\_count = 3
# *   **Texas**:
#     *   Has 4 cities (meets minimum requirement)
#     *   3 cities (Taylor, Temple, Tyler) start with 'T' (same as state)
#     *   matching\_letter\_count = 3
# *   **New York**:
#     *   Has 4 cities (meets minimum requirement)
#     *   2 cities (Newark, New York City) start with 'N' (same as state)
#     *   matching\_letter\_count = 2
# *   **California** is not included in the output because:
#     *   Although it has 4 cities (meets minimum requirement)
#     *   No cities start with 'C' (doesn't meet the matching letter requirement)

# **Note:**

# *   Results are ordered by matching\_letter\_count in descending order
# *   When matching\_letter\_count is the same (Texas and New York both have 2), they are ordered by state name alphabetically
# *   Cities in each row are ordered alphabetically




import pandas as pd


def state_city_analysis(cities: pd.DataFrame) -> pd.DataFrame:
    cities["matching_letter"] = cities["city"].str[0] == cities["state"].str[0]

    result = (
        cities.groupby("state")
        .agg(
            cities=("city", lambda x: ", ".join(sorted(x))),
            matching_letter_count=("matching_letter", "sum"),
            city_count=("city", "count"),
        )
        .reset_index()
    )

    result = result[(result["city_count"] >= 3) & (result["matching_letter_count"] > 0)]

    result = result.sort_values(
        by=["matching_letter_count", "state"], ascending=[False, True]
    )

    result = result.drop(columns=["city_count"])

    return result