# Easy

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Tweets`

# +----------------+---------+
# | Column Name    | Type    |
# +----------------+---------+
# | tweet\_id       | int     |
# | content        | varchar |
# +----------------+---------+
# tweet\_id is the primary key (column with unique values) for this table.
# This table contains all the tweets in a social media app.

# Write a solution to find **invalid tweets**. A tweet is considered invalid if it meets **any** of the following criteria:

# *   It exceeds `140` characters in length.
# *   It has more than `3` mentions.
# *   It includes more than `3` hashtags.

# Return _the result table ordered by_ `tweet_id` _in **ascending** order_.

# The result format is in the following example.

# **Example:**

# **Input:**

# Tweets table:

#   +----------+-----------------------------------------------------------------------------------+
#   | tweet\_id | content                                                                           |
#   +----------+-----------------------------------------------------------------------------------+
#   | 1        | Traveling, exploring, and living my best life @JaneSmith @SaraJohnson @LisaTaylor |
#   |          | @MikeBrown #Foodie #Fitness #Learning                                             | 
#   | 2        | Just had the best dinner with friends! #Foodie #Friends #Fun                      |
#   | 4        | Working hard on my new project #Work #Goals #Productivity #Fun                    |
#   +----------+-----------------------------------------------------------------------------------+
  

# **Output:**

#   +----------+
#   | tweet\_id |
#   +----------+
#   | 1        |
#   | 4        |
#   +----------+
  

# **Explanation:**

# *   tweet\_id 1 contains 4 mentions.
# *   tweet\_id 4 contains 4 hashtags.

# Output table is ordered by tweet\_id in ascending order.




import pandas as pd

def find_invalid_tweets(tweets: pd.DataFrame) -> pd.DataFrame:
    return tweets.loc[
        (tweets["content"].str.len() > 140) |
        (tweets["content"].str.count("@") > 3) |
        (tweets["content"].str.count("#") > 3),
        ["tweet_id"]
    ].sort_values(by="tweet_id", ascending=True)