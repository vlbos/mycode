# 30. Widen Output Display

# Write a Pandas program to widen output display to see more columns.

# Sample data:
# Original DataFrame
# col1 col2 col3
# 0 1 4 7
# 1 4 5 8
# 2 3 6 9
# 3 4 7 0
# 4 5 8 1

# Sample Solution :

# Python Code :

# import pandas as pd
# import numpy as np
# d = {'col1': [1, 4, 3, 4, 5], 'col2': [4, 5, 6, 7, 8], 'col3': [7, 8, 9, 0, 1]}
# df = pd.DataFrame(data=d)
# pd.set_option('display.max_rows', 500)
# pd.set_option('display.max_columns', 500)
# pd.set_option('display.width', 1000)
# print("Original DataFrame")
# print(df)
# Sample Output:

#    Original DataFrame
#    col1  col2  col3
# 0     1     4     7
# 1     4     5     8
# 2     3     6     9
# 3     4     7     0
# 4     5     8     1               
# Explanation:

# The above code first creates a Pandas DataFrame df with columns col1, col2, and col3 using a dictionary 'd'.

# Then the code sets some display options for Pandas dataframes 'df'.

# pd.set_option('display.max_rows', 500) sets the maximum number of rows that Pandas will display to 500.
# pd.set_option('display.max_columns', 500) sets the maximum number of columns that Pandas will display to 500.
# pd.set_option('display.width', 1000) sets the maximum width of the display to 1000 characters.
# Finally print() function prints the dataframe ‘df’.


# 31. Select Row by Integer Index

# Write a Pandas program to select a row of series/dataframe by given integer index.

# Sample data:
# Original DataFrame
# col1 col2 col3
# 0 1 4 7
# 1 4 5 8
# 2 3 6 9
# 3 4 7 0
# 4 5 8 1
# Index-2: Details
# col1 col2 col3
# 2 3 6 9

# Sample Solution :

# Python Code :

# import pandas as pd
# import numpy as np
# d = {'col1': [1, 4, 3, 4, 5], 'col2': [4, 5, 6, 7, 8], 'col3': [7, 8, 9, 0, 1]}
# df = pd.DataFrame(data=d)
# print("Original DataFrame")
# print(df)
# result = df.iloc[[2]]
# print("Index-2: Details")
# print(result)
# Sample Output:

#  Original DataFrame
#    col1  col2  col3
# 0     1     4     7
# 1     4     5     8
# 2     3     6     9
# 3     4     7     0
# 4     5     8     1
# Index-2: Details
#    col1  col2  col3
# 2     3     6     9              
# Explanation:

# The above code first creates a Pandas DataFrame df with columns col1, col2, and col3 using a dictionary 'd'.

# result = df.iloc[[2]] – This code selects the third row of the DataFrame using the iloc() method with index location [2] and stores it in a new DataFrame called result.

# Finally print() function prints the DataFrame containing only the third row of 'df'.

# 32. Replace NaN with Zeros

# Write a Pandas program to replace all the NaN values with Zero's in a column of a dataframe.

# Sample data:
# Original DataFrame
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
# New DataFrame replacing all NaN with 0:
# attempts name qualify score
# 0 1 Anastasia yes 12.5
# 1 3 Dima no 9.0
# 2 2 Katherine yes 16.5
# 3 3 James no 0.0
# 4 2 Emily no 9.0
# 5 3 Michael yes 20.0
# 6 1 Matthew yes 14.5
# 7 1 Laura no 0.0
# 8 2 Kevin no 8.0
# 9 1 Jonas yes 19.0

# Sample Solution :

# Python Code :

# import pandas as pd
# import numpy as np
# exam_data = {'name': ['Anastasia', 'Dima', 'Katherine', 'James', 'Emily', 'Michael', 'Matthew', 'Laura', 'Kevin', 'Jonas'],
#         'score': [12.5, 9, 16.5, np.nan, 9, 20, 14.5, np.nan, 8, 19],
#         'attempts': [1, 3, 2, 3, 2, 3, 1, 1, 2, 1],
#         'qualify': ['yes', 'no', 'yes', 'no', 'no', 'yes', 'yes', 'no', 'no', 'yes']}
# df = pd.DataFrame(exam_data)
# print("Original DataFrame")
# print(df)
# df =  df.fillna(0)
# print("\nNew DataFrame replacing all NaN with 0:")
# print(df)
# Sample Output:

#  Original DataFrame
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

