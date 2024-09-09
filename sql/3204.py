# Medium

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `user_permissions`

# +-------------+---------+
# | Column Name | Type    |
# +-------------+---------+
# | user\_id     | int     |
# | permissions | int     |
# +-------------+---------+
# user\_id is the primary key.
# Each row of this table contains the user ID and their permissions encoded as an integer.

# Consider that each bit in the `permissions` integer represents a different access level or feature that a user has.

# Write a solution to calculate the following:

# *   common\_perms: The access level granted to **all users**. This is computed using a **bitwise AND** operation on the `permissions` column.
# *   any\_perms: The access level granted to **any user**. This is computed using a **bitwise OR** operation on the `permissions` column.

# Return _the result table in **any** order_.

# The result format is shown in the following example.

# **Example:**

# **Input:**

# user\_permissions table:

# +---------+-------------+
# | user\_id | permissions |
# +---------+-------------+
# | 1       | 5           |
# | 2       | 12          |
# | 3       | 7           |
# | 4       | 3           |
# +---------+-------------+
 

# **Output:**

# +-------------+--------------+
# | common\_perms | any\_perms   |
# +--------------+-------------+
# | 0            | 15          |
# +--------------+-------------+
    

# **Explanation:**

# *   **common\_perms:** Represents the bitwise AND result of all permissions:
#     *   For user 1 (5): 5 (binary 0101)
#     *   For user 2 (12): 12 (binary 1100)
#     *   For user 3 (7): 7 (binary 0111)
#     *   For user 4 (3): 3 (binary 0011)
#     *   Bitwise AND: 5 & 12 & 7 & 3 = 0 (binary 0000)
# *   **any\_perms:** Represents the bitwise OR result of all permissions:
#     *   Bitwise OR: 5 | 12 | 7 | 3 = 15 (binary 1111)




import pandas as pd

def analyze_permissions(user_permissions: pd.DataFrame) -> pd.DataFrame:

    return pd.DataFrame(
         {'common_perms': [reduce(and_, user_permissions.permissions)],
          'any_perms'   : [reduce(or_ , user_permissions.permissions)]})