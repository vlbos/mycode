192\. 立体推箱子2

*    [题目](https://www.acwing.com/problem/content/description/194/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/194/1/)
*    [题解](https://www.acwing.com/problem/content/solution/194/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/194/)

  

达达发明了一种立体推箱子游戏。

他发明的游戏里并没有那么多的规则和限制，在他的设定里游戏具有无限的平面空间，并且所有的区域都属于硬地。（关于立体推箱子游戏的各种概念和设定请参考 172172 题）

终点永远都位于坐标 (0,0)(0,0) 处的情况下，请你求出从起点到终点所需的最少移动次数是多少。

#### 输入格式

输入包含多组测试用例。

每组测试数据在一行内，格式为 `C x y`，其中 CC 为一个字母，xx 和 yy 是两个整数。

这表示长方体覆盖住了平台上的格子 (x,y)(x,y)，且其状态为 CC。

若 CC 为字母 UU，表明长方体是竖立的。

若 CC 为字母 VV，表明长方体与 xx 轴平行，且其覆盖的另一个格子为 (x+1,y)(x+1,y)。

若 CC 为字母 HH，表明长方体与 yy 轴平行，且其覆盖的另一个格子为 (x,y+1)(x,y+1)。

#### 输出格式

对于每个测试用例，输出一个占一行的整数，表示所需的最少移动次数。

#### 数据范围

0≤x,y≤10000000000≤x,y≤1000000000

#### 输入样例：

    U 0 0
    H 0 0
    V 1 0
    

#### 输出样例：

    0
    4
    1
    

难度：困难

时/空限制：1s / 64MB

总通过数：425

总尝试数：929

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3743&show_algorithm_tags=0)

算法标签

[搜索](https://www.acwing.com/problem/search/1/?search_content=%E6%90%9C%E7%B4%A2&source_file_id=3743&show_algorithm_tags=1)[广度优先搜索](https://www.acwing.com/problem/search/1/?search_content=%E5%B9%BF%E5%BA%A6%E4%BC%98%E5%85%88%E6%90%9C%E7%B4%A2&source_file_id=3743&show_algorithm_tags=1)[BFS](https://www.acwing.com/problem/search/1/?search_content=BFS&source_file_id=3743&show_algorithm_tags=1)[数学计算](https://www.acwing.com/problem/search/1/?search_content=%E6%95%B0%E5%AD%A6%E8%AE%A1%E7%AE%97&source_file_id=3743&show_algorithm_tags=1)[分情况讨论](https://www.acwing.com/problem/search/1/?search_content=%E5%88%86%E6%83%85%E5%86%B5%E8%AE%A8%E8%AE%BA&source_file_id=3743&show_algorithm_tags=1)