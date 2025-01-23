173\. 矩阵距离

*    [题目](https://www.acwing.com/problem/content/description/175/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/175/1/)
*    [题解](https://www.acwing.com/problem/content/solution/175/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/175/)

  

给定一个 NN 行 MM 列的 0101 矩阵 AA，A\[i\]\[j\]A\[i\]\[j\] 与 A\[k\]\[l\]A\[k\]\[l\] 之间的曼哈顿距离定义为：

dist(i,j,k,l)\=|i−k|+|j−l|dist(i,j,k,l)\=|i−k|+|j−l|

输出一个 NN 行 MM 列的整数矩阵 BB，其中：

B\[i\]\[j\]\=min1≤x≤N,1≤y≤M,A\[x\]\[y\]\=1dist(i,j,x,y)B\[i\]\[j\]\=min1≤x≤N,1≤y≤M,A\[x\]\[y\]\=1⁡dist(i,j,x,y)

#### 输入格式

第一行两个整数 N,MN,M。

接下来一个 NN 行 MM 列的 0101 矩阵，数字之间没有空格。

#### 输出格式

一个 NN 行 MM 列的矩阵 BB，相邻两个整数之间用一个空格隔开。

#### 数据范围

1≤N,M≤10001≤N,M≤1000

#### 输入样例：

    3 4
    0001
    0011
    0110
    

#### 输出样例：

    3 2 1 0
    2 1 0 0
    1 0 0 1
    

难度：简单

时/空限制：1s / 64MB

总通过数：18886

总尝试数：29160

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3724&show_algorithm_tags=0)[小马智行面试题](https://www.acwing.com/problem/search/1/?search_content=%E5%B0%8F%E9%A9%AC%E6%99%BA%E8%A1%8C%E9%9D%A2%E8%AF%95%E9%A2%98&source_file_id=3724&show_algorithm_tags=0)

算法标签

[BFS](https://www.acwing.com/problem/search/1/?search_content=BFS&source_file_id=3724&show_algorithm_tags=1)[多源BFS](https://www.acwing.com/problem/search/1/?search_content=%E5%A4%9A%E6%BA%90BFS&source_file_id=3724&show_algorithm_tags=1)