# 70. Convert Continuous Column to Categorical

# Write a Pandas program to convert continuous values of a column in a given DataFrame to categorical.

# Input:
# { 'Name': ['Alberto Franco','Gino Mcneill','Ryan Parkes', 'Eesha Hinton', 'Syed Wharton'], 'Age': [18, 22, 40, 50, 80, 5] }
# Output:
# Age group:
# 0 kids
# 1 adult
# 2 elderly
# 3 adult
# 4 elderly
# 5 kids
# Name: age_groups, dtype: category
# Categories (3, object): [kids < adult < elderly]

# Sample Solution :

# Python Code :

# import pandas as pd
# df = pd.DataFrame({
#     'name': ['Alberto Franco','Gino Mcneill','Ryan Parkes', 'Eesha Hinton', 'Syed Wharton', 'Kierra Gentry'],
#       'age': [18, 22, 85, 50, 80, 5]
# })
# print("Original DataFrame:")
# print(df)
# print('\nAge group:')
# df["age_groups"] = pd.cut(df["age"], bins = [0, 18, 65, 99], labels = ["kids", "adult", "elderly"])
# print(df["age_groups"])
# Sample Output:

# Original DataFrame:
#              name  age
# 0  Alberto Franco   18
# 1    Gino Mcneill   22
# 2     Ryan Parkes   85
# 3    Eesha Hinton   50
# 4    Syed Wharton   80
# 5   Kierra Gentry    5

# Age group:
# 0       kids
# 1      adult
# 2    elderly
# 3      adult
# 4    elderly
# 5       kids
# Name: age_groups, dtype: category
# Categories (3, object): [kids < adult < elderly]


# 71. Display Memory Usage of DataFrame and Columns

# Write a Pandas program to display memory usage of a given DataFrame and every column of the DataFrame.

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
# print("\nGlobal usage of memory of the DataFrame:")
# print(df.info(memory_usage = "deep"))
# print("\nThe usage of memory of every column of the said DataFrame:")
# print(df.memory_usage(deep = True))
# Sample Output:

# Original DataFrame:
#              Name Date_Of_Birth    Age
# 0  Alberto Franco     17/05/2002  18.5
# 1    Gino Mcneill     16/02/1999  21.2
# 2     Ryan Parkes     25/09/1998  22.5
# 3    Eesha Hinton     11/05/2002  22.0
# 4    Syed Wharton     15/09/1997  23.0

# Global usage of memory of the DataFrame:
# <class 'pandas.core.frame.DataFrame'>
# RangeIndex: 5 entries, 0 to 4
# Data columns (total 3 columns):
# Name              5 non-null object
# Date_Of_Birth     5 non-null object
# Age               5 non-null float64
# dtypes: float64(1), object(2)
# memory usage: 801.0 bytes
# None

# The usage of memory of every column of the said DataFrame:
# Index              80
# Name              346
# Date_Of_Birth     335
# Age                40
# dtype: int64


# 72. Combine Many Series to Create a DataFrame

# Write a Pandas program to combine many given series to create a DataFrame.

# Sample Solution :

# Python Code :

# import pandas as pd
# sr1 = pd.Series(['php', 'python', 'java', 'c#', 'c++'])
# sr2 = pd.Series([1, 2, 3, 4, 5])
# print("Original Series:")
# print(sr1)
# print(sr2)
# print("Combine above series to a dataframe:")
# ser_df = pd.DataFrame(sr1, sr2).reset_index()
# print(ser_df.head())
# print("\nUsing pandas concat:")
# ser_df = pd.concat([sr1, sr2], axis = 1)
# print(ser_df.head())
# print("\nUsing pandas DataFrame with a dictionary, gives a specific name to the columns:")
# ser_df = pd.DataFrame({"col1":sr1, "col2":sr2})
# print(ser_df.head(5))
# Sample Output:

# Original Series:
# 0       php
# 1    python
# 2      java
# 3        c#
# 4       c++
# dtype: object
# 0    1
# 1    2
# 2    3
# 3    4
# 4    5
# dtype: int64
# Combine above series to a dataframe:
#    index       0
# 0      1  python
# 1      2    java
# 2      3      c#
# 3      4     c++
# 4      5     NaN

# Using pandas concat:
#         0  1
# 0     php  1
# 1  python  2
# 2    java  3
# 3      c#  4
# 4     c++  5

# Using pandas DataFrame with a dictionary, gives a specific name to the columns:
#      col1  col2
# 0     php     1
# 1  python     2
# 2    java     3
# 3      c#     4
# 4     c++     5


