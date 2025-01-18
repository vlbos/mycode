# [3368\. First Letter Capitalization 🔒](https://leetcode.com/problems/first-letter-capitalization)
# ==================================================================================================

# [![](https://img.shields.io/badge/Difficulty-Hard-4051B5?style=flat-square)](https://img.shields.io/badge/Difficulty-Hard-4051B5?style=flat-square)

# Description
# -----------

# Table: `user_content`

# +-------------+---------+
# | Column Name | Type    |
# +-------------+---------+
# | content\_id  | int     |
# | content\_text| varchar |
# +-------------+---------+
# content\_id is the unique key for this table.
# Each row contains a unique ID and the corresponding text content.

# Write a solution to transform the text in the `content_text` column by applying the following rules:

# *   Convert the first letter of each word to uppercase
# *   Keep all other letters in lowercase
# *   Preserve all existing spaces

# **Note**: There will be no special character in `content_text`.

# Return _the result table that includes both the original `content_text` and the modified text where each word starts with a capital letter_.

# The result format is in the following example.

# **Example:**

# **Input:**

# user\_content table:

# +------------+-----------------------------------+
# | content\_id | content\_text                      |
# +------------+-----------------------------------+
# | 1          | hello world of SQL                |
# | 2          | the QUICK brown fox               |
# | 3          | data science AND machine learning |
# | 4          | TOP rated programming BOOKS       |
# +------------+-----------------------------------+

# **Output:**

# +------------+-----------------------------------+-----------------------------------+
# | content\_id | original\_text                     | converted\_text                    |
# +------------+-----------------------------------+-----------------------------------+
# | 1          | hello world of SQL                | Hello World Of SQL                |
# | 2          | the QUICK brown fox               | The Quick Brown Fox               |
# | 3          | data science AND machine learning | Data Science And Machine Learning |
# | 4          | TOP rated programming BOOKS       | Top Rated Programming Books       |
# +------------+-----------------------------------+-----------------------------------+

# **Explanation:**

# *   For content\_id = 1:
#     *   Each word's first letter is capitalized: Hello World Of SQL
# *   For content\_id = 2:
#     *   Original mixed-case text is transformed to title case: The Quick Brown Fox
# *   For content\_id = 3:
#     *   The word AND is converted to "And": "Data Science And Machine Learning"
# *   For content\_id = 4:
#     *   Handles word TOP rated correctly: Top Rated
#     *   Converts BOOKS from all caps to title case: Books


import pandas as pd


def process_text(user_content: pd.DataFrame) -> pd.DataFrame:
    user_content["converted_text"] = user_content["content_text"].apply(
        lambda x: x.title())
    return user_content.rename(
        columns={"content_text": "original_text"}
    )


import pandas as pd


def process_text(user_content: pd.DataFrame) -> pd.DataFrame:
    user_content["converted_text"] = user_content["content_text"].apply(
        lambda text: " ".join(word.capitalize() for word in text.split(" "))
    )
    return user_content[["content_id", "content_text", "converted_text"]].rename(
        columns={"content_text": "original_text"}
    )