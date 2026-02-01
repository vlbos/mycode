# 1. Creating a DataFrame from a Dictionary

# Write a Pandas program to create a dataframe from a dictionary and display it.
# Sample data: {'X':[78,85,96,80,86], 'Y':[84,94,89,83,86],'Z':[86,97,96,72,83]}

# Sample Solution :

# Python Code :

# import pandas as pd
# df = pd.DataFrame({'X':[78,85,96,80,86], 'Y':[84,94,89,83,86],'Z':[86,97,96,72,83]});
# print(df)
# Sample Output:

#     X   Y   Z                                                          
# 0  78  84  86                                                          
# 1  85  94  97                                                          
# 2  96  89  96                                                          
# 3  80  83  72                                                          
# 4  86  86  83                                       
# Explanation:

 
# df = pd.DataFrame({'X':[78,85,96,80,86], 'Y':[84,94,89,83,86],'Z':[86,97,96,72,83]});
# The above code creates a pandas DataFrame object named ‘df’ with three columns X, Y, and Z and five rows. The values for each column are provided in a dictionary with keys X, Y, and Z.

# The print(df) statement prints the entire DataFrame to the console.


# 2. DataFrame with Specified Index Labels

# Write a Pandas program to create and display a DataFrame from a specified dictionary data which has the index labels.

# Sample DataFrame:
# exam_data = {'name': ['Anastasia', 'Dima', 'Katherine', 'James', 'Emily', 'Michael', 'Matthew', 'Laura', 'Kevin', 'Jonas'],
# 'score': [12.5, 9, 16.5, np.nan, 9, 20, 14.5, np.nan, 8, 19],
# 'attempts': [1, 3, 2, 3, 2, 3, 1, 1, 2, 1],
# 'qualify': ['yes', 'no', 'yes', 'no', 'no', 'yes', 'yes', 'no', 'no', 'yes']}
# labels = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j']

# Sample Solution :-

# Python Code :

# import pandas as pd
# import numpy as np

# exam_data  = {'name': ['Anastasia', 'Dima', 'Katherine', 'James', 'Emily', 'Michael', 'Matthew', 'Laura', 'Kevin', 'Jonas'],
#         'score': [12.5, 9, 16.5, np.nan, 9, 20, 14.5, np.nan, 8, 19],
#         'attempts': [1, 3, 2, 3, 2, 3, 1, 1, 2, 1],
#         'qualify': ['yes', 'no', 'yes', 'no', 'no', 'yes', 'yes', 'no', 'no', 'yes']}
# labels = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j']

# df = pd.DataFrame(exam_data , index=labels)
# print(df)
# Sample Output:

#    attempts       name qualify  score                                  
# a         1  Anastasia     yes   12.5                                  
# b         3       Dima      no    9.0                                  
# c         2  Katherine     yes   16.5                                  
# d         3      James      no    NaN                                  
# e         2      Emily      no    9.0                                  
# f         3    Michael     yes   20.0                                  
# g         1    Matthew     yes   14.5                                  
# h         1      Laura      no    NaN                                  
# i         2      Kevin      no    8.0                                  
# j         1      Jonas     yes   19.0                                      
# Explanation:

# The above code creates a Pandas DataFrame ‘df’ containing information about exam scores of ten students, with columns name, score, attempts, and qualify, and row labels 'a' to 'j'.

# The name column contains the names of the students.
# The score column contains the exam scores of the students.
# The attempts column contains the number of attempts made by the students to pass the exam.
# The qualify column contains whether the students have qualified for the exam or not.
# The DataFrame is created using a Python dictionary ‘exam_data’ and the index parameter is used to specify the row labels.

# Finally, the DataFrame is printed using the print function.

# 3. DataFrame Basic Summary Information

# Write a Pandas program to display a summary of the basic information about a specified DataFrame and its data.

