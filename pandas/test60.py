# 60. Get Last n Records

# Write a Pandas program to get last n records of a DataFrame.

# Sample Solution :

# Python Code :

# import pandas as pd
# d = {'col1': [1, 2, 3, 4, 7, 11], 'col2': [4, 5, 6, 9, 5, 0], 'col3': [7, 5, 8, 12, 1,11]}
# df = pd.DataFrame(data=d)
# print("Original DataFrame")
# print(df)
# print("\nLast 3 rows of the said DataFrame':")
# df1 = df.tail(3)
# print(df1)
# Sample Output:

# Original DataFrame
#    col1  col2  col3
# 0     1     4     7
# 1     2     5     5
# 2     3     6     8
# 3     4     9    12
# 4     7     5     1
# 5    11     0    11

# Last 3 rows of the said DataFrame':
#    col1  col2  col3
# 3     4     9    12
# 4     7     5     1
# 5    11     0    11


# 61. Get Topmost n Records Within Each Group

# Write a Pandas program to get topmost n records within each group of a DataFrame.

# Sample Solution :

# Python Code :

# import pandas as pd
# d = {'col1': [1, 2, 3, 4, 7, 11], 'col2': [4, 5, 6, 9, 5, 0], 'col3': [7, 5, 8, 12, 1,11]}
# df = pd.DataFrame(data=d)
# print("Original DataFrame")
# print(df)
# print("\ntopmost n records within each group of a DataFrame:")
# df1 = df.nlargest(3, 'col1')
# print(df1)
# df2 = df.nlargest(3, 'col2')
# print(df2)
# df3 = df.nlargest(3, 'col3')
# print(df3)
# Sample Output:

# Original DataFrame
#    col1  col2  col3
# 0     1     4     7
# 1     2     5     5
# 2     3     6     8
# 3     4     9    12
# 4     7     5     1
# 5    11     0    11

# topmost n records within each group of a DataFrame:
#    col1  col2  col3
# 5    11     0    11
# 4     7     5     1
# 3     4     9    12
#    col1  col2  col3
# 3     4     9    12
# 2     3     6     8
# 1     2     5     5
# 4     7     5     1
#    col1  col2  col3
# 3     4     9    12
# 5    11     0    11
# 2     3     6     8


# 62. Remove First n Rows

# Write a Pandas program to remove first n rows of a given DataFrame.

# Sample Solution :

# Python Code :

# import pandas as pd
# d = {'col1': [1, 2, 3, 4, 7, 11], 'col2': [4, 5, 6, 9, 5, 0], 'col3': [7, 5, 8, 12, 1,11]}
# df = pd.DataFrame(data=d)
# print("Original DataFrame")
# print(df)
# print("\nAfter removing first 3 rows of the said DataFrame:")
# df1 = df.iloc[3:]
# print(df1)
# Sample Output:

# Original DataFrame
#    col1  col2  col3
# 0     1     4     7
# 1     2     5     5
# 2     3     6     8
# 3     4     9    12
# 4     7     5     1
# 5    11     0    11

# After removing first 3 rows of the said DataFrame:
#    col1  col2  col3
# 3     4     9    12
# 4     7     5     1
# 5    11     0    11


# 63. Remove Last n Rows

# Write a Pandas program to remove last n rows of a given DataFrame.

# Sample Solution :

# Python Code :

# import pandas as pd
# d = {'col1': [1, 2, 3, 4, 7, 11], 'col2': [4, 5, 6, 9, 5, 0], 'col3': [7, 5, 8, 12, 1,11]}
# df = pd.DataFrame(data=d)
# print("Original DataFrame")
# print(df)
# print("\nAfter removing last 3 rows of the said DataFrame:")
# df1 = df.iloc[:3]
# print(df1)
# Sample Output:

# Original DataFrame
#    col1  col2  col3
# 0     1     4     7
# 1     2     5     5
# 2     3     6     8
# 3     4     9    12
# 4     7     5     1
# 5    11     0    11

# After removing last 3 rows of the said DataFrame:
#    col1  col2  col3
# 0     1     4     7
# 1     2     5     5
# 2     3     6     8


# 64. Add Prefix or Suffix to All Columns

# Write a Pandas program to add a prefix or suffix to all columns of a given DataFrame.

# Sample Solution :

# Python Code :

# import pandas as pd
# df = pd.DataFrame({'W':[68,75,86,80,66],'X':[78,85,96,80,86], 'Y':[84,94,89,83,86],'Z':[86,97,96,72,83]});
# print("Original DataFrame")
# print(df)
# print("\nAdd prefix:")
# print(df.add_prefix("A_"))
# print("\nAdd suffix:")
# print(df.add_suffix("_1"))
# Sample Output:

# Original DataFrame
#     W   X   Y   Z
# 0  68  78  84  86
# 1  75  85  94  97
# 2  86  96  89  96
# 3  80  80  83  72
# 4  66  86  86  83