# 73. Create DataFrames with Mixed Values

# Write a Pandas program to create DataFrames that contains random values, contains missing values, contains datetime values and contains mixed values.

# Sample Solution :

# Python Code :

# import pandas as pd
# print("DataFrame: Contains random values:")
# df1 = pd.util.testing.makeDataFrame() # contains random values
# print(df1)
# print("\nDataFrame: Contains missing values:")
# df2 = pd.util.testing.makeMissingDataframe() # contains missing values
# print(df2)
# print("\nDataFrame: Contains datetime values:")
# df3 = pd.util.testing.makeTimeDataFrame() # contains datetime values
# print(df3)
# print("\nDataFrame: Contains mixed values:")
# df4 = pd.util.testing.makeMixedDataFrame() # contains mixed values
# print(df4)
# Sample Output:

# DataFrame: Contains random values:
#                    A         B         C         D
# Dog2w4Dv4l  0.591058  1.883454 -1.608613 -0.502516
# kV7mfdFcF9  0.629642 -0.474377  0.567357  1.658445
# Il2etcpmi6 -0.650443 -1.135115 -0.125597  1.786536
# JdSXf3MEyq -1.679493  0.628239 -0.749637  0.852839
# 2H7lGkxiwL  1.186363 -0.615328  0.080556 -1.955239
# jR009ZtfA4 -0.620729  0.844086 -0.143764  0.472620
# baAWDkptTk  0.159193 -0.506624 -0.940083 -1.139910
# 8z1f7y6yzu  0.043180 -0.267833  0.431444  0.874783
# P9ZUxqpuJA -0.939453 -1.922785 -0.527641 -0.308326
# T4N91lVewM -0.013433 -0.252278  0.774136 -1.824968
# 7McfxCARW0  1.015361 -0.597383 -1.017453 -1.020482
# 8I59Iy2tV7  2.429052  0.441168 -0.215161 -0.333973
# jHxyr4Htsh -0.344973  0.070246  1.134062 -0.016310
# lyMSJjL3fE -0.383133  1.142060 -0.437973  0.372100
# iAksZz4YPH -0.189774  1.399061  1.294249  1.220887
# jcILDH1uSb  1.208005  0.031609  1.058339 -1.490341
# uLXp1wu84s  0.289758  0.428422  0.356415  0.643879
# Ie8ubHzNbh  1.699736 -0.018321 -0.670926  1.145490
# n4TmM5kPCA  0.122721 -0.890217 -0.980098 -0.338159
# CtdL5x1ofR  1.375652 -1.148859 -0.198355 -2.045092
# WqggnU8U1w  0.171769  1.276065  0.474320  0.126961
# UOCLGy3MJI -0.508391 -0.755753  0.239499  0.484506
# wZYF0HwbEY -1.061641 -0.923209  0.394357 -0.843273
# JP6QFva9u9 -0.022757  1.238850 -0.607959  1.645612
# r02ts3PRSV  0.050639 -1.016244  0.330882 -1.161764
# I8lMHDtdEa -0.848674  0.207307 -0.021109  1.421939
# rg1rThlQ4o -0.670269  0.853271 -0.384838  0.350151
# 4P5Xq4rxcL  1.041481 -2.341787 -1.157728  0.497949
# Oy6e83TXcQ -1.259630  0.433061  0.893792 -1.427895
# C7Zz3C0Jq5 -0.802454  1.001237 -2.233028  0.061644

