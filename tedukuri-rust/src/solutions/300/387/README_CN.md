387\. 北极网络

*    [题目](https://www.acwing.com/problem/content/description/389/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/389/1/)
*    [题解](https://www.acwing.com/problem/content/solution/389/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/389/)

  

国防部（DND）希望通过无线网络连接几个北部前哨站。

在建立网络时将使用两种不同的通信技术：每个前哨站都有一个无线电收发器，一些前哨站还有一个通信卫星。

任意两个拥有通信卫星的前哨站不论它们的位置如何，都可以通过卫星进行通信。

而如果利用无线电进行通信，则需要两个前哨站的距离不能超过 DD 方可进行通信。

而 DD 的大小取决于收发器的功率，收发器的功率越大，DD 也就越大，但是需要的成本也就越高。

出于采购和维护的考虑，所有的前哨站都采用相同的收发器，也就是说所有前哨站的无线电通信距离 DD 都是相同的。

你需要确定在保证任意两个前哨站之间都能进行通信（直接或间接）的情况下，DD 的最小值是多少。

#### 输入格式

第一行包含整数 NN，表示共有 NN 组测试数据。

每组数据的第一行包含两个整数 SS 和 PP，其中 SS 为卫星个数，PP 为前哨站个数。

接下来 PP 行每行包含两个整数 xx 和 yy，分别表示一个前哨站的横纵坐标。

#### 输出格式

输出一个实数，表示 DD 的最小值，结果保留两位小数。

#### 数据范围

1≤S≤1001≤S≤100,  
S≤P≤500S≤P≤500,  
0≤x,y≤100000≤x,y≤10000

#### 输入样例：

    1
    2 4
    0 100
    0 300
    0 600
    150 750
    

#### 输出样例：

    212.13
    

难度：困难

时/空限制：1s / 64MB

总通过数：764

总尝试数：2048

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3938&show_algorithm_tags=0)[POJ2349](https://www.acwing.com/problem/search/1/?search_content=POJ2349&source_file_id=3938&show_algorithm_tags=0)[kuangbin专题](https://www.acwing.com/problem/search/1/?search_content=kuangbin%E4%B8%93%E9%A2%98&source_file_id=3938&show_algorithm_tags=0)

算法标签

[图论](https://www.acwing.com/problem/search/1/?search_content=%E5%9B%BE%E8%AE%BA&source_file_id=3938&show_algorithm_tags=1)[最小生成树](https://www.acwing.com/problem/search/1/?search_content=%E6%9C%80%E5%B0%8F%E7%94%9F%E6%88%90%E6%A0%91&source_file_id=3938&show_algorithm_tags=1)