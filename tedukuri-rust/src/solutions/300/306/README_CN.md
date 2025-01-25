306\. 杰拉尔德和巨型象棋

*    [题目](https://www.acwing.com/problem/content/description/308/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/308/1/)
*    [题解](https://www.acwing.com/problem/content/solution/308/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/308/)

  

给定一个 H×WH×W 的棋盘，棋盘上只有 NN 个格子是黑色的，其他格子都是白色的。

在棋盘左上角有一个卒，每一步可以向右或向下移动一格，并且不能移动到黑色格子中。

求这个卒从左上角移动到右下角，一共有多少种路线。

#### 输入格式

第一行包含三个整数 H,W,NH,W,N。

接下来 NN 行，每行包含两个整数 x，yx，y，描述一个黑色格子位于 xx 行 yy 列。

数据保证左上角和右下角的格子都是白色的。

#### 输出格式

输出一个整数表示结果对 109+7109+7 取模后的值。

#### 数据范围

1≤H,W≤1051≤H,W≤105,  
1≤N≤20001≤N≤2000

#### 输入样例1：

    3 4 2 
    2 2 
    2 3
    

#### 输出样例1：

    2
    

#### 输入样例2：

    100 100 3
    15 16
    16 15
    99 88
    

#### 输出样例2：

    545732279
    

难度：中等

时/空限制：1s / 64MB

总通过数：1108

总尝试数：2626

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3857&show_algorithm_tags=0)[CF559C](https://www.acwing.com/problem/search/1/?search_content=CF559C&source_file_id=3857&show_algorithm_tags=0)

算法标签

[动态规划](https://www.acwing.com/problem/search/1/?search_content=%E5%8A%A8%E6%80%81%E8%A7%84%E5%88%92&source_file_id=3857&show_algorithm_tags=1)[计数类DP](https://www.acwing.com/problem/search/1/?search_content=%E8%AE%A1%E6%95%B0%E7%B1%BBDP&source_file_id=3857&show_algorithm_tags=1)