# DataFrame: Contains missing values:
#                    A         B         C         D
# i6i6Xn9l9c -0.299335  0.410871 -0.431840 -0.302177
# OGo5KNNYNJ -0.174594 -1.366146  0.435063 -2.779446
# u0mG9q1L7C  1.019094 -0.061077 -1.138138 -0.218460
# RNJGqpci4o -0.380815  0.189970 -2.148521 -1.163589
# vXIcxItZ1D       NaN -0.079448  0.604777  0.065290
# arou6zhX6q       NaN       NaN -0.827082 -0.377132
# BkcUNAyKII  0.196885  0.164628 -0.872416  0.578590
# Nar3sV5Z01 -0.269490       NaN -1.914949 -2.492530
# Sa6BpjQpms -0.035106 -0.531400  0.328387  0.463325
# eLlmKur2R2       NaN  1.252522  0.384160 -0.292494
# 4ZGLI9N5GI  1.103449  0.140680  0.101512 -0.117461
# 8JpVrcZRCz       NaN -1.228597 -0.889428  1.019362
# 3ww3qKh37f  1.678527  0.011843  0.405760  1.158411
# QlGQoxSVT6  0.763349  1.743806 -1.564245 -1.198915
# wrvoGhUQAd  1.045789  0.432039  0.593760  0.635557
# oKApKm6NcZ       NaN  0.561950 -1.064052 -2.380983
# Ka87bUAT3j -1.243862  0.681610 -0.018944 -1.127184
# O7zz89V5e0  0.132516  0.506075 -1.001728 -1.369704
# EE4Z8p7SzC  0.274650 -0.552164 -0.478510 -1.182832
# wWxAn2q4RD -0.829835       NaN  0.496359       NaN
# vzFsnyObuX -1.602297 -2.086616  1.329253  1.463064
# QtVb9b3gDQ  0.153850  0.799016  1.701532 -0.141876
# Vf6t2LO2Io  0.936485 -0.835217       NaN -0.560338
# ZEXVM5SUdU  1.733719  0.086513  0.562900  0.352225
# 5AvgYYFP05 -0.904654  0.401132 -0.478490  1.390773
# EngKTbWqSQ -2.172282 -0.749352 -1.243691  0.217420
# rgsi1atINq -1.548443  0.676526 -1.315938  1.314064
# zL9042RbHi       NaN       NaN -0.085687  0.303308
# uz3laJaCIw -1.390233 -0.822796 -0.132600 -1.138293
# f7myQshpvh  0.027210 -0.173178 -0.108948  0.738018

# DataFrame: Contains datetime values:
#                    A         B         C         D
# 2000-01-03  0.665402  0.860808 -0.180986 -0.970889
# 2000-01-04 -1.511533  0.487539 -0.710355 -0.807816
# 2000-01-05 -0.773294  0.197918 -1.214035  1.049529
# 2000-01-06 -1.074894  1.774147 -0.620025  0.740779
# 2000-01-07 -0.714355  0.330679  0.497667 -0.375923
# 2000-01-10 -0.060936  0.677847  0.686886  1.351782
# 2000-01-11 -1.692036 -0.470830 -0.249142  0.541105
# 2000-01-12 -0.077213 -0.592206 -0.132603 -0.656798
# 2000-01-13 -2.407360 -0.709951 -0.620317 -0.593090
# 2000-01-14 -0.243385 -1.654542  0.487391  0.595058
# 2000-01-17  0.139514  0.583979  0.211791 -1.809909
# 2000-01-18 -1.185097  2.688730  1.105632  0.322994
# 2000-01-19 -0.647685 -0.380803  0.056086 -1.299670
# 2000-01-20  0.781133  1.074446 -1.145552 -0.648223
# 2000-01-21 -0.428875  0.402555  1.735354 -1.230331
# 2000-01-24  1.282698  1.506384 -2.726718  0.480689
# 2000-01-25 -0.059287 -0.952011  0.066330  0.897042
# 2000-01-26 -1.503653 -1.689130 -0.488598 -0.890888
# 2000-01-27 -0.464802  0.250585 -1.462912  1.789611
# 2000-01-28 -1.213504  0.304826 -0.190335 -0.693164
# 2000-01-31 -0.565728 -1.317228 -1.707892 -0.404228
# 2000-02-01  0.160620  1.689041  0.171084 -0.004823
# 2000-02-02 -1.251242  2.242914 -0.430506 -0.042091
# 2000-02-03 -1.721439 -0.159966  1.523550 -0.742485
# 2000-02-04  0.002191  0.708701  0.029411  0.319738
# 2000-02-07  0.541060  0.905438  0.452724 -0.849368
# 2000-02-08  0.335644  1.776628  0.173110 -0.847064
# 2000-02-09  1.139137 -0.850207  0.718282  0.903825
# 2000-02-10  0.079852 -1.303238  1.400994 -0.560761
# 2000-02-11  1.496111  0.143146  0.509362  1.206039

# DataFrame: Contains mixed values:
#      A    B     C          D
# 0  0.0  0.0  foo1 2009-01-01
# 1  1.0  1.0  foo2 2009-01-02
# 2  2.0  0.0  foo3 2009-01-05
# 3  3.0  1.0  foo4 2009-01-06
# 4  4.0  0.0  foo5 2009-01-07

# 74. Fill Missing Values in Time Series Data

# Write a Pandas program to fill missing values in time series data.

# From Wikipedia , in the mathematical field of numerical analysis, interpolation is a type of estimation, a method of constructing new data points within the range of a discrete set of known data points.

# Sample Solution :

# Python Code :

