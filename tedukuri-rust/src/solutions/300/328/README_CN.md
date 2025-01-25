328\. 芯片

*    [题目](https://www.acwing.com/problem/content/description/330/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/330/1/)
*    [题解](https://www.acwing.com/problem/content/solution/330/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/330/)

  

Bugs Integrated，Inc 是高级存储芯片的主要制造商。

他们正在生产一种新的 6TB Q-RAM 芯片。

每个芯片由六个单位方块组成，以 2×32×3 矩形的形式排列。

该公司通过分割 N×MN×M 个单位方块组成的矩形硅片得到多个 Q-RAM 芯片。

大的矩形硅片会被完整检测，并且其中坏掉的方块会用黑色标明。

![bugs.gif](https://cdn.acwing.com/media/article/image/2019/02/08/19_5f4a325a2b-bugs.gif)

矩形硅片要被分割成尽可能多的芯片，并且每个芯片中都不能包含坏掉的方块，请你求出最优解法。

#### 输入格式

第一行包含整数 DD，表示共有 DD 组测试数据。

每组测试数据第一行包含三个整数 N,MN,M 和 KK，其中 KK 为坏掉的方块数量。

接下来 KK 行，每行包含两个整数 x，yx，y，表示一个坏掉的方块的位置坐标 (x,y)(x,y)。（矩形硅片的左上角坐标为 (1,1)(1,1),右下角坐标为 (N,M)(N,M)）

#### 输出格式

每组测试用例输出一个整数，表示能分割出的最大芯片数量。

每个结果占一行。

#### 数据范围

1≤D≤51≤D≤5，  
1≤N≤1501≤N≤150,  
1≤M≤101≤M≤10,  
1≤K≤NM1≤K≤NM,

#### 输入样例：

    2
    6 6 5
    1 4
    4 6
    2 2
    3 6
    6 4
    6 5 4
    3 3
    6 1
    6 2
    6 4
    

#### 输出样例：

    3
    4
    

难度：简单

时/空限制：1s / 30MB

总通过数：504

总尝试数：1100

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3879&show_algorithm_tags=0)

算法标签

[动态规划](https://www.acwing.com/problem/search/1/?search_content=%E5%8A%A8%E6%80%81%E8%A7%84%E5%88%92&source_file_id=3879&show_algorithm_tags=1)[状态压缩DP](https://www.acwing.com/problem/search/1/?search_content=%E7%8A%B6%E6%80%81%E5%8E%8B%E7%BC%A9DP&source_file_id=3879&show_algorithm_tags=1)