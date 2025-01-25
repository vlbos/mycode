350\. 巡逻

*    [题目](https://www.acwing.com/problem/content/description/352/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/352/1/)
*    [题解](https://www.acwing.com/problem/content/solution/352/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/352/)

  

在一个地区有 nn 个村庄，编号为 1,2,…,n1,2,…,n。

有 n−1n−1 条道路连接着这些村庄，每条道路刚好连接两个村庄，从任何一个村庄，都可以通过这些道路到达其他任一个村庄。

每条道路的长度均为 11 个单位。

为保证该地区的安全，巡警车每天都要到所有的道路上巡逻。

警察局设在编号为 11 的村庄里，每天巡警车总是从警局出发，最终又回到警局。

为了减少总的巡逻距离，该地区准备在这些村庄之间建立 KK 条新的道路，每条新道路可以连接任意两个村庄。

两条新道路可以在同一个村庄会合或结束，甚至新道路可以是一个环。

因为资金有限，所以 KK 只能为 11 或 22。

同时，为了不浪费资金，每天巡警车必须经过新建的道路正好一次。

编写一个程序，在给定村庄间道路信息和需要新建的道路数的情况下，计算出最佳的新建道路的方案，使得总的巡逻距离最小。

#### 输入格式

第一行包含两个整数 nn 和 KK。

接下来 n−1n−1 行每行两个整数 aa 和 bb，表示村庄 aa 和 bb 之间有一条道路。

#### 输出格式

输出一个整数，表示新建了 KK 条道路后能达到的最小巡逻距离。

#### 数据范围

3≤n≤1000003≤n≤100000,  
1≤K≤21≤K≤2,  
1≤a,b≤n1≤a,b≤n

#### 输入样例：

    8 1 
    1 2 
    3 1 
    3 4 
    5 3 
    7 5 
    8 5 
    5 6 
    

#### 输出样例：

    11
    

难度：简单

时/空限制：1s / 64MB

总通过数：1875

总尝试数：6917

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3901&show_algorithm_tags=0)

算法标签

[图论](https://www.acwing.com/problem/search/1/?search_content=%E5%9B%BE%E8%AE%BA&source_file_id=3901&show_algorithm_tags=1)[树的直径](https://www.acwing.com/problem/search/1/?search_content=%E6%A0%91%E7%9A%84%E7%9B%B4%E5%BE%84&source_file_id=3901&show_algorithm_tags=1)