# import pandas as pd
# import numpy as np
# sdata = {"c1":[120, 130 ,140, 150, np.nan, 170], "c2":[7, np.nan, 10, np.nan, 5.5, 16.5]}
# df = pd.DataFrame(sdata)
# df.index = pd.util.testing.makeDateIndex()[0:6]
# print("Original DataFrame:")
# print(df)
# print("\nDataFrame after interpolate:")
# print(df.interpolate())
# Sample Output:

# Original DataFrame:
#                c1    c2
# 2000-01-03  120.0   7.0
# 2000-01-04  130.0   NaN
# 2000-01-05  140.0  10.0
# 2000-01-06  150.0   NaN
# 2000-01-07    NaN   5.5
# 2000-01-10  170.0  16.5

# DataFrame after interpolate:
#                c1     c2
# 2000-01-03  120.0   7.00
# 2000-01-04  130.0   8.50
# 2000-01-05  140.0  10.00
# 2000-01-06  150.0   7.75
# 2000-01-07  160.0   5.50
# 2000-01-10  170.0  16.50


# 75. Use Local Variable Within a Query

# Write a Pandas program to use a local variable within a query.

# Sample Solution :

# Python Code :

# import pandas as pd
# df = pd.DataFrame({'W':[68,75,86,80,66],'X':[78,85,96,80,86], 'Y':[84,94,89,83,86],'Z':[86,97,96,72,83]});
# print("Original DataFrame")
# print(df)
# maxx = df["W"].max()
# print("\nValues which are less than maximum value of 'W' column")
# print(df.query("W < @maxx"))
# Sample Output:

# Original DataFrame
#     W   X   Y   Z
# 0  68  78  84  86
# 1  75  85  94  97
# 2  86  96  89  96
# 3  80  80  83  72
# 4  66  86  86  83

# Values which are less than maximum value of 'W' column
#     W   X   Y   Z
# 0  68  78  84  86
# 1  75  85  94  97
# 3  80  80  83  72
# 4  66  86  86  83


# 76. Clean Object Column with Regex

# Write a Pandas program to clean object column with mixed data of a given DataFrame using regular expression.

# Sample Solution :

# Python Code :

# import pandas as pd
# d = {"agent": ["a001", "a002", "a003", "a003", "a004"], "purchase":[4500.00, 7500.00, "$3000.25", "$1250.35", "9000.00"]}
# df = pd.DataFrame(d)
# print("Original dataframe:")
# print(df)
# print("\nData Types:")
# print(df["purchase"].apply(type))
# df["purchase"] = df["purchase"].replace("[$,]", "", regex = True).astype("float")
# print("\nNew Data Types:")
# print(df["purchase"].apply(type))            
# Sample Output:

# Original dataframe:
#   agent  purchase
# 0  a001      4500
# 1  a002      7500
# 2  a003  $3000.25
# 3  a003  $1250.35
# 4  a004   9000.00

# Data Types:
# 0    <class 'float'>
# 1    <class 'float'>
# 2      <class 'str'>
# 3      <class 'str'>
# 4      <class 'str'>
# Name: purchase, dtype: object

# New Data Types:
# 0    <class 'float'>
# 1    <class 'float'>
# 2    <class 'float'>
# 3    <class 'float'>
# 4    <class 'float'>
# Name: purchase, dtype: object

# 77. Get Numeric Representation from Distinct Values

# Write a Pandas program to get the numeric representation of an array by identifying distinct values of a given column of a DataFrame.

# Sample Solution :

# Python Code :

# import pandas as pd
# df = pd.DataFrame({
#     'Name': ['Alberto Franco','Gino Mcneill','Ryan Parkes', 'Eesha Hinton', 'Gino Mcneill'],
#     'Date_Of_Birth ': ['17/05/2002','16/02/1999','25/09/1998','11/05/2002','15/09/1997'],
#     'Age': [18.5, 21.2, 22.5, 22, 23]
# })
# print("Original DataFrame:")
# print(df)
# label1, unique1 = pd.factorize(df['Name'])
# print("\nNumeric representation of an array by identifying distinct values:")
# print(label1)
# print(unique1)
# Sample Output:

# Original DataFrame:
#              Name Date_Of_Birth    Age
# 0  Alberto Franco     17/05/2002  18.5
# 1    Gino Mcneill     16/02/1999  21.2
# 2     Ryan Parkes     25/09/1998  22.5
# 3    Eesha Hinton     11/05/2002  22.0
# 4    Gino Mcneill     15/09/1997  23.0

# Numeric representation of an array by identifying distinct values:
# [0 1 2 3 1]
# Index(['Alberto Franco', 'Gino Mcneill', 'Ryan Parkes', 'Eesha Hinton'], dtype='object')