# New DataFrame replacing all NaN with 0:
#    attempts       name qualify  score
# 0         1  Anastasia     yes   12.5
# 1         3       Dima      no    9.0
# 2         2  Katherine     yes   16.5
# 3         3      James      no    0.0
# 4         2      Emily      no    9.0
# 5         3    Michael     yes   20.0
# 6         1    Matthew     yes   14.5
# 7         1      Laura      no    0.0
# 8         2      Kevin      no    8.0
# 9         1      Jonas     yes   19.0                
# Explanation:

# The above code creates a Pandas DataFrame called ‘df’ from a dictionary called ‘exam_data’ that contains information about students and their exam scores. Some of the students have missing scores, which are represented as np.nan values.

# df = df.fillna(0): The fillna() method is then used to fill in these missing values with 0.

# Finally the resulting DataFrame is printed to the console using print() function.


# 33. Convert Index to Column

# Write a Pandas program to convert index in a column of the given dataframe.

# Sample data:
# Original DataFrame
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
# After converting index in a column:
# index attempts name qualify score
# 0 0 1 Anastasia yes 12.5
# 1 1 3 Dima no 9.0
# 2 2 2 Katherine yes 16.5
# 3 3 3 James no NaN
# 4 4 2 Emily no 9.0
# 5 5 3 Michael yes 20.0
# 6 6 1 Matthew yes 14.5
# 7 7 1 Laura no NaN
# 8 8 2 Kevin no 8.0
# 9 9 1 Jonas yes 19.0
# Hiding index:
# index attempts name qualify score
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

# Sample Solution :

# Python Code :

# import pandas as pd
# import numpy as np
# exam_data = {'name': ['Anastasia', 'Dima', 'Katherine', 'James', 'Emily', 'Michael', 'Matthew', 'Laura', 'Kevin', 'Jonas'],
#         'score': [12.5, 9, 16.5, np.nan, 9, 20, 14.5, np.nan, 8, 19],
#         'attempts': [1, 3, 2, 3, 2, 3, 1, 1, 2, 1],
#         'qualify': ['yes', 'no', 'yes', 'no', 'no', 'yes', 'yes', 'no', 'no', 'yes']}
# df = pd.DataFrame(exam_data)
# print("Original DataFrame")
# print(df)
# print("\nAfter converting index in a column:")
# df.reset_index(level=0, inplace=True)
# print(df)
# print("\nHiding index:")
# print( df.to_string(index=False))
# Sample Output:

#   Original DataFrame
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

# After converting index in a column:
#    index  attempts       name qualify  score
# 0      0         1  Anastasia     yes   12.5
# 1      1         3       Dima      no    9.0
# 2      2         2  Katherine     yes   16.5
# 3      3         3      James      no    NaN
# 4      4         2      Emily      no    9.0
# 5      5         3    Michael     yes   20.0
# 6      6         1    Matthew     yes   14.5
# 7      7         1      Laura      no    NaN
# 8      8         2      Kevin      no    8.0
# 9      9         1      Jonas     yes   19.0

# Hiding index:
# index  attempts       name qualify  score
#     0         1  Anastasia     yes   12.5
#     1         3       Dima      no    9.0
#     2         2  Katherine     yes   16.5
#     3         3      James      no    NaN
#     4         2      Emily      no    9.0
#     5         3    Michael     yes   20.0
#     6         1    Matthew     yes   14.5
#     7         1      Laura      no    NaN
#     8         2      Kevin      no    8.0
#     9         1      Jonas     yes   19.0                
# Explanation:

# The above code first creates a Pandas DataFrame 'df' from the 'exam_data' dictionary.

# df.reset_index(level=0, inplace=True): This line resets the index of the DataFrame by moving the current index to a new column and creating a new integer index.

# Finally, it prints the DataFrame to the console as a string, without the index.

# Note: The reset_index() function is used to generate a new DataFrame or Series with the index reset. This is useful when the index needs to be treated as a column, or when the index is meaningless and needs to be reset to the default before another operation.

# The to_string() function is used to render a DataFrame to a console-friendly tabular output.

# 34. Set Value in Cell by Index

# Write a Pandas program to set a given value for particular cell in DataFrame using index value.

# Sample data:
# Original DataFrame
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
# Set a given value for particular cell in the DataFrame
# attempts name qualify score
# 0 1 Anastasia yes 12.5
# 1 3 Dima no 9.0
# 2 2 Katherine yes 16.5
# 3 3 James no NaN
# 4 2 Emily no 9.0
# 5 3 Michael yes 20.0
# 6 1 Matthew yes 14.5
# 7 1 Laura no NaN
# 8 2 Kevin no 10.2
# 9 1 Jonas yes 19.0

