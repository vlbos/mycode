126\. 最大的和

*    [题目](https://www.acwing.com/problem/content/description/128/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/128/1/)
*    [题解](https://www.acwing.com/problem/content/solution/128/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/128/)

  

给定一个包含整数的二维矩阵，子矩形是位于整个阵列内的任何大小为 1×11×1 或更大的连续子阵列。

矩形的总和是该矩形中所有元素的总和。

在这个问题中，具有最大和的子矩形被称为最大子矩形。

例如，下列数组：

    0 -2 -7 0 
    9 2 -6 2 
    -4 1 -4 1 
    -1 8 0 -2 
    

其最大子矩形为：

    9 2 
    -4 1 
    -1 8 
    

它拥有最大和 1515。

#### 输入格式

输入中将包含一个 N×NN×N 的整数数组。

第一行只输入一个整数 NN，表示方形二维数组的大小。

从第二行开始，输入由空格和换行符隔开的 N2N2 个整数，它们即为二维数组中的 N2N2 个元素，输入顺序从二维数组的第一行开始向下逐行输入，同一行数据从左向右逐个输入。

数组中的数字会保持在 \[−127,127\]\[−127,127\] 的范围内。

#### 输出格式

输出一个整数，代表最大子矩形的总和。

#### 数据范围

1≤N≤1001≤N≤100

#### 输入样例：

    4
    0 -2 -7 0 9 2 -6 2
    -4 1 -4  1 -1
    
    8  0 -2
    

#### 输出样例：

    15
    

难度：简单

时/空限制：1s / 64MB

总通过数：6585

总尝试数：11752

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3677&show_algorithm_tags=0)[《信息学奥赛一本通》语言及算法基础篇](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E4%BF%A1%E6%81%AF%E5%AD%A6%E5%A5%A5%E8%B5%9B%E4%B8%80%E6%9C%AC%E9%80%9A%E3%80%8B%E8%AF%AD%E8%A8%80%E5%8F%8A%E7%AE%97%E6%B3%95%E5%9F%BA%E7%A1%80%E7%AF%87&source_file_id=3677&show_algorithm_tags=0)

算法标签

[枚举](https://www.acwing.com/problem/search/1/?search_content=%E6%9E%9A%E4%B8%BE&source_file_id=3677&show_algorithm_tags=1)[贪心](https://www.acwing.com/problem/search/1/?search_content=%E8%B4%AA%E5%BF%83&source_file_id=3677&show_algorithm_tags=1)