# 78. Replace Value Based on Last Largest Value

# Write a Pandas program to replace the current value in a dataframe column based on last largest value. If the current value is less than last largest value replaces the value with 0.

# Test data: rnum
# 0 23
# 1 21
# 2 27
# 3 22
# 4 34
# 5 33
# 6 34
# 7 31
# 8 25
# 9 22
# 10 34
# 11 19
# 12 31
# 13 32
# 14 19

# Sample Solution :

# Python Code :

# import pandas as pd
# df1=pd.DataFrame({'rnum':[23, 21, 27, 22, 34, 33, 34, 31, 25, 22, 34, 19, 31, 32, 19]})
# print("Original DataFrame:")
# print(df1)
# df1['rnum']=df1.rnum.where(df1.rnum.eq(df1.rnum.cummax()),0)
# print("\nReplace current value in a dataframe column based on last largest value:")
# print(df1)
# Sample Output:

# Original DataFrame:
#     rnum
# 0     23
# 1     21
# 2     27
# 3     22
# 4     34
# 5     33
# 6     34
# 7     31
# 8     25
# 9     22
# 10    34
# 11    19
# 12    31
# 13    32
# 14    19

# Replace current value in a dataframe column based on last largest value:
#     rnum
# 0     23
# 1      0
# 2     27
# 3      0
# 4     34
# 5      0
# 6     34
# 7      0
# 8      0
# 9      0
# 10    34
# 11     0
# 12     0
# 13     0
# 14     0


# 79. Create DataFrame from Clipboard

# Write a Pandas program to create a DataFrame from the clipboard (data from an Excel spreadsheet or a Google Sheet).

# Sample Excel Data:

# Python Exercises: Sample Excel data.
# Sample Solution :

# Python Code :

# import pandas as pd
# df = pd.read_clipboard()
# print("Data from clipboard to DataFrame:")
# print(df)
# Sample Output:

# Data from clipboard to DataFrame:
#    1  2  3  4
# 0  2  3  4  5
# 1  4  5  1  0
# 2  2  3  7  8


# 80. Check Inequality of Two DataFrames

# Write a Pandas program to check for inequality of two given DataFrames.

# Sample Solution :

# Python Code :

# import pandas as pd
# df1 = pd.DataFrame({'W':[68,75,86,80,None],'X':[78,85,None,80,86], 'Y':[84,94,89,83,86],'Z':[86,97,96,72,83]});
# df2 = pd.DataFrame({'W':[78,75,86,80,None],'X':[78,85,96,80,76], 'Y':[84,84,89,83,86],'Z':[86,97,96,72,83]});
# print("Original DataFrames:")
# print(df1)
# print(df2)
# print("\nCheck for inequality of the said dataframes:")
# print(df1.ne(df2))
# Sample Output:

# Original DataFrames:
#       W     X   Y   Z
# 0  68.0  78.0  84  86
# 1  75.0  85.0  94  97
# 2  86.0   NaN  89  96
# 3  80.0  80.0  83  72
# 4   NaN  86.0  86  83
#       W   X   Y   Z
# 0  78.0  78  84  86
# 1  75.0  85  84  97
# 2  86.0  96  89  96
# 3  80.0  80  83  72
# 4   NaN  76  86  83

# Check for inequality of the said dataframes:
#        W      X      Y      Z
# 0   True  False  False  False
# 1  False  False   True  False
# 2  False   True  False  False
# 3  False  False  False  False
# 4   True   True  False  False


# 81. Get Lowest n Records Within Each Group

# Write a Pandas program to get lowest n records within each group of a given DataFrame.

# Sample Solution :

# Python Code :

# import pandas as pd
# d = {'col1': [1, 2, 3, 4, 7, 11], 'col2': [4, 5, 6, 9, 5, 0], 'col3': [7, 5, 8, 12, 1,11]}
# df = pd.DataFrame(data=d)
# print("Original DataFrame")
# print(df)
# print("\nLowest n records within each group of a DataFrame:")
# df1 = df.nsmallest(3, 'col1')
# print(df1)
# df2 = df.nsmallest(3, 'col2')
# print(df2)
# df3 = df.nsmallest(3, 'col3')
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

# Lowest n records within each group of a DataFrame:
#    col1  col2  col3
# 0     1     4     7
# 1     2     5     5
# 2     3     6     8
#    col1  col2  col3
# 5    11     0    11
# 0     1     4     7
# 1     2     5     5
#    col1  col2  col3
# 4     7     5     1
# 1     2     5     5
# 0     1     4     7



