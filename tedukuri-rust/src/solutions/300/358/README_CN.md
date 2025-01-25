358\. 岛屿

*    [题目](https://www.acwing.com/problem/content/description/360/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/360/1/)
*    [题解](https://www.acwing.com/problem/content/solution/360/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/360/)

  

你准备游览一个公园，该公园由 NN 个岛屿组成，当地管理部门从每个岛屿出发向另外一个岛屿建了一座桥，不过桥是可以双向行走的。

同时，每对岛屿之间都有一艘专用的往来两岛之间的渡船。

相对于乘船而言，你更喜欢步行。

你希望所经过的桥的总长度尽可能的长，但受到以下的限制：

1.  可以自行挑选一个岛开始游览。
2.  任何一个岛都不能游览一次以上。
3.  无论任何时间你都可以由你现在所在的岛 SS 去另一个你从未到过的岛 DD。由 SS 到 DD 可以有以下方法：  
    （1）步行：仅当两个岛之间有一座桥时才有可能。对于这种情况，桥的长度会累加到你步行的总距离中。  
    （2）渡船：你可以选择这种方法，仅当没有任何桥和以前使用过的渡船的组合可以由 SS 走到 DD（当检查是否可到达时，你应该考虑所有的路径，包括经过你曾游览过的那些岛）。

注意，你不必游览所有的岛，也可能无法走完所有的桥。

请你编写一个程序，给定 NN 座桥以及它们的长度，按照上述的规则，计算你可以走过的桥的最大长度。

#### 输入格式

第 11 行包含整数 NN。

第 2..N+12..N+1 行，每行包含两个整数 aa 和 LL，第 i+1i+1 行表示岛屿 ii 上建了一座通向岛屿 aa 的桥，桥的长度为 LL。

#### 输出格式

输出一个整数，表示结果。

对某些测试，答案可能无法放进 32−bit32−bit 整数。

#### 数据范围

2≤N≤1062≤N≤106,  
1≤L≤1081≤L≤108

#### 输入样例：

    7
    3 8
    7 2
    4 2
    1 4
    1 9
    3 4
    2 3
    

#### 输出样例：

    24
    

难度：中等

时/空限制：2s / 256MB

总通过数：1837

总尝试数：6703

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3909&show_algorithm_tags=0)

算法标签

[DP](https://www.acwing.com/problem/search/1/?search_content=DP&source_file_id=3909&show_algorithm_tags=1)[基环树DP](https://www.acwing.com/problem/search/1/?search_content=%E5%9F%BA%E7%8E%AF%E6%A0%91DP&source_file_id=3909&show_algorithm_tags=1)[单调队列](https://www.acwing.com/problem/search/1/?search_content=%E5%8D%95%E8%B0%83%E9%98%9F%E5%88%97&source_file_id=3909&show_algorithm_tags=1)