137\. 雪花雪花雪花

*    [题目](https://www.acwing.com/problem/content/description/139/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/139/1/)
*    [题解](https://www.acwing.com/problem/content/solution/139/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/139/)

  

有 NN 片雪花，每片雪花由六个角组成，每个角都有长度。

第 ii 片雪花六个角的长度从某个角开始顺时针依次记为 ai,1,ai,2,…,ai,6ai,1,ai,2,…,ai,6。

因为雪花的形状是封闭的环形，所以从任何一个角开始顺时针或逆时针往后记录长度，得到的六元组都代表形状相同的雪花。

例如 ai,1,ai,2,…,ai,6ai,1,ai,2,…,ai,6 和 ai,2,ai,3,…,ai,6，ai,1ai,2,ai,3,…,ai,6，ai,1 就是形状相同的雪花。

ai,1,ai,2,…,ai,6ai,1,ai,2,…,ai,6 和 ai,6,ai,5,…,ai,1ai,6,ai,5,…,ai,1 也是形状相同的雪花。

我们称两片雪花形状相同，当且仅当它们各自从某一角开始顺时针或逆时针记录长度，能得到两个相同的六元组。

求这 NN 片雪花中是否存在两片形状相同的雪花。

#### 输入格式

第一行输入一个整数 NN，代表雪花的数量。

接下来 NN 行，每行描述一片雪花。

每行包含 66 个整数，分别代表雪花的六个角的长度（这六个数即为从雪花的随机一个角顺时针或逆时针记录长度得到）。

同行数值之间，用空格隔开。

#### 输出格式

如果不存在两片形状相同的雪花，则输出：

`No two snowflakes are alike.`

如果存在两片形状相同的雪花，则输出：

`Twin snowflakes found.`

#### 数据范围

1≤N≤1000001≤N≤100000,  
0≤ai,j<100000000≤ai,j<10000000

#### 输入样例：

    2
    1 2 3 4 5 6
    4 3 2 1 6 5
    

#### 输出样例：

    Twin snowflakes found.
    

难度：简单

时/空限制：1s / 64MB

总通过数：5545

总尝试数：19060

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3688&show_algorithm_tags=0)[2007 CCO Day1](https://www.acwing.com/problem/search/1/?search_content=2007%20CCO%20Day1&source_file_id=3688&show_algorithm_tags=0)

算法标签

[哈希表](https://www.acwing.com/problem/search/1/?search_content=%E5%93%88%E5%B8%8C%E8%A1%A8&source_file_id=3688&show_algorithm_tags=1)[字符串的最小表示法](https://www.acwing.com/problem/search/1/?search_content=%E5%AD%97%E7%AC%A6%E4%B8%B2%E7%9A%84%E6%9C%80%E5%B0%8F%E8%A1%A8%E7%A4%BA%E6%B3%95&source_file_id=3688&show_algorithm_tags=1)