# Sample DataFrame:
# exam_data = {'name': ['Anastasia', 'Dima', 'Katherine', 'James', 'Emily', 'Michael', 'Matthew', 'Laura', 'Kevin', 'Jonas'],
# 'score': [12.5, 9, 16.5, np.nan, 9, 20, 14.5, np.nan, 8, 19],
# 'attempts': [1, 3, 2, 3, 2, 3, 1, 1, 2, 1],
# 'qualify': ['yes', 'no', 'yes', 'no', 'no', 'yes', 'yes', 'no', 'no', 'yes']}
# labels = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j']

# Sample Solution :-

# Python Code :

# import pandas as pd
# import numpy as np

# exam_data  = {'name': ['Anastasia', 'Dima', 'Katherine', 'James', 'Emily', 'Michael', 'Matthew', 'Laura', 'Kevin', 'Jonas'],
#         'score': [12.5, 9, 16.5, np.nan, 9, 20, 14.5, np.nan, 8, 19],
#         'attempts': [1, 3, 2, 3, 2, 3, 1, 1, 2, 1],
#         'qualify': ['yes', 'no', 'yes', 'no', 'no', 'yes', 'yes', 'no', 'no', 'yes']}
# labels = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j']

# df = pd.DataFrame(exam_data , index=labels)
# print("Summary of the basic information about this DataFrame and its data:")
# print(df.info())
# Sample Output:

# Summary of the basic information about this DataFrame and its data:
# <class 'pandas.core.frame.DataFrame'>
# Index: 10 entries, a to j
# Data columns (total 4 columns):
# attempts    10 non-null int64
# name        10 non-null object
# qualify     10 non-null object
# score       8 non-null float64
# dtypes: float64(1), int64(1), object(2)
# memory usage: 400.0+ bytes
# None                              
# Explanation:

# The above code creates a Pandas DataFrame object 'df' containing information about an exam, such as the name of the student, their score, the number of attempts, and whether they qualify.

# The DataFrame is created using a Python dictionary 'exam_data' that contains lists of information about the students.

# The 'labels' list is used to set the index of the DataFrame.

# The DataFrame has four columns: 'name', 'score', 'attempts', and 'qualify'.

# The 'name' column contains the names of the students.
# The 'score' column contains the scores they received.
# The 'attempts' column contains the number of attempts taken to pass the exam.
# The 'qualify' column contains whether the student has qualified for the exam or not.
# Finally the DataFrame is then printed using the print() function.


# 4. Selecting the First 3 Rows

# Write a Pandas program to get the first 3 rows of a given DataFrame.

# Sample DataFrame:
# exam_data = {'name': ['Anastasia', 'Dima', 'Katherine', 'James', 'Emily', 'Michael', 'Matthew', 'Laura', 'Kevin', 'Jonas'],
# 'score': [12.5, 9, 16.5, np.nan, 9, 20, 14.5, np.nan, 8, 19],
# 'attempts': [1, 3, 2, 3, 2, 3, 1, 1, 2, 1],
# 'qualify': ['yes', 'no', 'yes', 'no', 'no', 'yes', 'yes', 'no', 'no', 'yes']}
# labels = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j']

# Sample Solution :

# Python Code :

# import pandas as pd
# import numpy as np

# exam_data  = {'name': ['Anastasia', 'Dima', 'Katherine', 'James', 'Emily', 'Michael', 'Matthew', 'Laura', 'Kevin', 'Jonas'],
#         'score': [12.5, 9, 16.5, np.nan, 9, 20, 14.5, np.nan, 8, 19],
#         'attempts': [1, 3, 2, 3, 2, 3, 1, 1, 2, 1],
#         'qualify': ['yes', 'no', 'yes', 'no', 'no', 'yes', 'yes', 'no', 'no', 'yes']}
# labels = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j']

# df = pd.DataFrame(exam_data , index=labels)
# print("First three rows of the data frame:")
# print(df.iloc[:3])
# Sample Output:

# First three rows of the data frame:                                    
#    attempts       name qualify  score                                  
# a         1  Anastasia     yes   12.5                                  
# b         3       Dima      no    9.0                                  
# c         2  Katherine     yes   16.5                                   
# Explanation:

# The above code creates a Pandas DataFrame named df with columns 'name', 'score', 'attempts', and 'qualify', and a custom index 'labels'. It then selects and prints the first three rows of the DataFrame using the .iloc indexing method.

# Specifically, df.iloc[:3] selects the first three rows of the DataFrame using integer-based indexing, where : indicates all rows and 3 indicates up to the third row (exclusive). This operation returns a new DataFrame containing the selected rows, which is then printed using the print() function.



# 5. Selecting 'name' and 'score' Columns

# Write a Pandas program to select the 'name' and 'score' columns from the following DataFrame.

# Sample DataFrame:
# exam_data = {'name': ['Anastasia', 'Dima', 'Katherine', 'James', 'Emily', 'Michael', 'Matthew', 'Laura', 'Kevin', 'Jonas'],
# 'score': [12.5, 9, 16.5, np.nan, 9, 20, 14.5, np.nan, 8, 19],
# 'attempts': [1, 3, 2, 3, 2, 3, 1, 1, 2, 1],
# 'qualify': ['yes', 'no', 'yes', 'no', 'no', 'yes', 'yes', 'no', 'no', 'yes']}
# labels = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j']

# Sample Solution :

# Python Code :

# import pandas as pd
# import numpy as np

# exam_data  = {'name': ['Anastasia', 'Dima', 'Katherine', 'James', 'Emily', 'Michael', 'Matthew', 'Laura', 'Kevin', 'Jonas'],
#         'score': [12.5, 9, 16.5, np.nan, 9, 20, 14.5, np.nan, 8, 19],
#         'attempts': [1, 3, 2, 3, 2, 3, 1, 1, 2, 1],
#         'qualify': ['yes', 'no', 'yes', 'no', 'no', 'yes', 'yes', 'no', 'no', 'yes']}
# labels = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j']

# df = pd.DataFrame(exam_data , index=labels)
# print("Select specific columns:")
# print(df[['name', 'score']])
# Sample Output:

# Select specific columns:                                               
#         name  score                                                    
# a  Anastasia   12.5                                                    
# b       Dima    9.0                                                    
# c  Katherine   16.5                                                    
# d      James    NaN                                                    
# e      Emily    9.0                                                    
# f    Michael   20.0                                                    
# g    Matthew   14.5                                                    
# h      Laura    NaN                                                    
# i      Kevin    8.0                                                    
# j      Jonas   19.0                                  
# Explanation:

# The above code creates a Pandas DataFrame ‘df’ with columns 'name', 'score', 'attempts', and 'qualify' using a Python dictionary ‘exam_data’ and index ‘labels’.

# df[['name', 'score']]: This line prints a subset of the DataFrame that includes only the 'name' and 'score' columns using the double square bracket notation .


# 6. Selecting Specific Columns and Rows

# Write a Pandas program to select the specified columns and rows from a given DataFrame.
# Select 'name' and 'score' columns in rows 1, 3, 5, 6 from the following data frame.

# Sample DataFrame:
# exam_data = {'name': ['Anastasia', 'Dima', 'Katherine', 'James', 'Emily', 'Michael', 'Matthew', 'Laura', 'Kevin', 'Jonas'],
# 'score': [12.5, 9, 16.5, np.nan, 9, 20, 14.5, np.nan, 8, 19],
# 'attempts': [1, 3, 2, 3, 2, 3, 1, 1, 2, 1],
# 'qualify': ['yes', 'no', 'yes', 'no', 'no', 'yes', 'yes', 'no', 'no', 'yes']}
# labels = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j']

# Sample Solution :

# Python Code :

# import pandas as pd
# import numpy as np

