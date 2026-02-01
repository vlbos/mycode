# 50. Sort DataFrame by Multiple Columns

# Write a Pandas program to sort a given DataFrame by two or more columns.

# Sample Solution :

# Python Code :

# import pandas as pd
# import numpy as np
# exam_data = {'name': ['Anastasia', 'Dima', 'Katherine', 'James', 'Emily', 'Michael', 'Matthew', 'Laura', 'Kevin', 'Jonas'],
#         'score': [12.5, 9, 16.5, np.nan, 9, 20, 14.5, np.nan, 8, 19],
#         'attempts': [1, 3, 2, 3, 2, 3, 1, 1, 2, 1],
#         'qualify': ['yes', 'no', 'yes', 'no', 'no', 'yes', 'yes', 'no', 'no', 'yes']}
# df = pd.DataFrame(exam_data)
# print("Original DataFrame:")
# print(df)
# print("\nSort the above DataFrame on attempts, name:")
# df = df.sort_values(['attempts', 'name'], ascending=[True, True])
# print(df)
# Sample Output:

# Original DataFrame:
#    attempts       name qualify  score
# 0         1  Anastasia     yes   12.5
# 1         3       Dima      no    9.0
# 2         2  Katherine     yes   16.5
# 3         3      James      no    NaN
# 4         2      Emily      no    9.0
# 5         3    Michael     yes   20.0
# 6         1    Matthew     yes   14.5
# 7         1      Laura      no    NaN
# 8         2      Kevin      no    8.0
# 9         1      Jonas     yes   19.0

# Sort the above DataFrame on attempts, name:
#    attempts       name qualify  score
# 0         1  Anastasia     yes   12.5
# 9         1      Jonas     yes   19.0
# 7         1      Laura      no    NaN
# 6         1    Matthew     yes   14.5
# 4         2      Emily      no    9.0
# 2         2  Katherine     yes   16.5
# 8         2      Kevin      no    8.0
# 1         3       Dima      no    9.0
# 3         3      James      no    NaN
# 5         3    Michael     yes   20.0            
# Explanation:

# The above code creates a Pandas DataFrame df using the dictionary exam_data. The DataFrame has four columns named name, score, attempts, and qualify.

# df = df.sort_values(['attempts', 'name'], ascending=[True, True]): Here the sort_values() method is used to sort the DataFrame based on two columns ‘attempts’ and ‘name’. The ascending parameter is set to [True, True] to indicate that the sorting should be done in ascending order for both columns. This will result in the DataFrame being sorted first by the ‘attempts’ column in ascending order, and then within each group of attempts, the ‘name‘ column will be sorted in ascending order as well. The sorted DataFrame is stored back into ‘df’.

# Finally print() function prints the ‘df’ DataFrame.

# 51. Convert Column DataType

# Write a Pandas program to convert the datatype of a given column(floats to ints).

# Sample Solution :

# Python Code :

# import pandas as pd
# import numpy as np
# exam_data = {'name': ['Anastasia', 'Dima', 'Katherine', 'James', 'Emily', 'Michael', 'Matthew', 'Laura', 'Kevin', 'Jonas'],
#         'score': [12.5, 9.1, 16.5, 12.77, 9.21, 20.22, 14.5, 11.34, 8.8, 19.13],
#         'attempts': [1, 3, 2, 3, 2, 3, 1, 1, 2, 1],
#         'qualify': ['yes', 'no', 'yes', 'no', 'no', 'yes', 'yes', 'no', 'no', 'yes']}
# df = pd.DataFrame(exam_data)
# print("Original DataFrame:")
# print(df)
# print("\nData types of the columns of the said DataFrame:")
# print(df.dtypes)
# print("\nNow change the Data type of 'score' column from float to int:")
# df.score = df.score.astype(int)
# print(df)
# print("\nData types of the columns of the DataFrame now:")
# print(df.dtypes)
# Sample Output:

# Original DataFrame:
#    attempts       name qualify  score
# 0         1  Anastasia     yes  12.50
# 1         3       Dima      no   9.10
# 2         2  Katherine     yes  16.50
# 3         3      James      no  12.77
# 4         2      Emily      no   9.21
# 5         3    Michael     yes  20.22
# 6         1    Matthew     yes  14.50
# 7         1      Laura      no  11.34
# 8         2      Kevin      no   8.80
# 9         1      Jonas     yes  19.13

# Data types of the columns of the said DataFrame:
# attempts      int64
# name         object
# qualify      object
# score       float64
# dtype: object

# Now change the Data type of 'score' column from float to int:
#    attempts       name qualify  score
# 0         1  Anastasia     yes     12
# 1         3       Dima      no      9
# 2         2  Katherine     yes     16
# 3         3      James      no     12
# 4         2      Emily      no      9
# 5         3    Michael     yes     20
# 6         1    Matthew     yes     14
# 7         1      Laura      no     11
# 8         2      Kevin      no      8
# 9         1      Jonas     yes     19

# Data types of the columns of the DataFrame now:
# attempts     int64
# name        object
# qualify     object
# score        int64
# dtype: object      


# 52. Remove Infinite Values

# Write a Pandas program to remove infinite values from a given DataFrame.

# Sample Solution :

# Python Code :

# import pandas as pd
# import numpy as np
# df = pd.DataFrame([1000, 2000, 3000, -4000, np.inf, -np.inf])
# print("Original DataFrame:")
# print(df)
# print("Removing infinite values:")
# df = df.replace([np.inf, -np.inf], np.nan)
# print(df)
# Sample Output:

# Original DataFrame:
#              0
# 0  1000.000000
# 1  2000.000000
# 2  3000.000000
# 3 -4000.000000
# 4          inf
# 5         -inf
# Removing infinite values:
#         0
# 0  1000.0
# 1  2000.0
# 2  3000.0
# 3 -4000.0
# 4     NaN
# 5     NaN           
# 53. Insert Column at Specific Index

# Write a Pandas program to insert a given column at a specific column index in a DataFrame.

# Sample Solution :

# Python Code :

# import pandas as pd
# d = {'col2': [4, 5, 6, 9, 5], 'col3': [7, 8, 12, 1, 11]}
# df = pd.DataFrame(data=d)
# print("Original DataFrame")
# print(df)
# new_col = [1, 2, 3, 4, 7]  
# # insert the said column at the beginning in the DataFrame
# idx = 0
# df.insert(loc=idx, column='col1', value=new_col)
# print("\nNew DataFrame")
# print(df)
# Sample Output:

# Original DataFrame
#    col2  col3
# 0     4     7
# 1     5     8
# 2     6    12
# 3     9     1
# 4     5    11

# New DataFrame
#    col1  col2  col3
# 0     1     4     7
# 1     2     5     8
# 2     3     6    12
# 3     4     9     1
# 4     7     5    11       
# 54. Convert List of Lists into DataFrame

# Write a Pandas program to convert a given list of lists into a Dataframe.

# Sample Solution :

# Python Code :

# import pandas as pd
# my_lists = [['col1', 'col2'], [2, 4], [1, 3]]
# # sets the headers as list
# headers = my_lists.pop(0) 
# print("Original list of lists:")
# print(my_lists)
# df = pd.DataFrame(my_lists, columns = headers)
# print("New DataFrame")
# print(df)
# Sample Output:

# Original list of lists:
# [[2, 4], [1, 3]]
# New DataFrame
#    col1  col2
# 0     2     4
# 1     1     3    



# 55. Group by First Column to Get Lists

# Write a Pandas program to group by the first column and get second column as lists in rows.

# Sample Solution :

# Python Code :

# import pandas as pd
# df = pd.DataFrame( {'col1':['C1','C1','C2','C2','C2','C3','C2'], 'col2':[1,2,3,3,4,6,5]})
# print("Original DataFrame")
# print(df)
# df = df.groupby('col1')['col2'].apply(list)
# print("\nGroup on the col1:")
# print(df)
# Sample Output:

# Original DataFrame
#   col1  col2
# 0   C1     1
# 1   C1     2
# 2   C2     3
# 3   C2     3
# 4   C2     4
# 5   C3     6
# 6   C2     5

# Group on the col1:
# col1
# C1          [1, 2]
# C2    [3, 3, 4, 5]
# C3             [6]
# Name: col2, dtype: object       



# 56. Get Column Index by Column Name

# Write a Pandas program to get column index from column name of a given DataFrame.

# Sample Solution :

# Python Code :

# import pandas as pd
# d = {'col1': [1, 2, 3, 4, 7], 'col2': [4, 5, 6, 9, 5], 'col3': [7, 8, 12, 1, 11]}
# df = pd.DataFrame(data=d)
# print("Original DataFrame")
# print(df)
# print("\nIndex of 'col2'")
# print(df.columns.get_loc("col2"))
# Sample Output:

# Original DataFrame
#    col1  col2  col3
# 0     1     4     7
# 1     2     5     8
# 2     3     6    12
# 3     4     9     1
# 4     7     5    11

# Index of 'col2'
# 1
# 57. Count Number of Columns

# Write a Pandas program to count number of columns of a DataFrame.

# Sample Solution :

# Python Code :

# import pandas as pd
# d = {'col1': [1, 2, 3, 4, 7], 'col2': [4, 5, 6, 9, 5], 'col3': [7, 8, 12, 1, 11]}
# df = pd.DataFrame(data=d)
# print("Original DataFrame")
# print(df)
# print("\nNumber of columns:")
# print(len(df.columns))
# Sample Output:

# Original DataFrame
#    col1  col2  col3
# 0     1     4     7
# 1     2     5     8
# 2     3     6    12
# 3     4     9     1
# 4     7     5    11

# Number of columns:
# 3   


# 58. Select All Except One Column

# Write a Pandas program to select all columns, except one given column in a DataFrame.

# Sample Solution :

# Python Code :

# import pandas as pd
# d = {'col1': [1, 2, 3, 4, 7], 'col2': [4, 5, 6, 9, 5], 'col3': [7, 8, 12, 1, 11]}
# df = pd.DataFrame(data=d)
# print("Original DataFrame")
# print(df)
# print("\nAll columns except 'col3':")
# df = df.loc[:, df.columns != 'col3']
# print(df)
# Sample Output:

# Original DataFrame
#    col1  col2  col3
# 0     1     4     7
# 1     2     5     8
# 2     3     6    12
# 3     4     9     1
# 4     7     5    11

# All columns except 'col3':
#    col1  col2
# 0     1     4
# 1     2     5
# 2     3     6
# 3     4     9
# 4     7     5


# 59. Get First n Records

# Write a Pandas program to get first n records of a DataFrame.

# Sample Solution :

# Python Code :

# import pandas as pd
# d = {'col1': [1, 2, 3, 4, 7, 11], 'col2': [4, 5, 6, 9, 5, 0], 'col3': [7, 5, 8, 12, 1,11]}
# df = pd.DataFrame(data=d)
# print("Original DataFrame")
# print(df)
# print("\nFirst 3 rows of the said DataFrame':")
# df1 = df.head(3)
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

# First 3 rows of the said DataFrame':
#    col1  col2  col3
# 0     1     4     7
# 1     2     5     5
# 2     3     6     8


