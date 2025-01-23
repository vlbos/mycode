109\. 天才ACM

*    [题目](https://www.acwing.com/problem/content/description/111/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/111/1/)
*    [题解](https://www.acwing.com/problem/content/solution/111/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/111/)

  

给定一个整数 MM，对于任意一个整数集合 SS，定义“校验值”如下:

从集合 SS 中取出 MM 对数(即 2×M2×M 个数，不能重复使用集合中的数，如果 SS 中的整数不够 MM 对，则取到不能取为止)，使得“每对数的差的平方”之和最大，这个最大值就称为集合 SS 的“校验值”。

现在给定一个长度为 NN 的数列 AA 以及一个整数 TT。

我们要把 AA 分成若干段，使得每一段的“校验值”都不超过 TT。

求最少需要分成几段。

#### 输入格式

第一行输入整数 KK，代表有 KK 组测试数据。

对于每组测试数据，第一行包含三个整数 N,M,TN,M,T 。

第二行包含 NN 个整数，表示数列A1,A2…ANA1,A2…AN。

#### 输出格式

对于每组测试数据，输出其答案，每个答案占一行。

#### 数据范围

1≤K≤121≤K≤12,  
1≤N,M≤5000001≤N,M≤500000,  
0≤T≤10180≤T≤1018,  
0≤Ai≤2200≤Ai≤220

#### 输入样例：

    2
    5 1 49
    8 2 1 7 9
    5 1 64
    8 2 1 7 9
    

#### 输出样例：

    2
    1
    

难度：困难

时/空限制：10s / 64MB

总通过数：5049

总尝试数：18612

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3660&show_algorithm_tags=0)

算法标签

[倍增](https://www.acwing.com/problem/search/1/?search_content=%E5%80%8D%E5%A2%9E&source_file_id=3660&show_algorithm_tags=1)