# exam_data  = {'name': ['Anastasia', 'Dima', 'Katherine', 'James', 'Emily', 'Michael', 'Matthew', 'Laura', 'Kevin', 'Jonas'],
#         'score': [12.5, 9, 16.5, np.nan, 9, 20, 14.5, np.nan, 8, 19],
#         'attempts': [1, 3, 2, 3, 2, 3, 1, 1, 2, 1],
#         'qualify': ['yes', 'no', 'yes', 'no', 'no', 'yes', 'yes', 'no', 'no', 'yes']}
# labels = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j']

# df = pd.DataFrame(exam_data , index=labels)
# print("Select specific columns and rows:")
# print(df.iloc[[1, 3, 5, 6], [1, 3]])
# Sample Output:

# Select specific columns and rows:
#    score qualify
# b    9.0      no
# d    NaN      no
# f   20.0     yes
# g   14.5     yes                               
# Explanation:

# The above code creates a Pandas DataFrame named 'df' containing information related to students' 'exam_data'

# print(df.iloc[[1, 3, 5, 6], [1, 3]]):

# In the said code iloc() function is used to select specific rows and columns from the DataFrame based on their integer location.
# [[1, 3, 5, 6], [1, 3]] is used as the parameter to select the rows with index 1, 3, 5, and 6 and columns with index 1 and 3 from the DataFrame.
# The selected rows contain the information for the students with the index label b, d, f, and g.
# The selected columns contain the information for the columns ‘score’ and ‘qualify’.
# Finally print() function prints 4 rows and 2 columns.


# 7. Selecting Rows Where Attempts > 2

# Write a Pandas program to select the rows where the number of attempts in the examination is greater than 2.

# Sample DataFrame:
# exam_data = {'name': ['Anastasia', 'Dima', 'Katherine', 'James', 'Emily', 'Michael', 'Matthew', 'Laura', 'Kevin', 'Jonas'],
# 'score': [12.5, 9, 16.5, np.nan, 9, 20, 14.5, np.nan, 8, 19],
# 'attempts': [1, 3, 2, 3, 2, 3, 1, 1, 2, 1],
# 'qualify': ['yes', 'no', 'yes', 'no', 'no', 'yes', 'yes', 'no', 'no', 'yes']}
# labels = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j']

# Sample Solution :

# Python Code :

# import pandas as pd
# import numpy as np

# exam_data  = {'name': ['Anastasia', 'Dima', 'Katherine', 'James', 'Emily', 'Michael', 'Matthew', 'Laura', 'Kevin', 'Jonas'],
#         'score': [12.5, 9, 16.5, np.nan, 9, 20, 14.5, np.nan, 8, 19],
#         'attempts' : [1, 3, 2, 3, 2, 3, 1, 1, 2, 1],
#         'qualify': ['yes', 'no', 'yes', 'no', 'no', 'yes', 'yes', 'no', 'no', 'yes']}
# labels = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j']

# df = pd.DataFrame(exam_data , index=labels)
# print("Number of attempts in the examination is greater than 2:")
# print(df[df['attempts'] > 2])
# Sample Output:

# Number of attempts in the examination is greater than 2:
#       name  score  attempts qualify
# b     Dima    9.0         3      no
# d    James    NaN         3      no
# f  Michael   20.0         3     yes                                                                                   
# Explanation:


# The above code creates a DataFrame 'df' containing information about a group of students who took an exam. The code then filters the DataFrame to only include rows where the 'attempts' column has a value greater than 2 using the boolean indexing method.

# Finally, print() function prints the filtered DataFrame.

# 8. Counting Rows and Columns

# Write a Pandas program to count the number of rows and columns of a DataFrame.

# Sample DataFrame:
# exam_data = {'name': ['Anastasia', 'Dima', 'Katherine', 'James', 'Emily', 'Michael', 'Matthew', 'Laura', 'Kevin', 'Jonas'],
# 'score': [12.5, 9, 16.5, np.nan, 9, 20, 14.5, np.nan, 8, 19],
# 'attempts': [1, 3, 2, 3, 2, 3, 1, 1, 2, 1],
# 'qualify': ['yes', 'no', 'yes', 'no', 'no', 'yes', 'yes', 'no', 'no', 'yes']}
# labels = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j']

