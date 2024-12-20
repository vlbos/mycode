# [3358\. Books with NULL Ratings 🔒](https://leetcode.com/problems/books-with-null-ratings)
# ==========================================================================================

# [![](https://img.shields.io/badge/Difficulty-Easy-4051B5?style=flat-square)](https://img.shields.io/badge/Difficulty-Easy-4051B5?style=flat-square)

# Description
# -----------

# Table: `books`

# +----------------+---------+
# | Column Name    | Type    |
# +----------------+---------+
# | book\_id        | int     |
# | title          | varchar |
# | author         | varchar |
# | published\_year | int     |
# | rating         | decimal |
# +----------------+---------+
# book\_id is the unique key for this table.
# Each row of this table contains information about a book including its unique ID, title, author, publication year, and rating.
# rating can be NULL, indicating that the book hasn't been rated yet.

# Write a solution to find all books that have not been rated yet (i.e., have a **NULL** rating).

# Return _the result table_ _ordered by_ `book_id` in **ascending** order.

# The result format is in the following example.

# **Example:**

# **Input:**

# books table:

# +---------+------------------------+------------------+----------------+--------+
# | book\_id | title                  | author           | published\_year | rating |
# +---------+------------------------+------------------+----------------+--------+
# | 1       | The Great Gatsby       | F. Scott         | 1925           | 4.5    |
# | 2       | To Kill a Mockingbird  | Harper Lee       | 1960           | NULL   |
# | 3       | Pride and Prejudice    | Jane Austen      | 1813           | 4.8    |
# | 4       | The Catcher in the Rye | J.D. Salinger    | 1951           | NULL   |
# | 5       | Animal Farm            | George Orwell    | 1945           | 4.2    |
# | 6       | Lord of the Flies      | William Golding  | 1954           | NULL   |
# +---------+------------------------+------------------+----------------+--------+

# **Output:**

# +---------+------------------------+------------------+----------------+
# | book\_id | title                  | author           | published\_year |
# +---------+------------------------+------------------+----------------+
# | 2       | To Kill a Mockingbird  | Harper Lee       | 1960           |
# | 4       | The Catcher in the Rye | J.D. Salinger    | 1951           |
# | 6       | Lord of the Flies      | William Golding  | 1954           |
# +---------+------------------------+------------------+----------------+

# **Explanation:**

# *   The books with book\_id 2, 4, and 6 have NULL ratings.
# *   These books are included in the result table.
# *   The other books (book\_id 1, 3, and 5) have ratings and are not included.

# The result is ordered by book\_id in ascending order


import pandas as pd


def find_unrated_books(books: pd.DataFrame) -> pd.DataFrame:
    unrated_books = books[books["rating"].isnull()]
    return unrated_books[["book_id", "title", "author", "published_year"]].sort_values(
        by="book_id"
    )