# Add prefix:
#    A_W  A_X  A_Y  A_Z
# 0   68   78   84   86
# 1   75   85   94   97
# 2   86   96   89   96
# 3   80   80   83   72
# 4   66   86   86   83

# Add suffix:
#    W_1  X_1  Y_1  Z_1
# 0   68   78   84   86
# 1   75   85   94   97
# 2   86   96   89   96
# 3   80   80   83   72
# 4   66   86   86   83


# 65. Reverse Order of DataFrame (Rows, Columns)

# Write a Pandas program to reverse order (rows, columns) of a given DataFrame.

# Sample Solution :

# Python Code :

# import pandas as pd
# df = pd.DataFrame({'W':[68,75,86,80,66],'X':[78,85,96,80,86], 'Y':[84,94,89,83,86],'Z':[86,97,96,72,83]});
# print("Original DataFrame")
# print(df)
# print("\nReverse column order:")
# print(df.loc[:, ::-1])
# print("\nReverse row order:")
# print(df.loc[::-1])
# print("\nReverse row order and reset index:")
# print(df.loc[::-1].reset_index(drop = True))
# Sample Output:

# Original DataFrame
#     W   X   Y   Z
# 0  68  78  84  86
# 1  75  85  94  97
# 2  86  96  89  96
# 3  80  80  83  72
# 4  66  86  86  83

# Reverse column order:
#     Z   Y   X   W
# 0  86  84  78  68
# 1  97  94  85  75
# 2  96  89  96  86
# 3  72  83  80  80
# 4  83  86  86  66

# Reverse row order:
#     W   X   Y   Z
# 4  66  86  86  83
# 3  80  80  83  72
# 2  86  96  89  96
# 1  75  85  94  97
# 0  68  78  84  86

# Reverse row order and reset index:
#     W   X   Y   Z
# 0  66  86  86  83
# 1  80  80  83  72
# 2  86  96  89  96
# 3  75  85  94  97
# 4  68  78  84  86


# 66. Select Columns by Data Type

# Write a Pandas program to select columns by data type of a given DataFrame.

# Sample Solution :

# Python Code :

# import pandas as pd
# df = pd.DataFrame({
#     'name': ['Alberto Franco','Gino Mcneill','Ryan Parkes', 'Eesha Hinton', 'Syed Wharton'],
#     'date_of_birth': ['17/05/2002','16/02/1999','25/09/1998','11/05/2002','15/09/1997'],
#     'age': [18.5, 21.2, 22.5, 22, 23]
# })

# print("Original DataFrame")
# print(df)
# print("\nSelect numerical columns")
# print(df.select_dtypes(include = "number"))
# print("\nSelect string columns")
# print(df.select_dtypes(include = "object"))
# Sample Output:

# Original DataFrame
#              name date_of_birth   age
# 0  Alberto Franco    17/05/2002  18.5
# 1    Gino Mcneill    16/02/1999  21.2
# 2     Ryan Parkes    25/09/1998  22.5
# 3    Eesha Hinton    11/05/2002  22.0
# 4    Syed Wharton    15/09/1997  23.0

# Select numerical columns
#     age
# 0  18.5
# 1  21.2
# 2  22.5
# 3  22.0
# 4  23.0

# Select string columns
#              name date_of_birth
# 0  Alberto Franco    17/05/2002
# 1    Gino Mcneill    16/02/1999
# 2     Ryan Parkes    25/09/1998
# 3    Eesha Hinton    11/05/2002
# 4    Syed Wharton    15/09/1997


# 67. Split DataFrame into Two Random Subsets

# Write a Pandas program to split a given DataFrame into two random subsets.

# Sample Solution :

# Python Code :

# import pandas as pd
# df = pd.DataFrame({
#     'name': ['Alberto Franco','Gino Mcneill','Ryan Parkes', 'Eesha Hinton', 'Syed Wharton'],
#     'date_of_birth': ['17/05/2002','16/02/1999','25/09/1998','11/05/2002','15/09/1997'],
#     'age': ['18', '21', '22', '22', '23']
# })

# df_1 = df.sample(frac = 0.6)
# df_2 = df.drop(df_1.index)
# print("Original Dataframe and shape:")
# print(df)
# print(df.shape)
# print("\nSubset-1 and shape:")
# print(df_1)
# print(df_1.shape)
# print("\nSubset-2 and shape:")
# print(df_2)
# print(df_2.shape)
# Sample Output:

# Original Dataframe and shape:
#              name date_of_birth age
# 0  Alberto Franco    17/05/2002  18
# 1    Gino Mcneill    16/02/1999  21
# 2     Ryan Parkes    25/09/1998  22
# 3    Eesha Hinton    11/05/2002  22
# 4    Syed Wharton    15/09/1997  23
# (5, 3)