# Sample Solution :-

# Python Code :

# import pandas as pd
# import numpy as np
# exam_data = {'name': ['Anastasia', 'Dima', 'Katherine', 'James', 'Emily', 'Michael', 'Matthew', 'Laura', 'Kevin', 'Jonas'],
#         'score': [12.5, 9, 16.5, np.nan, 9, 20, 14.5, np.nan, 8, 19],
#         'attempts': [1, 3, 2, 3, 2, 3, 1, 1, 2, 1],
#         'qualify': ['yes', 'no', 'yes', 'no', 'no', 'yes', 'yes', 'no', 'no', 'yes']}
# df = pd.DataFrame(exam_data)
# print("Original DataFrame")
# print(df)
# print("\nSet a given value for particular cell in the DataFrame")
# df.set_value(8, 'score', 10.2)
# print(df)
# Sample Output:

#  Original DataFrame
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

# Set a given value for particular cell in the DataFrame
#    attempts       name qualify  score
# 0         1  Anastasia     yes   12.5
# 1         3       Dima      no    9.0
# 2         2  Katherine     yes   16.5
# 3         3      James      no    NaN
# 4         2      Emily      no    9.0
# 5         3    Michael     yes   20.0
# 6         1    Matthew     yes   14.5
# 7         1      Laura      no    NaN
# 8         2      Kevin      no   10.2
# 9         1      Jonas     yes   19.0                 
# Explanation:

# The above code first creates a Pandas DataFrame 'df' from the 'exam_data' dictionary.

# df.set_value(8, 'score', 10.2): The set_value() function is used to modify the value of the cell in the 8th row and 'score' column to 10.2.

# Finally print() function prints the DataFrame 'df'.

# 35. Count NaN Values

# Write a Pandas program to count the NaN values in one or more columns in DataFrame.

# Sample data:
# Original DataFrame
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
# Number of NaN values in one or more columns:
# 2

# Sample Solution :

# Python Code :

# import pandas as pd
# import numpy as np
# exam_data = {'name': ['Anastasia', 'Dima', 'Katherine', 'James', 'Emily', 'Michael', 'Matthew', 'Laura', 'Kevin', 'Jonas'],
#         'score': [12.5, 9, 16.5, np.nan, 9, 20, 14.5, np.nan, 8, 19],
#         'attempts': [1, 3, 2, 3, 2, 3, 1, 1, 2, 1],
#         'qualify': ['yes', 'no', 'yes', 'no', 'no', 'yes', 'yes', 'no', 'no', 'yes']}
# df = pd.DataFrame(exam_data)
# print("Original DataFrame")
# print(df)
# print("\nNumber of NaN values in one or more columns:")
# print(df.isnull().values.sum())
# Sample Output:

#       Original DataFrame
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

# Number of NaN values in one or more columns:
# 2            
# Explanation:

# The above code creates a pandas DataFrame ‘df’ from a dictionary ‘exam_data’ containing information about some exam scores.

# df.isnull().values.sum(): This code uses the isnull() function to check which values in the DataFrame are null or NaN, and returns a DataFrame containing the same shape as ‘df’ with True for missing values and False for non-missing values. The values attribute is used to extract the values of the resulting DataFrame and the sum() function is applied to the values to get the total count of missing values in the original DataFrame.

# Finally print() function prints the total number of missing values in the DataFrame.

# 36. Drop Rows from DataFrame

# Write a Pandas program to drop a list of rows from a specified DataFrame.

# Sample data:
# Original DataFrame
# col1 col2 col3
# 0 1 4 7
# 1 4 5 8
# 2 3 6 9
# 3 4 7 0
# 4 5 8 1
# New DataFrame after removing 2nd & 4th rows:
# col1 col2 col3
# 0 1 4 7
# 1 4 5 8
# 3 4 7 0

# Sample Solution :

# Python Code :

# import pandas as pd
# import numpy as np
# d = {'col1': [1, 4, 3, 4, 5], 'col2': [4, 5, 6, 7, 8], 'col3': [7, 8, 9, 0, 1]}
# df = pd.DataFrame(d)
# print("Original DataFrame")
# print(df)
# print("New DataFrame after removing 2nd & 4th rows:")
# df = df.drop(df.index[[2,4]])
# print(df)
# Sample Output:

# Original DataFrame
#    col1  col2  col3
# 0     1     4     7
# 1     4     5     8
# 2     3     6     9
# 3     4     7     0
# 4     5     8     1
# New DataFrame after removing 2nd & 4th rows:
#    col1  col2  col3
# 0     1     4     7
# 1     4     5     8
# 3     4     7     0              
# Explanation:

# The above code creates a Pandas DataFrame ‘df’ using a Python dictionary ‘d’. The DataFrame has three columns: 'col1', 'col2', and 'col3'.

# df = df.drop(df.index[[2,4]]): This code drops rows with indices 2 and 4 using the drop() method with the index parameter set to a list of indices to drop.

# Finally print() function prints the resulting DataFrame with the two rows dropped.

# 37. Reset DataFrame Index

# Write a Pandas program to reset index in a given DataFrame.

# Sample data:
# Original DataFrame
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
# After removing first and second rows
# attempts name qualify score
# 2 2 Katherine yes 16.5
# 3 3 James no NaN
# 4 2 Emily no 9.0
# 5 3 Michael yes 20.0
# 6 1 Matthew yes 14.5
# 7 1 Laura no NaN
# 8 2 Kevin no 8.0
# 9 1 Jonas yes 19.0
# Reset the Index:
# index attempts name qualify score
# 0 2 2 Katherine yes 16.5
# 1 3 3 James no NaN
# 2 4 2 Emily no 9.0
# 3 5 3 Michael yes 20.0
# 4 6 1 Matthew yes 14.5
# 5 7 1 Laura no NaN
# 6 8 2 Kevin no 8.0
# 7 9 1 Jonas yes 19.0

# Sample Solution :

# Python Code :

# import pandas as pd
# import numpy as np
# exam_data = {'name': ['Anastasia', 'Dima', 'Katherine', 'James', 'Emily', 'Michael', 'Matthew', 'Laura', 'Kevin', 'Jonas'],
#         'score': [12.5, 9, 16.5, np.nan, 9, 20, 14.5, np.nan, 8, 19],
#         'attempts': [1, 3, 2, 3, 2, 3, 1, 1, 2, 1],
#         'qualify': ['yes', 'no', 'yes', 'no', 'no', 'yes', 'yes', 'no', 'no', 'yes']}
# df = pd.DataFrame(exam_data)
# print("Original DataFrame")
# print(df)
# print("\nAfter removing first and second rows")
# df = df.drop([0, 1])
# print(df)
# print("\nReset the Index:")
# df = df.reset_index()
# print(df)
# Sample Output:

#   Original DataFrame
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

# After removing first and second rows
#    attempts       name qualify  score
# 2         2  Katherine     yes   16.5
# 3         3      James      no    NaN
# 4         2      Emily      no    9.0
# 5         3    Michael     yes   20.0
# 6         1    Matthew     yes   14.5
# 7         1      Laura      no    NaN
# 8         2      Kevin      no    8.0
# 9         1      Jonas     yes   19.0

# Reset the Index:
#    index  attempts       name qualify  score
# 0      2         2  Katherine     yes   16.5
# 1      3         3      James      no    NaN
# 2      4         2      Emily      no    9.0
# 3      5         3    Michael     yes   20.0
# 4      6         1    Matthew     yes   14.5
# 5      7         1      Laura      no    NaN
# 6      8         2      Kevin      no    8.0
# 7      9         1      Jonas     yes   19.0                
# Explanation:

# The said code first creates a Pandas DataFrame df from the dictionary ‘exam_data’. The DataFrame contains information about students' names, scores, number of attempts and whether they qualify or not.

# df = df.drop([0, 1]): This code drops the rows with index 0 and 1 from the DataFrame using the drop() method.

# df = df.reset_index(): This code resets the index using the reset_index() method, which creates a new column called "index" with the index values before resetting.

# Finally, it prints the resulting DataFrame with the dropped rows and the new index.

# 38. Divide DataFrame by Ratio

# Write a Pandas program to divide a DataFrame in a given ratio.