# Sample Solution :-

# Python Code :

# import pandas as pd
# import numpy as np
# exam_data  = {'name': ['Anastasia', 'Dima', 'Katherine', 'James', 'Emily', 'Michael', 'Matthew', 'Laura', 'Kevin', 'Jonas'],
#         'score': [12.5, 9, 16.5, np.nan, 9, 20, 14.5, np.nan, 8, 19],
#         'attempts': [1, 3, 2, 3, 2, 3, 1, 1, 2, 1],
#         'qualify': ['yes', 'no', 'yes', 'no', 'no', 'yes', 'yes', 'no', 'no', 'yes']}
# labels = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j']
# df = pd.DataFrame(exam_data , index=labels)
# total_rows=len(df.axes[0])
# total_cols=len(df.axes[1])
# print("Number of Rows: "+str(total_rows))
# print("Number of Columns: "+str(total_cols))
# Sample Output:

# Number of Rows: 10                                                     
# Number of Columns: 4                
# Explanation:

# The above code creates a pandas dataframe ‘df’ with the given data in ‘exam_data’ dictionary and assigns the labels to rows using labels list. Then it calculates the number of rows and columns in the dataframe using len(df.axes[0]) and len(df.axes[1]) respectively, and stores them in total_rows and total_cols variables.

# Finally, it prints the number of rows and columns using these variables.

# 9. Selecting Rows with Missing Score

# Write a Pandas program to select the rows where the score is missing, i.e. is NaN.

# Sample DataFrame:
# Sample Python dictionary data and list labels:
# exam_data = {'name': ['Anastasia', 'Dima', 'Katherine', 'James', 'Emily', 'Michael', 'Matthew', 'Laura', 'Kevin', 'Jonas'],
# 'score': [12.5, 9, 16.5, np.nan, 9, 20, 14.5, np.nan, 8, 19],
# 'attempts': [1, 3, 2, 3, 2, 3, 1, 1, 2, 1],
# 'qualify': ['yes', 'no', 'yes', 'no', 'no', 'yes', 'yes', 'no', 'no', 'yes']}
# labels = ['a', 'b', 'c', 'd', 'e', 'f', 'g', labels = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j']

# Sample Solution :

# Python Code :

# import pandas as pd
# import numpy as np
# exam_data  = {'name': ['Anastasia', 'Dima', 'Katherine', 'James', 'Emily', 'Michael', 'Matthew', 'Laura', 'Kevin', 'Jonas'],
#         'score': [12.5, 9, 16.5, np.nan, 9, 20, 14.5, np.nan, 8, 19],
#         'attempts': [1, 3, 2, 3, 2, 3, 1, 1, 2, 1],
#         'qualify': ['yes', 'no', 'yes', 'no', 'no', 'yes', 'yes', 'no', 'no', 'yes']}
# labels = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j']

# df = pd.DataFrame(exam_data , index=labels)
# print("Rows where score is missing:")
# print(df[df['score'].isnull()])
# Sample Output:

# Rows where score is missing:
#    attempts   name qualify  score
# d         3  James      no    NaN
# h         1  Laura      no    NaN                              
# Explanation:

# In the above code -

# df = pd.DataFrame(exam_data , index=labels): This line creates a pandas DataFrame called ‘df’ from a dictionary ’exam_data’ with specified index labels. The DataFrame has columns 'name', 'score', 'attempts', and 'qualify' which are created from the corresponding values in the dictionary.

# print(df[df['score'].isnull()]): This code filters the rows in the DataFrame where the 'score' column is null using the isnull() method and prints the resulting subset of the DataFrame using boolean indexing with df[df['score'].isnull()]. This will show the rows where the 'score' column has missing or NaN (not a number) values.