# Subset-1 and shape:
#            name date_of_birth age
# 1  Gino Mcneill    16/02/1999  21
# 4  Syed Wharton    15/09/1997  23
# 2   Ryan Parkes    25/09/1998  22
# (3, 3)

# Subset-2 and shape:
#              name date_of_birth age
# 0  Alberto Franco    17/05/2002  18
# 3    Eesha Hinton    11/05/2002  22
# (2, 3)


# 68. Rename All Columns with Same Pattern

# Write a Pandas program to rename all columns with the same pattern of a given DataFrame.

# Sample Solution :

# Python Code :

# import pandas as pd
# df = pd.DataFrame({
#     'Name': ['Alberto Franco','Gino Mcneill','Ryan Parkes', 'Eesha Hinton', 'Syed Wharton'],
#     'Date_Of_Birth ': ['17/05/2002','16/02/1999','25/09/1998','11/05/2002','15/09/1997'],
#     'Age': [18.5, 21.2, 22.5, 22, 23]
# })

# print("Original DataFrame")
# print(df)
# df.columns = df.columns.str.lower().str.rstrip()
# print("\nRemove trailing (at the end) whitesapce and convert to lowercase of the columns name")
# print(df.head())
# Sample Output:

# Original DataFrame
#              Name Date_Of_Birth    Age
# 0  Alberto Franco     17/05/2002  18.5
# 1    Gino Mcneill     16/02/1999  21.2
# 2     Ryan Parkes     25/09/1998  22.5
# 3    Eesha Hinton     11/05/2002  22.0
# 4    Syed Wharton     15/09/1997  23.0

# Remove trailing (at the end) whitesapce and convert to lowercase of the columns name
#              name date_of_birth   age
# 0  Alberto Franco    17/05/2002  18.5
# 1    Gino Mcneill    16/02/1999  21.2
# 2     Ryan Parkes    25/09/1998  22.5
# 3    Eesha Hinton    11/05/2002  22.0
# 4    Syed Wharton    15/09/1997  23.0


# 69. Merge Datasets and Check Uniqueness

# Write a Pandas program to merge datasets and check uniqueness.

# Sample Solution :

# Python Code :

# import pandas as pd
# df = pd.DataFrame({
#     'Name': ['Alberto Franco','Gino Mcneill','Ryan Parkes', 'Eesha Hinton', 'Syed Wharton'],
#     'Date_Of_Birth ': ['17/05/2002','16/02/1999','25/09/1998','11/05/2002','15/09/1997'],
#     'Age': [18.5, 21.2, 22.5, 22, 23]
# })
# print("Original DataFrame:")
# print(df)
# df1 = df.copy(deep = True)
# df = df.drop([0, 1])
# df1 = df1.drop([2])
# print("\nNew DataFrames:")
# print(df)
# print(df1)
# print('\n"one_to_one”: check if merge keys are unique in both left and right datasets:"')
# df_one_to_one = pd.merge(df, df1, validate = "one_to_one")
# print(df_one_to_one)
# print('\n"one_to_many” or “1:m”: check if merge keys are unique in left dataset:')
# df_one_to_many = pd.merge(df, df1, validate = "one_to_many")
# print(df_one_to_many)
# print('“many_to_one” or “m:1”: check if merge keys are unique in right dataset:')
# df_many_to_one = pd.merge(df, df1, validate = "many_to_one")
# print(df_many_to_one)
# Sample Output:

# Original DataFrame:
#              Name Date_Of_Birth    Age
# 0  Alberto Franco     17/05/2002  18.5
# 1    Gino Mcneill     16/02/1999  21.2
# 2     Ryan Parkes     25/09/1998  22.5
# 3    Eesha Hinton     11/05/2002  22.0
# 4    Syed Wharton     15/09/1997  23.0

# New DataFrames:
#            Name Date_Of_Birth    Age
# 2   Ryan Parkes     25/09/1998  22.5
# 3  Eesha Hinton     11/05/2002  22.0
# 4  Syed Wharton     15/09/1997  23.0
#              Name Date_Of_Birth    Age
# 0  Alberto Franco     17/05/2002  18.5
# 1    Gino Mcneill     16/02/1999  21.2
# 3    Eesha Hinton     11/05/2002  22.0
# 4    Syed Wharton     15/09/1997  23.0

# "one_to_one”: check if merge keys are unique in both left and right datasets:"
#            Name Date_Of_Birth    Age
# 0  Eesha Hinton     11/05/2002  22.0
# 1  Syed Wharton     15/09/1997  23.0

# "one_to_many” or “1:m”: check if merge keys are unique in left dataset:
#            Name Date_Of_Birth    Age
# 0  Eesha Hinton     11/05/2002  22.0
# 1  Syed Wharton     15/09/1997  23.0
# “many_to_one” or “m:1”: check if merge keys are unique in right dataset:
#            Name Date_Of_Birth    Age
# 0  Eesha Hinton     11/05/2002  22.0
# 1  Syed Wharton     15/09/1997  23.0


