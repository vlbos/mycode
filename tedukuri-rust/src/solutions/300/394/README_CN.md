394\. 最优高铁环

*    [题目](https://www.acwing.com/problem/content/description/396/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/396/1/)
*    [题解](https://www.acwing.com/problem/content/solution/396/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/396/)

  

幻影国建成了当今世界上最先进的高铁，该国高铁分为以下几类：

*   SS—高速光子动力列车—时速 1000km/h1000km/h
*   GG—高速动车—时速 500km/h500km/h
*   DD—动车组—时速 300km/h300km/h
*   TT—特快—时速 200km/h200km/h
*   KK—快速—时速 150km/h150km/h

该国列车车次标号由上述字母开头，后面跟着一个正整数(≤1000≤1000)构成。

由于该国地形起伏不平，各地铁路的适宜运行速度不同。

因此该国的每一条行车路线都由 KK 列车次构成。

例如：K\=5K\=5 的一条路线为：T120−D135−S1−G12−K856T120−D135−S1−G12−K856。

当某一条路线的末尾车次与另一条路线的开头车次相同时，这两条路线可以连接起来变为一条更长的行车路线。

显然若干条路线连接起来有可能构成一个环。

若有 33 条行车路线分别为:

x1−x2−x3x1−x2−x3  
x3−x4x3−x4  
x4−x5−x1x4−x5−x1

x1∼x5x1∼x5 车次的速度分别为 v1∼v5v1∼v5。

定义高铁环的值为(环上各条行车路线速度和)的平均值，即：

\[(v1+v2+v3)+(v3+v4)+(v4+v5+v1)\]/3\[(v1+v2+v3)+(v3+v4)+(v4+v5+v1)\]/3

所有高铁环的值的最大值称为最优高铁环的值。

给出 MM 条行车路线，求最优高铁环的值。

#### 输入格式

第一行为行车路线条数 MM。

接下来 MM 行每行一条行车路线，由若干车次构成，各车次之间用 `-` 号隔开，车次的标号方式如上所述。

数据保证输入的合法性。

#### 输出格式

输出最优高铁环的值，四舍五入到最接近的整数。

若不存在这样的环，输出 −1−1。

#### 数据范围

0<M≤500000<M≤50000,  
每条行车路线车次个数不超过 2020，数据保证结果不超过 231−1231−1。

#### 输入样例：

    3
    T120-D135-S1
    S1-G12
    G12-K856-T120
    

#### 输出样例：

    1283
    

难度：困难

时/空限制：2s / 64MB

总通过数：388

总尝试数：1957

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3945&show_algorithm_tags=0)

算法标签

[图论](https://www.acwing.com/problem/search/1/?search_content=%E5%9B%BE%E8%AE%BA&source_file_id=3945&show_algorithm_tags=1)[负环判定](https://www.acwing.com/problem/search/1/?search_content=%E8%B4%9F%E7%8E%AF%E5%88%A4%E5%AE%9A&source_file_id=3945&show_algorithm_tags=1)[01分数规划](https://www.acwing.com/problem/search/1/?search_content=01%E5%88%86%E6%95%B0%E8%A7%84%E5%88%92&source_file_id=3945&show_algorithm_tags=1)