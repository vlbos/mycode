# 40. Shuffle DataFrame Rows

# Write a Pandas program to shuffle a given DataFrame rows.
# Sample data:
# Original DataFrame:
# attempts name qualify score
# 0 1 Anastasia yes 12.5
# 1 3 Dima no 9.0
# 2 2 Katherine yes 16.5
# 3 3 James no NaN
# 4 2 Emily no 9.0
# 5 3 Michael yes 20.0
# 6 1 Matthew yes 14.5
# 7 1 Laura no NaN
# 8 2 Kevin no 8.0
# 9 1 Jonas yes 19.0
# New DataFrame:
# attempts name qualify score
# 5 3 Michael yes 20.0
# 0 1 Anastasia yes 12.5
# 9 1 Jonas yes 19.0
# 6 1 Matthew yes 14.5
# 7 1 Laura no NaN
# 1 3 Dima no 9.0
# 3 3 James no NaN
# 4 2 Emily no 9.0
# 8 2 Kevin no 8.0
# 2 2 Katherine yes 16.5

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
# df = df.sample(frac=1)
# print("\nNew DataFrame:")
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

# New DataFrame:
#    attempts       name qualify  score
# 5         3    Michael     yes   20.0
# 0         1  Anastasia     yes   12.5
# 9         1      Jonas     yes   19.0
# 6         1    Matthew     yes   14.5
# 7         1      Laura      no    NaN
# 1         3       Dima      no    9.0
# 3         3      James      no    NaN
# 4         2      Emily      no    9.0
# 8         2      Kevin      no    8.0
# 2         2  Katherine     yes   16.5               
# Explanation:

# The said code first creates a Pandas DataFrame df from the dictionary ‘exam_data’. The DataFrame contains information about students' names, scores, number of attempts and whether they qualify or not.

# df = df.sample(frac=1): This code shuffles the rows of the Pandas DataFrame df randomly using the sample method with frac=1, which means to sample all rows. It essentially reorders the rows of the DataFrame randomly.

# The original DataFrame is ‘exam_data’. The DataFrame has 4 columns, namely name, score, attempts, and qualify. Each column has 10 elements. The sample method is used to shuffle the rows of this DataFrame in a random order.

# 41. String to Datetime

# Write a Pandas program to convert DataFrame column type from string to datetime.
# Sample data:
# String Date:
# 0 3/11/2000
# 1 3/12/2000
# 2 3/13/2000
# dtype: object
# Original DataFrame (string to datetime):
# 0
# 0 2000-03-11
# 1 2000-03-12
# 2 2000-03-13

# Sample Solution :

# Python Code :

# import pandas as pd
# import numpy as np
# s = pd.Series(['3/11/2000', '3/12/2000', '3/13/2000'])
# print("String Date:")
# print(s)
# r = pd.to_datetime(pd.Series(s))
# df = pd.DataFrame(r)
# print("Original DataFrame (string to datetime):")
# print(df)
# Sample Output:

#  String Date:
# 0    3/11/2000
# 1    3/12/2000
# 2    3/13/2000
# dtype: object
# Original DataFrame (string to datetime):
#            0
# 0 2000-03-11
# 1 2000-03-12
# 2 2000-03-13             
# Explanation:

# The above code first creates a Pandas Series object s containing three strings that represent dates in 'month/day/year' format.

# r = pd.to_datetime(pd.Series(s)): This line uses the pd.to_datetime() method to convert each string date into a Pandas datetime object, and then create a new Pandas Series object ‘r’ containing these datetime objects.

# df = pd.DataFrame(r): Finally, the code creates a new Pandas DataFrame ‘df’ from ‘r’ by passing it as the only column of the DataFrame. The resulting DataFrame df contains a single column of datetime objects representing the dates from the original Series ‘s’

# 42. Rename Specific Column

# Write a Pandas program to rename a specific column name in a given DataFrame.
# Sample data:
# Original DataFrame
# col1 col2 col3
# 0 1 4 7
# 1 2 5 8
# 2 3 6 9
# New DataFrame after renaming second column:
# col1 Column2 col3
# 0 1 4 7
# 1 2 5 8
# 2 3 6 9

# Sample Solution :

# Python Code :

# import pandas as pd
# d = {'col1': [1, 2, 3], 'col2': [4, 5, 6], 'col3': [7, 8, 9]}
# df = pd.DataFrame(data=d)
# print("Original DataFrame")
# print(df)
# df=df.rename(columns = {'col2':'Column2'})
# print("New DataFrame after renaming second column:")
# print(df)
# Sample Output:

#     Original DataFrame
#    col1  col2  col3
# 0     1     4     7
# 1     2     5     8
# 2     3     6     9
# New DataFrame after renaming second column:
#    col1  Column2  col3
# 0     1        4     7
# 1     2        5     8
# 2     3        6     9              
# Explanation:

# The above code first creates a dataframe ‘df’ with three columns 'col1', 'col2' and 'col3', and three rows with some values.

# df=df.rename(columns = {'col2':'Column2'}): This code renames the 'col2' column to 'Column2' using the rename method of Pandas. The new dataframe ‘df’ now has columns 'col1', 'Column2' and 'col3' with the same values as before, except for the renamed column 'Column2'. The original column name 'col2' is no longer present in the dataframe.

# 43. Column to List

# Write a Pandas program to get a list of a specified column of a DataFrame.
# Sample data:
# Powered by
# Original DataFrame
# col1 col2 col3
# 0 1 4 7
# 1 2 5 8
# 2 3 6 9
# Col2 of the DataFrame to list:
# [4, 5, 6]

# Sample Solution :-

# Python Code :

# import pandas as pd
# d = {'col1': [1, 2, 3], 'col2': [4, 5, 6], 'col3': [7, 8, 9]}
# df = pd.DataFrame(data=d)
# print("Original DataFrame")
# print(df)
# col2_list = df["col2"].tolist()
# print("Col2 of the DataFrame to list:")
# print(col2_list)
# Sample Output:

#  Powered by 
# Original DataFrame
#    col1  col2  col3
# 0     1     4     7
# 1     2     5     8
# 2     3     6     9
# Col2 of the DataFrame to list:
# [4, 5, 6]                 
# Explanation:

# The above code creates a dictionary ‘d’ containing 3 columns with some sample data, then uses the pd.DataFrame() function from pandas to create a DataFrame ‘df’ from this dictionary.

# col2_list = df["col2"].tolist(): This line of code extracts the column named col2 from the DataFrame df using df["col2"], and converts it into a Python list using the tolist() method. The resulting col2_list variable contains the values of the col2 column in a list format.

# 44. DataFrame from NumPy Array

# Write a Pandas program to create a DataFrame from a Numpy array and specify the index column and column headers.

# Sample Solution :

# Python Code :

# import pandas
# import numpy
# dtype = [('Column1','int32'), ('Column2','float32'), ('Column3','float32')]
# values = numpy.zeros(15, dtype=dtype)
# index = ['Index'+str(i) for i in range(1, len(values)+1)]
# df = pandas.DataFrame(values, index=index)
# print(df)
# Sample Output:

#           Column1  Column2  Column3
# Index1         0      0.0      0.0
# Index2         0      0.0      0.0
# Index3         0      0.0      0.0
# Index4         0      0.0      0.0
# Index5         0      0.0      0.0
# Index6         0      0.0      0.0
# Index7         0      0.0      0.0
# Index8         0      0.0      0.0
# Index9         0      0.0      0.0
# Index10        0      0.0      0.0
# Index11        0      0.0      0.0
# Index12        0      0.0      0.0
# Index13        0      0.0      0.0
# Index14        0      0.0      0.0
# Index15        0      0.0      0.0                 
# Explanation:

# dtype = [('Column1','int32'), ('Column2','float32'), ('Column3','float32')]: This code creates a Pandas DataFrame with 15 rows and 3 columns, named 'Column1', 'Column2', and 'Column3', respectively. The data type of the columns are set to be 'int32', 'float32', and 'float32', respectively.

# values = numpy.zeros(15, dtype=dtype): This code creates a NumPy structured array with 15 rows and 3 fields using numpy.zeros function. numpy.zeros function initializes an array with zeros of given shape and data type.

# index = ['Index'+str(i) for i in range(1, len(values)+1)]: This code sets the index for the DataFrame using a list comprehension. Here, the index is a list of strings starting from "Index1" to "Index15".

# df = pandas.DataFrame(values, index=index): Finally, it creates the DataFrame using the Pandas DataFrame function with the values and index as parameters, and prints it using the print function.

# 45. Row with Maximum Value

# Write a Pandas program to find the row for where the value of a given column is maximum.

# Sample Solution:

# Python Code :

# import pandas as pd
# d = {'col1': [1, 2, 3, 4, 7], 'col2': [4, 5, 6, 9, 5], 'col3': [7, 8, 12, 1, 11]}
# df = pd.DataFrame(data=d)
# print("Original DataFrame")
# print(df)
# print("Row where col1 has maximum value:")
# print(df['col1'].argmax())
# print("Row where col2 has maximum value:")
# print(df['col2'].argmax())
# print("Row where col3 has maximum value:")
# print(df['col3'].argmax())
# Sample Output:

#     Original DataFrame
#    col1  col2  col3
# 0     1     4     7
# 1     2     5     8
# 2     3     6    12
# 3     4     9     1
# 4     7     5    11
# Row where col1 has maximum value:
# 4
# Row where col2 has maximum value:
# 3
# Row where col3 has maximum value:
# 2              
# Explanation:

# The above code creates a pandas DataFrame 'df' with three columns - 'col1', 'col2', and 'col3'. The code then uses the 'argmax()' function to find the index of the maximum value in each column.

