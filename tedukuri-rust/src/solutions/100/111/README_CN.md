111\. 畜栏预定

*    [题目](https://www.acwing.com/problem/content/description/113/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/113/1/)
*    [题解](https://www.acwing.com/problem/content/solution/113/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/113/)

  

有 NN 头牛在畜栏中吃草。

每个畜栏在同一时间段只能提供给一头牛吃草，所以可能会需要多个畜栏。

给定 NN 头牛和每头牛开始吃草的时间 AA 以及结束吃草的时间 BB，每头牛在 \[A,B\]\[A,B\] 这一时间段内都会一直吃草。

当两头牛的吃草区间存在交集时（**包括端点**），这两头牛不能被安排在同一个畜栏吃草。

求需要的最小畜栏数目和每头牛对应的畜栏方案。

#### 输入格式

第 11 行：输入一个整数 NN。

第 2..N+12..N+1 行：第 i+1i+1 行输入第 ii 头牛的开始吃草时间 AA 以及结束吃草时间 BB，数之间用空格隔开。

#### 输出格式

第 11 行：输出一个整数，代表所需最小畜栏数。

第 2..N+12..N+1 行：第 i+1i+1 行输出第 ii 头牛被安排到的畜栏编号，编号是从 11 开始的 **连续** 整数，只要方案合法即可。

#### 数据范围

1≤N≤500001≤N≤50000,  
1≤A,B≤10000001≤A,B≤1000000

#### 输入样例：

    5
    1 10
    2 4
    3 6
    5 8
    4 7
    

#### 输出样例：

    4
    1
    2
    3
    2
    4
    

难度：简单

时/空限制：1s / 64MB

总通过数：5710

总尝试数：14460

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3662&show_algorithm_tags=0)

算法标签

[贪心](https://www.acwing.com/problem/search/1/?search_content=%E8%B4%AA%E5%BF%83&source_file_id=3662&show_algorithm_tags=1)