# Sample data:
# Original DataFrame:
# 0 1
# 0 0.316147 -0.767359
# 1 -0.813410 -2.522672
# 2 0.869615 1.194704
# 3 -0.892915 -0.055133
# 4 -0.341126 0.518266
# 5 1.857342 1.361229
# 6 -0.044353 -1.205002
# 7 -0.726346 -0.535147
# 8 -1.350726 0.563117
# 9 1.051666 -0.441533
# 70% of the said DataFrame:
# 0 1
# 8 -1.350726 0.563117
# 2 0.869615 1.194704
# 5 1.857342 1.361229
# 6 -0.044353 -1.205002
# 3 -0.892915 -0.055133
# 1 -0.813410 -2.522672
# 0 0.316147 -0.767359
# 30% of the said DataFrame:
# 0 1
# 4 -0.341126 0.518266
# 7 -0.726346 -0.535147
# 9 1.051666 -0.441533

# Sample Solution :

# Python Code :

# import pandas as pd
# import numpy as np
# df = pd.DataFrame(np.random.randn(10, 2))
# print("Original DataFrame:")
# print(df)
# part_70 = df.sample(frac=0.7,random_state=10)
# part_30 = df.drop(part_70.index)
# print("\n70% of the said DataFrame:")
# print(part_70)
# print("\n30% of the said DataFrame:")
# print(part_30)
# Sample Output:

# Original DataFrame:
#           0         1
# 0  0.316147 -0.767359
# 1 -0.813410 -2.522672
# 2  0.869615  1.194704
# 3 -0.892915 -0.055133
# 4 -0.341126  0.518266
# 5  1.857342  1.361229
# 6 -0.044353 -1.205002
# 7 -0.726346 -0.535147
# 8 -1.350726  0.563117
# 9  1.051666 -0.441533

# 70% of the said DataFrame:
#           0         1
# 8 -1.350726  0.563117
# 2  0.869615  1.194704
# 5  1.857342  1.361229
# 6 -0.044353 -1.205002
# 3 -0.892915 -0.055133
# 1 -0.813410 -2.522672
# 0  0.316147 -0.767359

# 30% of the said DataFrame:
#           0         1
# 4 -0.341126  0.518266
# 7 -0.726346 -0.535147
# 9  1.051666 -0.441533                  
# Explanation:

# The above code first generates a Pandas DataFrame df with 10 rows and 2 columns filled with random numbers using NumPy.

# part_70 = df.sample(frac=0.7,random_state=10): This code creates a new DataFrame 'part_70' by sampling 70% of the rows from 'df' using the sample method. The 'frac' parameter specifies the fraction of the rows to be sampled, while the random_state parameter is used to ensure that the same set of rows is always sampled if the code is run again with the same random_state value.

# part_30 = df.drop(part_70.index): This code creates another DataFrame 'part_30' by dropping the rows in ‘part_70’ from ‘df’. This is achieved by calling the drop method on ‘df’ with the indices of the rows to be dropped, which are obtained by calling the index attribute on ‘part_70’. The resulting DataFrame ‘part_30’ contains the remaining 30% of the rows from df.

# 39. Combine Two Series

# Write a Pandas program to combining two series into a DataFrame.

# Sample data:
# Data Series:
# 0 100
# 1 200
# 2 python
# 3 300.12
# 4 400
# dtype: object
# 0 10
# 1 20
# 2 php
# 3 30.12
# 4 40
# dtype: object
# New DataFrame combining two series:
# 0 1
# 0 100 10
# 1 200 20
# 2 python php
# 3 300.12 30.12
# 4 400 40

# Sample Solution :

# Python Code :

# import pandas as pd
# import numpy as np
# s1 = pd.Series(['100', '200', 'python', '300.12', '400'])
# s2 = pd.Series(['10', '20', 'php', '30.12', '40'])
# print("Data Series:")
# print(s1)
# print(s2)
# df = pd.concat([s1, s2], axis=1)
# print("New DataFrame combining two series:")
# print(df)
# Sample Output:

#           Data Series:
# 0       100
# 1       200
# 2    python
# 3    300.12
# 4       400
# dtype: object
# 0       10
# 1       20
# 2      php
# 3    30.12
# 4       40
# dtype: object
# New DataFrame combining two series:
#         0      1
# 0     100     10
# 1     200     20
# 2  python    php
# 3  300.12  30.12
# 4     400     40        
# Explanation:

# The above code creates two Pandas Series ‘s1’ and ‘s2’ with five elements each. The elements in the two series are a mix of integer, string, and float values.

# df = pd.concat([s1, s2], axis=1): This code concatenates the two series along axis 1 using the pd.concat() function to create a new DataFrame df. Since the axis is 1, the two series are stacked horizontally as columns.

# The resulting DataFrame will have 5 rows and 2 columns.