# Therefore -

# The first 'print' statement returns the index of the row that has the maximum value in 'col1'.
# The second 'print' statement returns the index of the row that has the maximum value in 'col2'.
# The third 'print' statement returns the index of the row that has the maximum value in 'col3'.
# 46. Check Column Presence

# Write a Pandas program to check whether a given column is present in a DataFrame or not.

# Sample Solution :

# Python Code :

# import pandas as pd
# d = {'col1': [1, 2, 3, 4, 7], 'col2': [4, 5, 6, 9, 5], 'col3': [7, 8, 12, 1, 11]}
# df = pd.DataFrame(data=d)
# print("Original DataFrame")
# print(df)
# if 'col4' in df.columns:
#   print("Col4 is present in DataFrame.")
# else:
#   print("Col4 is not present in DataFrame.")
# if 'col1' in df.columns:
#   print("Col1 is present in DataFrame.")
# else:
#   print("Col1 is not present in DataFrame.")
# Sample Output:

# Original DataFrame
#    col1  col2  col3
# 0     1     4     7
# 1     2     5     8
# 2     3     6    12
# 3     4     9     1
# 4     7     5    11
# Col4 is not present in DataFrame.
# Col1 is present in DataFrame.                
# Explanation:

# The above code first creates a Pandas DataFrame ‘df’ with three columns named col1, col2, and col3 and five rows of data.

# The code then checks if the column 'col4' is present in the DataFrame using the in operator with the columns attribute of the DataFrame. Since 'col4' is not one of the DataFrame columns, the output will be "Col4 is not present in DataFrame."

# Next, the code checks if the column 'col1' is present in the DataFrame. Since 'col1' is one of the DataFrame columns, the output will be "Col1 is present in DataFrame."

# 47. Get Row Value

# Write a Pandas program to get the specified row value of a given DataFrame.

# Sample Solution :

# Python Code :

# import pandas as pd
# d = {'col1': [1, 2, 3, 4, 7], 'col2': [4, 5, 6, 9, 5], 'col3': [7, 8, 12, 1, 11]}
# df = pd.DataFrame(data=d)
# print("Original DataFrame")
# print(df)
# print("Value of Row1")
# print(df.iloc[0])
# print("Value of Row4")
# print(df.iloc[3])
# Sample Output:

# Original DataFrame
#    col1  col2  col3
# 0     1     4     7
# 1     2     5     8
# 2     3     6    12
# 3     4     9     1
# 4     7     5    11
# Value of Row1
# col1    1
# col2    4
# col3    7
# Name: 0, dtype: int64
# Value of Row4
# col1    4
# col2    9
# col3    1
# Name: 3, dtype: int64     
# Explanation:

# The above code first creates a Pandas DataFrame df using a dictionary ‘d’ with three columns 'col1', 'col2', and 'col3', and five rows of data.

# df.iloc[0]: This line selects the first row of the DataFrame df using the .iloc indexer, which selects rows by their integer position. Since the first row has an integer position of 0, this will select the first row of the DataFrame.

# df.iloc[3] : This line selects the fourth row of the DataFrame df using the .iloc indexer, which selects rows by their integer position. Since the fourth row has an integer position of 3, this will select the fourth row of the DataFrame.

# 48. Get Column DataTypes

# Write a Pandas program to get the datatypes of columns of a DataFrame.

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
# print("Data types of the columns of the said DataFrame:")
# print(df.dtypes)
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
# Data types of the columns of the said DataFrame:
# attempts      int64
# name         object
# qualify      object
# score       float64
# dtype: object                  
# Explanation:

# In the above code, a Pandas DataFrame named 'df' is created using a dictionary of lists 'exam_data' containing columns 'name', 'score', 'attempts', and 'qualify'.

# print(df.dtypes): This code prints the data types of each column of the DataFrame df.

# 49. Append Data to Empty DataFrame

# Write a Pandas program to append data to an empty DataFrame.

# Sample data:
# Original DataFrame:
# After appending some data:
# col1 col2
# 0 0 0
# 1 1 1
# 2 2 2

# Sample Solution :-

# Python Code :

# import pandas as pd
# import numpy as np
# df = pd.DataFrame()
# data = pd.DataFrame({"col1": range(3),"col2": range(3)})
# print("After appending some data:")
# df = df.append(data)
# print(df)
# Sample Output:

# After appending some data:
#    col1  col2
# 0     0     0
# 1     1     1
# 2     2     2         
# Explanation:

# The above code creates an empty Pandas DataFrame ‘df’, then creates another DataFrame called ‘data’ with two columns, ‘col1’ and ‘col2’, containing values from 0 to 2.

# df = df.append(data): Here the DataFrame ‘data’ is appended to the ‘df’ DataFrame using the append() method. The resulting DataFrame, ‘df’, has the same columns as data and contains the values from data.

# Finally print(df) statement outputs the contents of the df DataFrame.

