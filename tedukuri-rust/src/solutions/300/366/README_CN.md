366\. 看牛

*    [题目](https://www.acwing.com/problem/content/description/368/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/368/1/)
*    [题解](https://www.acwing.com/problem/content/solution/368/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/368/)

  

给定 NN 个点 MM 条边的无向图，求一条路径，从节点 11 出发，最后回到节点 11，并且满足每条边恰好被沿着正、反两个方向分别经过一次。

若有多种方案，输出任意一种即可。

#### 输入格式

第一行包含两个整数 NN 和 MM。

接下来 MM 行每行包含两个整数 aa 和 bb，表示点 aa 和点 bb 之间存在一条边。

#### 输出格式

共 2M+12M+1 行，每行包含一个整数，共同描述出了满足条件的一条路径。

#### 数据范围

1≤N≤1041≤N≤104,  
1≤M≤5∗1041≤M≤5∗104

#### 输入样例：

    4 5
    1 2
    1 4
    2 3
    2 4
    3 4
    

#### 输出样例：

    1
    2
    3
    4
    2
    1
    4
    3
    2
    4
    1
    

难度：简单

时/空限制：1s / 64MB

总通过数：1090

总尝试数：2171

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3917&show_algorithm_tags=0)

算法标签

[图论](https://www.acwing.com/problem/search/1/?search_content=%E5%9B%BE%E8%AE%BA&source_file_id=3917&show_algorithm_tags=1)[欧拉路](https://www.acwing.com/problem/search/1/?search_content=%E6%AC%A7%E6%8B%89%E8%B7%AF&source_file_id=3917&show_algorithm_tags=1)