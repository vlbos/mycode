383\. 观光

*    [题目](https://www.acwing.com/problem/content/description/385/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/385/1/)
*    [题解](https://www.acwing.com/problem/content/solution/385/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/385/)

  

“您的个人假期”旅行社组织了一次比荷卢经济联盟的巴士之旅。

比荷卢经济联盟有很多公交线路。

每天公共汽车都会从一座城市开往另一座城市。

沿途汽车可能会在一些城市（零或更多）停靠。

旅行社计划旅途从 SS 城市出发，到 FF 城市结束。

由于不同旅客的景点偏好不同，所以为了迎合更多旅客，旅行社将为客户提供多种不同线路。

游客可以选择的行进路线有所限制，要么满足所选路线总路程为 SS 到 FF 的最小路程，要么满足所选路线总路程仅比最小路程多一个单位长度。

![3463_1.png](https://cdn.acwing.com/media/article/image/2019/02/26/19_75361c2839-3463_1.png)

如上图所示，如果 S\=1，F\=5S\=1，F\=5，则这里有两条最短路线 1→2→5,1→3→51→2→5,1→3→5，长度为 66；有一条比最短路程多一个单位长度的路线 1→3→4→51→3→4→5，长度为 77。

现在给定比荷卢经济联盟的公交路线图以及两个城市 SS 和 FF，请你求出旅行社最多可以为旅客提供多少种不同的满足限制条件的线路。

#### 输入格式

第一行包含整数 TT，表示共有 TT 组测试数据。

每组数据第一行包含两个整数 NN 和 MM，分别表示总城市数量和道路数量。

接下来 MM 行，每行包含三个整数 A,B,LA,B,L，表示有一条线路从城市 AA 通往城市 BB，长度为 LL。

需注意，线路是 **单向的**，存在从 AA 到 BB 的线路不代表一定存在从 BB 到 AA 的线路，另外从城市 AA 到城市 BB 可能存在多个不同的线路。

接下来一行，包含两个整数 SS 和 FF，数据保证 SS 和 FF 不同，并且 S、FS、F 之间至少存在一条线路。

#### 输出格式

每组数据输出一个结果，每个结果占一行。

数据保证结果不超过 109109。

#### 数据范围

2≤N≤10002≤N≤1000,  
1≤M≤100001≤M≤10000,  
1≤L≤10001≤L≤1000，  
1≤A,B,S,F≤N1≤A,B,S,F≤N

#### 输入样例：

    2
    5 8
    1 2 3
    1 3 2
    1 4 5
    2 3 1
    2 5 3
    3 4 2
    3 5 4
    4 5 3
    1 5
    5 6
    2 3 1
    3 2 1
    3 1 10
    4 5 2
    5 2 7
    5 2 7
    4 1
    

#### 输出样例：

    3
    2
    

难度：中等

时/空限制：1s / 64MB

总通过数：6887

总尝试数：14239

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3934&show_algorithm_tags=0)

算法标签

[图论](https://www.acwing.com/problem/search/1/?search_content=%E5%9B%BE%E8%AE%BA&source_file_id=3934&show_algorithm_tags=1)[最短路](https://www.acwing.com/problem/search/1/?search_content=%E6%9C%80%E7%9F%AD%E8%B7%AF&source_file_id=3934&show_algorithm_tags=1)[单源次短路及其条数](https://www.acwing.com/problem/search/1/?search_content=%E5%8D%95%E6%BA%90%E6%AC%A1%E7%9F%AD%E8%B7%AF%E5%8F%8A%E5%85%B6%E6%9D%A1%E6%95%B0&source_file_id=3934&show_algorithm_tags=1)