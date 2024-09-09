# Easy

# Topics

# Companies

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `SchoolA`

# +---------------+---------+
# | Column Name   | Type    |
# +---------------+---------+
# | student\_id    | int     |
# | student\_name  | varchar |
# +---------------+---------+
# student\_id is the column with unique values for this table.
# Each row of this table contains the name and the id of a student in school A.
# All student\_name are distinct.

# Table: `SchoolB`

# +---------------+---------+
# | Column Name   | Type    |
# +---------------+---------+
# | student\_id    | int     |
# | student\_name  | varchar |
# +---------------+---------+
# student\_id is the column with unique values for this table.
# Each row of this table contains the name and the id of a student in school B.
# All student\_name are distinct.

# Table: `SchoolC`

# +---------------+---------+
# | Column Name   | Type    |
# +---------------+---------+
# | student\_id    | int     |
# | student\_name  | varchar |
# +---------------+---------+
# student\_id is the column with unique values for this table.
# Each row of this table contains the name and the id of a student in school C.
# All student\_name are distinct.

# There is a country with three schools, where each student is enrolled in **exactly one** school. The country is joining a competition and wants to select one student from each school to represent the country such that:

# *   `member_A` is selected from `SchoolA`,
# *   `member_B` is selected from `SchoolB`,
# *   `member_C` is selected from `SchoolC`, and
# *   The selected students' names and IDs are pairwise distinct (i.e. no two students share the same name, and no two students share the same ID).

# Write a solution to find all the possible triplets representing the country under the given constraints.

# Return the result table in **any order**.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# SchoolA table:
# +------------+--------------+
# | student\_id | student\_name |
# +------------+--------------+
# | 1          | Alice        |
# | 2          | Bob          |
# +------------+--------------+
# SchoolB table:
# +------------+--------------+
# | student\_id | student\_name |
# +------------+--------------+
# | 3          | Tom          |
# +------------+--------------+
# SchoolC table:
# +------------+--------------+
# | student\_id | student\_name |
# +------------+--------------+
# | 3          | Tom          |
# | 2          | Jerry        |
# | 10         | Alice        |
# +------------+--------------+
# **Output:** 
# +----------+----------+----------+
# | member\_A | member\_B | member\_C |
# +----------+----------+----------+
# | Alice    | Tom      | Jerry    |
# | Bob      | Tom      | Alice    |
# +----------+----------+----------+
# **Explanation:** 
# Let us see all the possible triplets.
# - (Alice, Tom, Tom) --> Rejected because member\_B and member\_C have the same name and the same ID.
# - (Alice, Tom, Jerry) --> Valid triplet.
# - (Alice, Tom, Alice) --> Rejected because member\_A and member\_C have the same name.
# - (Bob, Tom, Tom) --> Rejected because member\_B and member\_C have the same name and the same ID.
# - (Bob, Tom, Jerry) --> Rejected because member\_A and member\_C have the same ID.
# - (Bob, Tom, Alice) --> Valid triplet.


import pandas as pd

def find_valid_triplets(school_a: pd.DataFrame, school_b: pd.DataFrame, school_c: pd.DataFrame) -> pd.DataFrame:
    df = (
        school_a.join(school_b, how="cross", lsuffix="_A", rsuffix="_B").join(
            school_c, how="cross"
        )
    ).rename(
        columns={
            "student_name_A": "member_A",
            "student_name_B": "member_B",
            "student_name": "member_C",
        }
    )
    return df[
        (df["member_A"] != df["member_B"])
        & (df["member_B"] != df["member_C"])
        & (df["member_C"] != df["member_A"])
        & (df["student_id_A"] != df["student_id_B"])
        & (df["student_id_B"] != df["student_id"])
        & (df["student_id"] != df["student_id_A"])
    ][["member_A", "member_B", "member_C"]]
