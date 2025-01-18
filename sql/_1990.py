# Medium

# Topics

# Companies

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Experiments`

# +-----------------+------+
# | Column Name     | Type |
# +-----------------+------+
# | experiment\_id   | int  |
# | platform        | enum |
# | experiment\_name | enum |
# +-----------------+------+
# experiment\_id is the column with unique values for this table.
# platform is an enum (category) type of values ('Android', 'IOS', 'Web').
# experiment\_name is an enum (category) type of values ('Reading', 'Sports', 'Programming').
# This table contains information about the ID of an experiment done with a random person, the platform used to do the experiment, and the name of the experiment.

# Write a solution to report the **number of experiments** done on each of the three platforms for each of the three given experiments. Notice that all the pairs of (platform, experiment) should be included in the output **including** the pairs with **zero experiments**.

# Return the result table in **any order**.

# The result format is in the following example.

# **Example 1:**

# **Input:**
# Experiments table:
# +---------------+----------+-----------------+
# | experiment\_id | platform | experiment\_name |
# +---------------+----------+-----------------+
# | 4             | IOS      | Programming     |
# | 13            | IOS      | Sports          |
# | 14            | Android  | Reading         |
# | 8             | Web      | Reading         |
# | 12            | Web      | Reading         |
# | 18            | Web      | Programming     |
# +---------------+----------+-----------------+
# **Output:** 
# +----------+-----------------+-----------------+
# | platform | experiment\_name | num\_experiments |
# +----------+-----------------+-----------------+
# | Android  | Reading         | 1               |
# | Android  | Sports          | 0               |
# | Android  | Programming     | 0               |
# | IOS      | Reading         | 0               |
# | IOS      | Sports          | 1               |
# | IOS      | Programming     | 1               |
# | Web      | Reading         | 2               |
# | Web      | Sports          | 0               |
# | Web      | Programming     | 1               |
# +----------+-----------------+-----------------+
# **Explanation:** 
# On the platform "Android", we had only one "Reading" experiment.
# On the platform "IOS", we had one "Sports" experiment and one "Programming" experiment.
# On the platform "Web", we had two "Reading" experiments and one "Programming" experiment.

import pandas as pd

def count_experiments(df: pd.DataFrame) -> pd.DataFrame:
    platforms = pd.DataFrame({'platform': ['Android', 'IOS', 'Web']})
    experiments = pd.DataFrame({'experiment_name': ['Reading', 'Sports', 'Programming']})
    res = platforms.merge(experiments, how='cross')
    res = pd.concat((res, df))
    return (res.groupby(['platform', 'experiment_name']).size() - 1).reset_index(name='num_experiments')