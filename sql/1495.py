# Easy

# Topics

# Companies

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `TVProgram`

# +---------------+---------+
# | Column Name   | Type    |
# +---------------+---------+
# | program\_date  | date    |
# | content\_id    | int     |
# | channel       | varchar |
# +---------------+---------+
# (program\_date, content\_id) is the primary key (combination of columns with unique values) for this table.
# This table contains information of the programs on the TV.
# content\_id is the id of the program in some channel on the TV.

# Table: `Content`

# +------------------+---------+
# | Column Name      | Type    |
# +------------------+---------+
# | content\_id       | varchar |
# | title            | varchar |
# | Kids\_content     | enum    |
# | content\_type     | varchar |
# +------------------+---------+
# content\_id is the primary key (column with unique values) for this table.
# Kids\_content is an ENUM (category) of types ('Y', 'N') where: 
# 'Y' means is content for kids otherwise 'N' is not content for kids.
# content\_type is the category of the content as movies, series, etc.

# Write a solution to report the distinct titles of the kid-friendly movies streamed in **June 2020**.

# Return the result table in **any order**.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# TVProgram table:
# +--------------------+--------------+-------------+
# | program\_date       | content\_id   | channel     |
# +--------------------+--------------+-------------+
# | 2020-06-10 08:00   | 1            | LC-Channel  |
# | 2020-05-11 12:00   | 2            | LC-Channel  |
# | 2020-05-12 12:00   | 3            | LC-Channel  |
# | 2020-05-13 14:00   | 4            | Disney Ch   |
# | 2020-06-18 14:00   | 4            | Disney Ch   |
# | 2020-07-15 16:00   | 5            | Disney Ch   |
# +--------------------+--------------+-------------+
# Content table:
# +------------+----------------+---------------+---------------+
# | content\_id | title          | Kids\_content  | content\_type  |
# +------------+----------------+---------------+---------------+
# | 1          | Leetcode Movie | N             | Movies        |
# | 2          | Alg. for Kids  | Y             | Series        |
# | 3          | Database Sols  | N             | Series        |
# | 4          | Aladdin        | Y             | Movies        |
# | 5          | Cinderella     | Y             | Movies        |
# +------------+----------------+---------------+---------------+
# **Output:** 
# +--------------+
# | title        |
# +--------------+
# | Aladdin      |
# +--------------+
# **Explanation:** 
# "Leetcode Movie" is not a content for kids.
# "Alg. for Kids" is not a movie.
# "Database Sols" is not a movie
# "Alladin" is a movie, content for kids and was streamed in June 2020.
# "Cinderella" was not streamed in June 2020.



import pandas as pd

def friendly_movies(tv_program: pd.DataFrame, content: pd.DataFrame) -> pd.DataFrame:
    df0 = tv_program[(tv_program.program_date.dt.year == 2020) & (tv_program.program_date.dt.month == 6)]
    df1 = content[(content.Kids_content == 'Y') & (content.content_type == 'Movies')]
    df1['content_id'] = df1['content_id'].astype(int)
    df = pd.merge(df0, df1, on = 'content_id', how = 'inner')
    return df[['title']].drop_duplicates()