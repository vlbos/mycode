116\. 飞行员兄弟

*    [题目](https://www.acwing.com/problem/content/description/118/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/118/1/)
*    [题解](https://www.acwing.com/problem/content/solution/118/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/118/)

  

“飞行员兄弟”这个游戏，需要玩家顺利的打开一个拥有 1616 个把手的冰箱。

已知每个把手可以处于以下两种状态之一：打开或关闭。

只有当所有把手都打开时，冰箱才会打开。

把手可以表示为一个 4×44×4 的矩阵，您可以改变任何一个位置 \[i,j\]\[i,j\] 上把手的状态。

但是，这也会使得第 ii 行和第 jj 列上的所有把手的状态也随着改变。

请你求出打开冰箱所需的切换把手的次数最小值是多少。

#### 输入格式

输入一共包含四行，每行包含四个把手的初始状态。

符号 `+` 表示把手处于闭合状态，而符号 `-` 表示把手处于打开状态。

至少一个手柄的初始状态是关闭的。

#### 输出格式

第一行输出一个整数 NN，表示所需的最小切换把手次数。

接下来 NN 行描述切换顺序，每行输出两个整数，代表被切换状态的把手的行号和列号，数字之间用空格隔开。

**注意**：如果存在多种打开冰箱的方式，则按照优先级整体从上到下，同行从左到右打开。

#### 数据范围

1≤i,j≤41≤i,j≤4

#### 输入样例：

    -+--
    ----
    ----
    -+--
    

#### 输出样例：

    6
    1 1
    1 3
    1 4
    4 1
    4 3
    4 4
    

难度：简单

时/空限制：1s / 64MB

总通过数：23925

总尝试数：32924

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3667&show_algorithm_tags=0)

算法标签

[枚举](https://www.acwing.com/problem/search/1/?search_content=%E6%9E%9A%E4%B8%BE&source_file_id=3667&show_algorithm_tags=1)[位运算](https://www.acwing.com/problem/search/1/?search_content=%E4%BD%8D%E8%BF%90%E7%AE%97&source_file_id=3667&show_algorithm_tags=1)